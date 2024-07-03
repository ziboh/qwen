pub mod config;
/// The module contains the qwen client
pub mod client;

pub mod err;

pub mod types;

pub mod converse;

pub type Result<T> = std::result::Result<T,err::Error>;
