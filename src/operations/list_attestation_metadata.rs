// Copyright 2022 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
//! # ListRoTs operation
//!
//! List the Roots of Trust available within the service,
//! and their corresponding configurations.
use super::psa_algorithm::Hash;
use super::psa_key_attributes::Attributes;
use crate::requests::ProviderId;
use derivative::Derivative;
use zeroize::Zeroizing;

/// Native operation for listing RoTs
#[derive(Debug, Copy, Clone)]
pub struct Operation {}

/// Provider-specific attestation metadata
#[derive(Derivative)]
#[derivative(Debug)]
#[non_exhaustive]
pub enum AttestationMetadata {
    /// Configuration of TPM2 for attestation
    Tpm {
        /// Attributes of attesting key
        ak_attributes: Attributes,
        /// Public part of attesting key
        #[derivative(Debug = "ignore")]
        ak_public: Zeroizing<Vec<u8>>,
        /// List of PCRs which will be quoted
        pcr_list: Vec<u8>,
        /// Hash algorithm used to extend the PCRs
        pcr_hash: Hash,
    },
}

impl AttestationMetadata {
    /// Get the provider ID corresponding to the given configuration
    pub fn provider_id(&self) -> ProviderId {
        match self {
            AttestationMetadata::Tpm { .. } => ProviderId::Tpm,
        }
    }
}

/// Native result of listing RoTs
#[derive(Debug)]
pub struct Result {
    /// List of supported RoTs and their configuration
    pub attestation_metadata: Vec<AttestationMetadata>,
}
