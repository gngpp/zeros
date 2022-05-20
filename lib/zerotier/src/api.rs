use crate::r#trait::NetworkMemberCentral;
use crate::{NetworkCentral, NetworkMemberResult, NetworkResult, Result};
use anyhow::{anyhow, Ok};
use std::ops::Deref;

#[warn(dead_code)]
pub struct ZtNetworkCentral {
    token: String,
    client: reqwest::Client,
}

impl ZtNetworkCentral {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: reqwest::Client::new(),
        }
    }

    fn response_arr_handler(&self, mut resp: reqwest::Response) -> Result<NetworkResult> {
        return if resp.status().is_success() {
            let result = resp.json::<NetworkResult>()?;
            Ok(result)
        } else {
            // fail status msg
            let msg = resp.text()?;
            Err(anyhow!(msg))
        };
    }
}

impl NetworkCentral for ZtNetworkCentral {
    fn find_network_list(&self) -> Result<Vec<NetworkResult>> {
        let url = "https://my.zerotier.com/api/v1/network";
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let mut resp = self
            .client
            .get(url)
            .header(reqwest::header::AUTHORIZATION, bearer_token)
            .send()?;
        return if resp.status().is_success() {
            let result = resp.json::<Vec<NetworkResult>>()?;
            Ok(result)
        } else {
            // fail status msg
            let msg = resp.text()?;
            Err(anyhow!(msg))
        };
    }

    fn create_network(&self) -> Result<NetworkResult> {
        let url = "https://my.zerotier.com/api/v1/network";
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let resp = self
            .client
            .post(url)
            .header(reqwest::header::AUTHORIZATION, bearer_token)
            .body("{}")
            .send()?;
        self.response_arr_handler(resp)
    }

    fn find_network_by_id(&self, network_id: &String) -> Result<NetworkResult> {
        let url = format!("https://my.zerotier.com/api/v1/network/{}", network_id);
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let resp = self
            .client
            .get(url.deref())
            .header(reqwest::header::AUTHORIZATION, bearer_token)
            .send()?;
        self.response_arr_handler(resp)
    }

    fn update_network_config(
        &self,
        network_id: &String,
        payload: crate::NetworkUpdatePayload,
    ) -> Result<NetworkResult> {
        let url = format!("https://my.zerotier.com/api/v1/network/{}", network_id);
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let resp = self
            .client
            .post(url.deref())
            .header(reqwest::header::AUTHORIZATION, bearer_token)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&payload)
            .send()?;
        self.response_arr_handler(resp)
    }

    fn delete_network(&self, network_id: &String) -> Result<()> {
        let url = format!("https://my.zerotier.com/api/v1/network/{}", network_id);
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let mut resp = self
            .client
            .delete(url.deref())
            .header(reqwest::header::AUTHORIZATION, bearer_token)
            .send()?;
        if !resp.status().is_success() {
            let msg = resp.text()?;
            return Err(anyhow!(msg));
        }
        Ok(())
    }
}

pub struct ZtNetworkMemberCentral {
    token: String,
    client: reqwest::Client,
}

impl ZtNetworkMemberCentral {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: reqwest::Client::new(),
        }
    }
}

impl NetworkMemberCentral for ZtNetworkMemberCentral {
    fn find_network_member_list(&self, network_id: &String) -> Result<Vec<NetworkMemberResult>> {
        let url = format!("https://my.zerotier.com/api/network/{}/member", network_id);
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let mut resp = self
            .client
            .get(&url)
            .header(reqwest::header::AUTHORIZATION, bearer_token)
            .send()?;
        if resp.status().is_success() {
            let result = resp.json::<Vec<crate::NetworkMemberResult>>()?;
            return Ok(result);
        }
        let msg = resp.text()?;
        Err(anyhow!(msg))
    }

    fn find_member(&self, network_id: &String, member_id: &String) -> Result<NetworkMemberResult> {
        let url = format!(
            "https://my.zerotier.com/api/v1/network/{}/member/{}",
            network_id, member_id
        );
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let mut resp = self
            .client
            .get(url.deref())
            .header(reqwest::header::AUTHORIZATION, bearer_token)
            .send()?;
        if resp.status().is_success() {
            let result = resp.json::<crate::NetworkMemberResult>()?;
            return Ok(result);
        }
        let msg = resp.text()?;
        Err(anyhow!(msg))
    }

    fn update_member(
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
        let mut resp = self
            .client
            .post(url.deref())
            .header(reqwest::header::AUTHORIZATION, bearer_token)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&payload)
            .send()?;
        if resp.status().is_success() {
            let result = resp.json::<NetworkMemberResult>()?;
            return Ok(result);
        }
        let msg = resp.text()?;
        Err(anyhow!(msg))
    }

    fn delete_member(&self, network_id: &String, member_id: &String) -> Result<()> {
        let url = format!(
            "https://my.zerotier.com/api/v1/network/{}/member/{}",
            network_id, member_id
        );
        let bearer_token = crate::format::BaseReqFormat::format_bearer_token(&self.token);
        let mut resp = self
            .client
            .delete(url.deref())
            .header(reqwest::header::AUTHORIZATION, bearer_token)
            .send()?;
        if !resp.status().is_success() {
            let msg = resp.text()?;
            return Err(anyhow!(msg));
        }
        Ok(())
    }
}
