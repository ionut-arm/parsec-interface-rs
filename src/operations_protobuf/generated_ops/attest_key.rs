#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    #[prost(string, tag="1")]
    pub attested_key_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub attesting_key_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Result {
    #[prost(bytes="vec", tag="1")]
    pub attestation_token: ::prost::alloc::vec::Vec<u8>,
}
