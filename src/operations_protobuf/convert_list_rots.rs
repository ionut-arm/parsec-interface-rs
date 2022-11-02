// Copyright 2022 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use super::convert_psa_algorithm::{hash_to_i32, i32_to_hash};
use super::generated_ops::list_rots::{
    rot_info, Operation as OperationProto, Result as ResultProto, RotInfo,
};
use crate::operations::list_rots::{Mechanism, Operation, Result, RoTInfo};
use crate::requests::{ProviderId, ResponseStatus};
use num::FromPrimitive;
use std::convert::{TryFrom, TryInto};

impl TryFrom<Operation> for OperationProto {
    type Error = ResponseStatus;

    fn try_from(_op: Operation) -> std::result::Result<Self, Self::Error> {
        Ok(OperationProto {})
    }
}

impl TryFrom<OperationProto> for Operation {
    type Error = ResponseStatus;

    fn try_from(_proto_op: OperationProto) -> std::result::Result<Self, Self::Error> {
        Ok(Operation {})
    }
}

impl TryFrom<RoTInfo> for RotInfo {
    type Error = ResponseStatus;

    fn try_from(rot_info: RoTInfo) -> std::result::Result<Self, Self::Error> {
        match rot_info {
            RoTInfo {
                provider_id,
                mechanism:
                    Mechanism::CertifyAndQuote {
                        ak_attributes,
                        ak_public,
                        pcr_list,
                        pcr_hash,
                    },
            } => Ok(RotInfo {
                provider_id: provider_id as u32,
                mechanism: Some(rot_info::Mechanism::CertifyAndQuote(
                    rot_info::CertifyAndQuote {
                        ak_attributes: Some(ak_attributes.try_into()?),
                        ak_public: ak_public.to_vec(),
                        pcr_list: pcr_list.iter().map(|pcr| *pcr as u32).collect(),
                        pcr_hash: hash_to_i32(pcr_hash),
                    },
                )),
            }),
        }
    }
}

impl TryFrom<RotInfo> for RoTInfo {
    type Error = ResponseStatus;

    fn try_from(rot_info: RotInfo) -> std::result::Result<Self, Self::Error> {
        match rot_info {
            RotInfo {
                mechanism:
                    Some(rot_info::Mechanism::CertifyAndQuote(rot_info::CertifyAndQuote {
                        ak_attributes: Some(ak_attributes),
                        ak_public,
                        pcr_list,
                        pcr_hash,
                    })),
                provider_id,
            } => {
                if pcr_list.iter().any(|pcr| *pcr > 255) {
                    // TODO: new response status?
                    return Err(ResponseStatus::DeserializingBodyFailed);
                }
                let provider_id: ProviderId = match FromPrimitive::from_u32(provider_id) {
                    Some(id) => id,
                    None => return Err(ResponseStatus::ProviderDoesNotExist),
                };
                Ok(RoTInfo {
                    provider_id,
                    mechanism: Mechanism::CertifyAndQuote {
                        ak_attributes: ak_attributes.try_into()?,
                        ak_public: ak_public.into(),
                        pcr_list: pcr_list.iter().map(|pcr| *pcr as u8).collect(),
                        pcr_hash: i32_to_hash(pcr_hash)?,
                    },
                })
            }
            // TODO: new response status?
            _ => Err(ResponseStatus::DeserializingBodyFailed),
        }
    }
}

impl TryFrom<Result> for ResultProto {
    type Error = ResponseStatus;

    fn try_from(op: Result) -> std::result::Result<Self, Self::Error> {
        let mut rots: Vec<RotInfo> = Vec::new();
        for rot in op.rots {
            rots.push(rot.try_into()?);
        }

        Ok(ResultProto { rots })
    }
}

impl TryFrom<ResultProto> for Result {
    type Error = ResponseStatus;

    fn try_from(proto_op: ResultProto) -> std::result::Result<Self, Self::Error> {
        let mut rots: Vec<RoTInfo> = Vec::new();
        for rot in proto_op.rots {
            rots.push(rot.try_into()?);
        }

        Ok(Result { rots })
    }
}

#[cfg(test)]
mod test {
    // TODO
}
