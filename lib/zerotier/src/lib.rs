use crate::r#trait::NetworkCentral;

pub mod api;
mod format;
pub mod model;
pub mod r#trait;

pub type Result<T> = anyhow::Result<T>;
pub type NetworkResult = crate::model::network::NetworkResult;
pub type NetworkConfig = crate::model::network::NetworkConfig;
pub type NetworkUpdatePayload = crate::model::network::NetworkUpdatePayload;
pub type NetworkUpdatePayloadConfig = crate::model::network::NetworkUpdatePayloadConfig;

pub type NetworkMemberResult = crate::model::network_member::NetworkMemberResult;
pub type NetworkMemberConfig = crate::model::network_member::NetworkMemberConfig;
pub type NetworkMemberUpdatePayload = crate::model::network_member::NetworkMemberUpdatePayload;
pub type NetworkMemberUpdatePayloadConfig =
    crate::model::network_member::NetworkMemberUpdatePayloadConfig;

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
