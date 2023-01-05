#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttestationMetadata {
    #[prost(oneof="attestation_metadata::AttestationMetadata", tags="1")]
    pub attestation_metadata: ::core::option::Option<attestation_metadata::AttestationMetadata>,
}
/// Nested message and enum types in `AttestationMetadata`.
pub mod attestation_metadata {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Tpm {
        #[prost(message, optional, tag="1")]
        pub ak_attributes: ::core::option::Option<super::super::psa_key_attributes::KeyAttributes>,
        #[prost(bytes="vec", tag="2")]
        pub ak_public: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint32, repeated, tag="3")]
        pub pcr_list: ::prost::alloc::vec::Vec<u32>,
        #[prost(enumeration="super::super::psa_algorithm::algorithm::Hash", tag="4")]
        pub pcr_hash: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AttestationMetadata {
        #[prost(message, tag="1")]
        Tpm(Tpm),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Result {
    #[prost(message, repeated, tag="1")]
    pub attestation_metadata: ::prost::alloc::vec::Vec<AttestationMetadata>,
}
