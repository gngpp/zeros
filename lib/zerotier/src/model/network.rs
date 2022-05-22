#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkResult {
    #[serde(default)]
    pub id: Option<String>,

    #[serde(default)]
    #[serde(rename = "type")]
    pub _type: Option<String>,

    #[serde(default)]
    pub clock: i64,

    #[serde(default)]
    pub config: Option<NetworkConfig>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub rulesSource: Option<String>,

    #[serde(default)]
    pub ownerId: Option<String>,

    #[serde(default)]
    pub onlineMemberCount: i64,

    #[serde(default)]
    pub authorizedMemberCount: i64,

    #[serde(default)]
    pub totalMemberCount: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    #[serde(default)]
    pub authTokens: Option<String>,

    #[serde(default)]
    pub creationTime: i64,

    #[serde(default)]
    pub enableBroadcast: bool,

    #[serde(default)]
    pub id: Option<String>,

    #[serde(default)]
    pub ipAssignmentPools: Option<Vec<IpAssignmentPools>>,

    #[serde(default)]
    pub lastModified: i64,

    #[serde(default)]
    pub mtu: i64,

    #[serde(default)]
    pub multicastLimit: i64,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub private: bool,

    #[serde(default)]
    pub remoteTraceLevel: i64,

    #[serde(default)]
    pub remoteTraceTarget: Option<String>,

    #[serde(default)]
    pub routes: Option<Vec<Routes>>,

    #[serde(default)]
    pub tags: Option<Vec<String>>,

    #[serde(default)]
    pub v4AssignMode: Option<V4AssignMode>,

    #[serde(default)]
    pub v6AssignMode: Option<V6AssignMode>,

    #[serde(default)]
    pub dns: Option<Dns>,

    #[serde(default)]
    pub ssoConfig: Option<SsoConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dns {
    #[serde(default)]
    pub domain: Option<String>,

    #[serde(default)]
    pub servers: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpAssignmentPools {
    #[serde(default)]
    pub ipRangeStart: Option<String>,

    #[serde(default)]
    pub ipRangeEnd: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Routes {
    #[serde(default)]
    pub target: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SsoConfig {
    #[serde(default)]
    pub enabled: bool,

    #[serde(default)]
    pub mode: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct V4AssignMode {
    #[serde(default)]
    pub zt: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct V6AssignMode {
    #[serde(default)]
    #[serde(rename = "6plane")]
    pub plane6: bool,

    #[serde(default)]
    pub rfc4193: bool,

    #[serde(default)]
    pub zt: bool,
}

// update payload
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkUpdatePayloadConfig {
    #[serde(default)]
    dns: Option<Dns>,

    #[serde(default)]
    enableBroadcast: bool,

    #[serde(default)]
    ipAssignmentPools: Option<Vec<IpAssignmentPools>>,

    #[serde(default)]
    mtu: i64,

    #[serde(default)]
    multicastLimit: i64,

    #[serde(default)]
    name: Option<String>,

    #[serde(default)]
    private: bool,

    #[serde(default)]
    routes: Option<Vec<Routes>>,

    #[serde(default)]
    v4AssignMode: Option<V4AssignMode>,

    #[serde(default)]
    v6AssignMode: Option<V6AssignMode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkUpdatePayload {
    #[serde(default)]
    config: Option<NetworkUpdatePayloadConfig>,

    #[serde(default)]
    description: Option<String>,

    #[serde(default)]
    rulesSource: Option<String>,

    #[serde(default)]
    ownerId: Option<String>,
}
