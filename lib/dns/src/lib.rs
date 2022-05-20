pub mod buffer;
pub mod header;
pub mod packet;
pub mod query_type;
pub mod question;
pub mod record;
pub mod result;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
