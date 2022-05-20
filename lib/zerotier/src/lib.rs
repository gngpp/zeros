pub mod api;
pub mod model;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Network {
    pub id: String,
    pub remark: Option<String>
}

impl Network {
    
    pub fn new(id: String, remark: Option<String>) -> Self {
        Self { 
            id, 
            remark 
        }
    }
}

#[warn(dead_code)]
pub struct Zerotier {
    token: String,
    network_list: Vec<Network>,
    client: reqwest::Client
}

impl Zerotier {

    pub fn new(token: String, network_id_array: Vec<Network>) -> Zerotier {
        Zerotier{
            token,
            network_list: network_id_array,
            client: reqwest::Client::new()
        }
    }

    pub fn push_network_id(&mut self, network_id: Network) {
        self.network_list.push(network_id)
    }

    pub fn get_network_id_array(self) -> Vec<Network> {
        self.network_list
    }

    
    pub fn get_member_result(self, network_id: &String) -> crate::Result<Vec<model::member::MemberResult>> {
        let member_api = crate::api::format_zerotier_member_api(network_id);
        let bearer_token = crate::api::format_bearer_token(&self.token);
        let result = self.client.get(&member_api)
            .header( reqwest::header::AUTHORIZATION, bearer_token)
            .send()?
            .json::<Vec<crate::model::member::MemberResult>>()?;
        Ok(result)
    }

}