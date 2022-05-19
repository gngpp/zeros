pub mod api;
pub mod model;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Zerotier {
    token: String,
    network_id_array: Vec<String>,
    client: reqwest::Client
}

impl Zerotier {

    pub fn new(token: String, network_id_array: Vec<String>) -> Zerotier {
        Zerotier{
            token,
            network_id_array,
            client: reqwest::Client::new()
        }
    }

    pub fn push_network_id(&mut self, network_id: String) {
        self.network_id_array.push(network_id)
    }

    pub fn get_network_id_array(self) -> Vec<String> {
        self.network_id_array
    }

    pub fn get_member_result(self, network_id: String) -> crate::Result<Vec<model::member::MemberResult>> {
        let member_api = crate::api::format_zerotier_member_api(network_id);
        let bearer_token = crate::api::format_bearer_token(self.token);
        let result = self.client.get(&member_api)
            .header( reqwest::header::AUTHORIZATION, bearer_token)
            .send()?
            .json::<Vec<crate::model::member::MemberResult>>()?;
        Ok(result)
    }



}