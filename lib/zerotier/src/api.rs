use crate::r#trait::NetworkMemberCentral;
use crate::{NetworkCentral, NetworkMemberResult, NetworkResult, Result};
use anyhow::{anyhow, Ok};
use reqwest::Response;
use std::ops::Deref;
use std::thread::sleep;

#[warn(dead_code)]
pub struct ZtNetworkCentral {
    token: String,
    client: RequestClient,
}

impl ZtNetworkCentral {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: RequestClient::new(),
        }
    }
}

impl NetworkCentral for ZtNetworkCentral {
    fn find_network_list(&self) -> Result<Vec<NetworkResult>> {
        let url = "https://my.zerotier.com/api/v1/network";
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let resp = self.client.get(url.to_string(), bearer_token)?;
        ResponseHandler::response_handler::<Vec<NetworkResult>>(resp)
    }

    fn create_network(&self) -> Result<NetworkResult> {
        let url = "https://my.zerotier.com/api/v1/network";
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let resp = self.client.post(url.to_string(), bearer_token, std::collections::HashMap::from([("", "")]))?;
        ResponseHandler::response_handler::<NetworkResult>(resp)
    }

    fn find_network_by_id(&self, network_id: &String) -> Result<NetworkResult> {
        let url = format!("https://my.zerotier.com/api/v1/network/{}", network_id);
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let resp = self.client.get(url, bearer_token)?;
        ResponseHandler::response_handler::<NetworkResult>(resp)
    }

    fn update_network_config(
        &self,
        network_id: &String,
        payload: crate::NetworkUpdatePayload,
    ) -> Result<NetworkResult> {
        let url = format!("https://my.zerotier.com/api/v1/network/{}", network_id);
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let resp = self.client.post(url, bearer_token, payload)?;
        ResponseHandler::response_handler::<NetworkResult>(resp)
    }

    fn delete_network(&self, network_id: &String) -> Result<()> {
        let url = format!("https://my.zerotier.com/api/v1/network/{}", network_id);
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let resp = self.client.delete(url, bearer_token)?;
        ResponseHandler::response_unit_handler(resp)
    }
}

pub struct ZtNetworkMemberCentral {
    token: String,
    client: RequestClient,
}

impl ZtNetworkMemberCentral {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: RequestClient::new(),
        }
    }
}

impl NetworkMemberCentral for ZtNetworkMemberCentral {
    fn find_network_member_list(&self, network_id: &String) -> Result<Vec<NetworkMemberResult>> {
        let url = format!("https://my.zerotier.com/api/network/{}/member", network_id);
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let resp = self.client.get(url, bearer_token)?;
        ResponseHandler::response_handler::<Vec<NetworkMemberResult>>(resp)
    }

    fn find_network_member(
        &self,
        network_id: &String,
        member_id: &String,
    ) -> Result<NetworkMemberResult> {
        let url = format!(
            "https://my.zerotier.com/api/v1/network/{}/member/{}",
            network_id, member_id
        );
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let resp = self.client.get(url, bearer_token)?;
        ResponseHandler::response_handler::<NetworkMemberResult>(resp)
    }

    fn update_network_member(
        &self,
        network_id: &String,
        member_id: &String,
        payload: crate::NetworkMemberUpdatePayload,
    ) -> Result<NetworkMemberResult> {
        let url = format!(
            "https://my.zerotier.com/api/v1/network/{}/member/{}",
            network_id, member_id
        );
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let resp = self.client.post(url, bearer_token, payload)?;
        ResponseHandler::response_handler::<NetworkMemberResult>(resp)
    }

    fn delete_network_member(&self, network_id: &String, member_id: &String) -> Result<()> {
        let url = format!(
            "https://my.zerotier.com/api/v1/network/{}/member/{}",
            network_id, member_id
        );
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let resp = self.client.delete(url, bearer_token)?;
        ResponseHandler::response_unit_handler(resp)
    }
}

struct RequestClient {
    client: reqwest::Client,
}

impl RequestClient {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    fn get(&self, url: String, token: String) -> Result<Response> {
        let resp = self
            .client
            .get(url.deref())
            .header(reqwest::header::AUTHORIZATION, token)
            .send()?;
        Ok(resp)
    }

    fn post<T: serde::Serialize>(&self, url: String, token: String, body: T) -> Result<Response> {
        let resp = self
            .client
            .post(url.deref())
            .header(reqwest::header::AUTHORIZATION, token)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&body)
            .send()?;
        Ok(resp)
    }

    fn delete(&self, url: String, token: String) -> Result<Response> {
        let resp = self
            .client
            .delete(url.deref())
            .header(reqwest::header::AUTHORIZATION, token)
            .send()?;
        Ok(resp)
    }
}

struct ResponseHandler;

impl ResponseHandler {
    fn response_unit_handler(mut resp: Response) -> Result<()> {
        if let Some(x) = ResponseHandler::response_error_handler(&mut resp) {
            return Err(anyhow!(x));
        }
        Ok(())
    }

    fn response_handler<T: serde::de::DeserializeOwned>(mut resp: Response) -> Result<T> {
        if let Some(x) = ResponseHandler::response_error_handler(&mut resp) {
            return Err(anyhow!(x));
        }
        let result = resp.json::<T>()?;
        Ok(result)
    }

    fn response_error_handler(resp: &mut Response) -> Option<String> {
        if resp.status().is_success() {
            return None;
        }
        let msg = resp
            .text()
            .unwrap_or(String::from("An error occurred while extracting the body."));
        log::debug!(
            "defined in file: {}, defined on line: {}\nmessage: {:?}",
            file!(),
            line!(),
            &msg
        );
        Some(msg)
    }
}
