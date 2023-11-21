#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MakeFile {
    #[prost(string, required, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub output_binary: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub dependencies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Makefiles {
    #[prost(message, repeated, tag = "1")]
    pub makefiles: ::prost::alloc::vec::Vec<Makefiles>,
}
