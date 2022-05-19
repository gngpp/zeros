pub mod buffer;
pub mod result;
pub mod header;
pub mod query_type;
pub mod packet;
pub mod question;
pub mod record;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;