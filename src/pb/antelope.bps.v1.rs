// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pays {
    #[prost(message, repeated, tag="1")]
    pub vpays: ::prost::alloc::vec::Vec<Pay>,
    #[prost(message, repeated, tag="2")]
    pub bpays: ::prost::alloc::vec::Vec<Pay>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pay {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    #[prost(string, tag="7")]
    pub bp: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub quantity: ::prost::alloc::string::String,
    /// extras
    #[prost(int64, tag="11")]
    pub amount: i64,
    #[prost(double, tag="12")]
    pub value: f64,
    /// block information
    #[prost(uint64, tag="13")]
    pub block_num: u64,
    #[prost(message, optional, tag="14")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
// @@protoc_insertion_point(module)
