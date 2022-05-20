use crate::r#trait::NetworkCentral;

pub mod api;
mod format;
pub mod model;
pub mod r#trait;

pub type Result<T> = anyhow::Result<T>;
pub type NetworkResult = crate::model::network::NetworkResult;
pub type NetworkConfig = crate::model::network::NetworkConfig;
pub type NetworkMemberResult = crate::model::network_member::NetworkMemberResult;
pub type NetworkMemberConfig = crate::model::network_member::NetworkMemberConfig;

#[derive(Debug)]
pub struct Network {
    pub id: String,
    pub remark: Option<String>,
}

impl Network {
    pub fn new(id: String, remark: Option<String>) -> Self {
        Self { id, remark }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_member_api() -> super::Result<()> {
        Ok(())
    }

    #[test]
    pub fn test_network_api() -> super::Result<()> {
        Ok(())
    }
}
