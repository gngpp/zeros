use crate::r#trait::NetworkCentral;

pub mod api;
mod format;
pub mod model;
pub mod r#trait;

pub type Result<T> = anyhow::Result<T>;
pub type NetworkResult = model::network::NetworkResult;
pub type NetworkConfig = model::network::NetworkConfig;
pub type NetworkUpdatePayload = model::network::NetworkUpdatePayload;
pub type NetworkUpdatePayloadConfig = model::network::NetworkUpdatePayloadConfig;

pub type NetworkMemberResult = model::network_member::NetworkMemberResult;
pub type NetworkMemberConfig = model::network_member::NetworkMemberConfig;
pub type NetworkMemberUpdatePayload = model::network_member::NetworkMemberUpdatePayload;
pub type NetworkMemberUpdatePayloadConfig =
    model::network_member::NetworkMemberUpdatePayloadConfig;

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
