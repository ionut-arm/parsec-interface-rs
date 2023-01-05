// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
//! # AttestKey operation
//!
//! Produce an attestation token as proof that the given
//! key was produced and is stored in the hardware backend.
use derivative::Derivative;
use zeroize::Zeroizing;

/// Native operation for key attestation
#[derive(Derivative)]
#[derivative(Debug)]
#[non_exhaustive]
pub struct Operation {
    /// Name of key to be attested
    pub attested_key_name: String,
    /// The nonce to be used in the TLS handshake
    #[derivative(Debug = "ignore")]
    pub nonce: Vec<u8>,
    /// Name of key to be used for attesting
    pub attesting_key_name: Option<String>,
}

/// Native result of key attestation
#[derive(Derivative)]
#[derivative(Debug)]
#[non_exhaustive]
pub struct Result {
    /// Token that resulted from the attestation operation
    pub attestation_token: Zeroizing<Vec<u8>>,
}
