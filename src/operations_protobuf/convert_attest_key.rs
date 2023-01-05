// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use super::generated_ops::attest_key::{Operation as OperationProto, Result as ResultProto};
use crate::operations::attest_key::{Operation, Result};
use crate::requests::ResponseStatus;
use std::convert::TryFrom;

impl TryFrom<Operation> for OperationProto {
    type Error = ResponseStatus;

    fn try_from(op: Operation) -> std::result::Result<Self, Self::Error> {
        let Operation {
            attested_key_name,
            nonce,
            attesting_key_name,
        } = op;
        Ok(OperationProto {
            attested_key_name,
            attesting_key_name: attesting_key_name.unwrap_or_default(),
            nonce,
        })
    }
}

impl TryFrom<OperationProto> for Operation {
    type Error = ResponseStatus;

    fn try_from(proto_op: OperationProto) -> std::result::Result<Self, Self::Error> {
        let OperationProto {
            attesting_key_name,
            attested_key_name,
            nonce,
        } = proto_op;
        Ok(Operation {
            attested_key_name,
            attesting_key_name: if attesting_key_name.is_empty() {
                None
            } else {
                Some(attesting_key_name)
            },
            nonce,
        })
    }
}

impl TryFrom<Result> for ResultProto {
    type Error = ResponseStatus;

    fn try_from(op: Result) -> std::result::Result<Self, Self::Error> {
        let Result { attestation_token } = op;
        Ok(ResultProto {
            attestation_token: attestation_token.to_vec(),
        })
    }
}

impl TryFrom<ResultProto> for Result {
    type Error = ResponseStatus;

    fn try_from(proto_op: ResultProto) -> std::result::Result<Self, Self::Error> {
        let ResultProto { attestation_token } = proto_op;
        Ok(Result {
            attestation_token: attestation_token.into(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::super::generated_ops::attest_key::{
        Operation as OperationProto, Result as ResultProto,
    };
    use super::super::{Convert, ProtobufConverter};
    use crate::operations::{
        attest_key::Operation, attest_key::Result, NativeOperation, NativeResult,
    };
    use crate::requests::Opcode;
    use std::convert::TryInto;

    static CONVERTER: ProtobufConverter = ProtobufConverter {};

    #[test]
    fn attest_key_op_from_proto() {
        let op_attested_key_name = String::from("attested key name");
        let op_attesting_key_name = String::from("attesting key name");
        let op_nonce = vec![0x88; 8];

        let proto = OperationProto {
            attested_key_name: op_attested_key_name.clone(),
            attesting_key_name: op_attesting_key_name.clone(),
            nonce: op_nonce.clone(),
        };

        let op: Operation = proto.try_into().expect("Failed conversion");
        let Operation {
            attested_key_name,
            attesting_key_name,
            nonce,
        } = op;
        assert_eq!(attested_key_name, op_attested_key_name);
        assert_eq!(
            attesting_key_name.unwrap_or_default(),
            op_attesting_key_name
        );
        assert_eq!(nonce.to_vec(), op_nonce);
    }

    #[test]
    fn attest_key_proto_no_attesting() {
        let op_attested_key_name = String::from("attested key name");
        let op_nonce = vec![0x88; 8];

        let proto = OperationProto {
            attested_key_name: op_attested_key_name,
            attesting_key_name: String::new(),
            nonce: op_nonce,
        };

        let op: Operation = proto.try_into().expect("Failed conversion");
        let Operation {
            attesting_key_name, ..
        } = op;
        assert!(attesting_key_name.is_none());
    }

    #[test]
    fn attest_key_proto_from_op() {
        let op_attested_key_name = String::from("attested key name");
        let op_attesting_key_name = String::from("attesting key name");
        let op_nonce = vec![0x88; 8];

        let op = Operation {
            attested_key_name: op_attested_key_name.clone(),
            attesting_key_name: Some(op_attesting_key_name.clone()),
            nonce: op_nonce.clone(),
        };

        let proto: OperationProto = op.try_into().expect("Failed conversion");
        let OperationProto {
            attested_key_name,
            attesting_key_name,
            nonce,
        } = proto;
        assert_eq!(attested_key_name, op_attested_key_name);
        assert_eq!(attesting_key_name, op_attesting_key_name);
        assert_eq!(nonce, op_nonce);
    }

    #[test]
    fn attest_key_op_no_attesting() {
        let op_attested_key_name = String::from("attested key name");
        let op_nonce = vec![0x88; 8];

        let op = Operation {
            attested_key_name: op_attested_key_name,
            attesting_key_name: None,
            nonce: op_nonce,
        };

        let proto: OperationProto = op.try_into().expect("Failed conversion");
        assert_eq!(proto.attesting_key_name, String::new());
    }

    #[test]
    fn attest_key_op_e2e() {
        let op_attested_key_name = String::from("attested key name");
        let op_attesting_key_name = String::from("attesting key name");
        let op_nonce = vec![0x88; 8];

        let op = Operation {
            attested_key_name: op_attested_key_name,
            attesting_key_name: Some(op_attesting_key_name),
            nonce: op_nonce,
        };

        let body = CONVERTER
            .operation_to_body(NativeOperation::AttestKey(op))
            .expect("Failed to convert to body");

        let _ = CONVERTER
            .body_to_operation(body, Opcode::AttestKey)
            .expect("Failed to convert to operation");
    }

    #[test]
    fn attest_key_resp_from_proto() {
        let resp_attestation_token = vec![0x99; 64];

        let proto = ResultProto {
            attestation_token: resp_attestation_token.clone(),
        };

        let resp: Result = proto.try_into().expect("Failed conversion");

        let Result { attestation_token } = resp;
        assert_eq!(attestation_token.to_vec(), resp_attestation_token);
    }

    #[test]
    fn attest_key_resp_to_proto() {
        let resp_attestation_token = vec![0x99; 64];

        let resp = Result {
            attestation_token: resp_attestation_token.clone().into(),
        };

        let proto: ResultProto = resp.try_into().expect("Failed conversion");

        let ResultProto { attestation_token } = proto;
        assert_eq!(attestation_token.to_vec(), resp_attestation_token);
    }

    #[test]
    fn attest_key_resp_e2e() {
        let resp_attestation_token = vec![0x99; 64];

        let resp = Result {
            attestation_token: resp_attestation_token.into(),
        };

        let body = CONVERTER
            .result_to_body(NativeResult::AttestKey(resp))
            .expect("Failed to convert to body");

        let _ = CONVERTER
            .body_to_result(body, Opcode::AttestKey)
            .expect("Failed to convert to operation");
    }
}
