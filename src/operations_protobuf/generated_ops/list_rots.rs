#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RotInfo {
    #[prost(uint32, tag="1")]
    pub provider_id: u32,
    #[prost(oneof="rot_info::Mechanism", tags="2")]
    pub mechanism: ::core::option::Option<rot_info::Mechanism>,
}
/// Nested message and enum types in `RotInfo`.
pub mod rot_info {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CertifyAndQuote {
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
    pub enum Mechanism {
        #[prost(message, tag="2")]
        CertifyAndQuote(CertifyAndQuote),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Result {
    #[prost(message, repeated, tag="1")]
    pub rots: ::prost::alloc::vec::Vec<RotInfo>,
}
