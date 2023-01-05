// Copyright 2022 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use super::convert_psa_algorithm::{hash_to_i32, i32_to_hash};
use super::generated_ops::list_attestation_metadata::{
    attestation_metadata, AttestationMetadata as AttestationMetadataProto,
    Operation as OperationProto, Result as ResultProto,
};
use crate::operations::list_attestation_metadata::{AttestationMetadata, Operation, Result};
use crate::requests::ResponseStatus;
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

impl TryFrom<AttestationMetadata> for AttestationMetadataProto {
    type Error = ResponseStatus;

    fn try_from(rot_info: AttestationMetadata) -> std::result::Result<Self, Self::Error> {
        match rot_info {
            AttestationMetadata::Tpm {
                ak_attributes,
                ak_public,
                pcr_list,
                pcr_hash,
            } => Ok(AttestationMetadataProto {
                attestation_metadata: Some(attestation_metadata::AttestationMetadata::Tpm(
                    attestation_metadata::Tpm {
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

impl TryFrom<AttestationMetadataProto> for AttestationMetadata {
    type Error = ResponseStatus;

    fn try_from(rot_info: AttestationMetadataProto) -> std::result::Result<Self, Self::Error> {
        match rot_info {
            AttestationMetadataProto {
                attestation_metadata:
                    Some(attestation_metadata::AttestationMetadata::Tpm(attestation_metadata::Tpm {
                        ak_attributes: Some(ak_attributes),
                        ak_public,
                        pcr_list,
                        pcr_hash,
                    })),
            } => {
                if pcr_list.iter().any(|pcr| *pcr > 255) {
                    // TODO: new response status?
                    return Err(ResponseStatus::DeserializingBodyFailed);
                }
                Ok(AttestationMetadata::Tpm {
                    ak_attributes: ak_attributes.try_into()?,
                    ak_public: ak_public.into(),
                    pcr_list: pcr_list.iter().map(|pcr| *pcr as u8).collect(),
                    pcr_hash: i32_to_hash(pcr_hash)?,
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
        let mut metadata_list: Vec<AttestationMetadataProto> = Vec::new();
        for attestation_metadata in op.attestation_metadata {
            metadata_list.push(attestation_metadata.try_into()?);
        }

        Ok(ResultProto {
            attestation_metadata: metadata_list,
        })
    }
}

impl TryFrom<ResultProto> for Result {
    type Error = ResponseStatus;

    fn try_from(proto_op: ResultProto) -> std::result::Result<Self, Self::Error> {
        let mut metadata_list: Vec<AttestationMetadata> = Vec::new();
        for rot in proto_op.attestation_metadata {
            metadata_list.push(rot.try_into()?);
        }

        Ok(Result {
            attestation_metadata: metadata_list,
        })
    }
}

#[cfg(test)]
mod test {
    // TODO
}
