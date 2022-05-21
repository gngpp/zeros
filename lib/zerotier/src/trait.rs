pub trait NetworkCentral {
    // Returns a list of Networks you have access to.
    fn find_network_list(&self) -> crate::Result<Vec<crate::NetworkResult>>;

    // Create a new network.
    fn create_network(&self) -> crate::Result<crate::NetworkResult>;

    // Get network by ID
    fn find_network_by_id(&self, network_id: &String) -> crate::Result<crate::NetworkResult>;

    // Update network configuration.
    fn update_network_config(
        &self,
        network_id: &String,
        config: crate::NetworkUpdatePayload,
    ) -> crate::Result<crate::NetworkResult>;

    // Delete network.
    fn delete_network(&self, network_id: &String) -> crate::Result<()>;
}

pub trait NetworkMemberCentral {
    // Returns a list of Members on the network.
    fn find_network_member_list(
        &self,
        network_id: &String,
    ) -> crate::Result<Vec<crate::NetworkMemberResult>>;

    // Return an individual member on a network.
    fn find_member(
        &self,
        network_id: &String,
        member_id: &String,
    ) -> crate::Result<crate::NetworkMemberResult>;

    // Modify a network member.
    fn update_member(
        &self,
        network_id: &String,
        member_id: &String,
        payload: crate::NetworkMemberUpdatePayload,
    ) -> crate::Result<crate::NetworkMemberResult>;

    // Delete a network member.
    fn delete_member(&self, network_id: &String, member_id: &String) -> crate::Result<()>;
}

