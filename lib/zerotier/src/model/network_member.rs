#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkMemberResult {
    #[serde(default)]
    pub id: Option<String>,

    #[serde(default)]
    #[serde(rename = "type")]
    pub _type: Option<String>,

    #[serde(default)]
    pub clock: i64,

    #[serde(default)]
    pub networkId: Option<String>,

    #[serde(default)]
    pub nodeId: Option<String>,

    #[serde(default)]
    pub controllerId: Option<String>,

    #[serde(default)]
    pub hidden: bool,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub online: bool,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub config: Option<NetworkMemberConfig>,

    #[serde(default)]
    pub lastOnline: i64,

    #[serde(default)]
    pub physicalAddress: Option<String>,

    #[serde(default)]
    pub physicalLocation: Option<String>,

    #[serde(default)]
    pub clientVersion: Option<String>,

    #[serde(default)]
    pub protocolVersion: i64,

    #[serde(default)]
    pub supportsRulesEngine: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkMemberConfig {
    #[serde(default)]
    pub activeBridge: bool,

    #[serde(default)]
    pub address: Option<String>,

    #[serde(default)]
    pub authorized: bool,

    #[serde(default)]
    pub creationTime: i64,

    #[serde(default)]
    pub id: Option<String>,

    #[serde(default)]
    pub identity: Option<String>,

    #[serde(default)]
    pub ipAssignments: Vec<String>,

    #[serde(default)]
    pub lastAuthorizedTime: i64,

    #[serde(default)]
    pub lastDeauthorizedTime: i64,

    #[serde(default)]
    pub noAutoAssignIps: bool,

    #[serde(default)]
    pub nwid: Option<String>,

    #[serde(default)]
    pub objtype: Option<String>,

    #[serde(default)]
    pub remoteTraceLevel: i64,

    #[serde(default)]
    pub remoteTraceTarget: Option<String>,

    #[serde(default)]
    pub revision: i64,

    #[serde(default)]
    pub vMajor: i64,

    #[serde(default)]
    pub vMinor: i64,

    #[serde(default)]
    pub vRev: i64,

    #[serde(default)]
    pub vProto: i64,

    #[serde(default)]
    pub ssoExempt: bool,
}

// update payload
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkMemberUpdatePayload {
    #[serde(default)]
    pub hidden: bool,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub config: Option<NetworkMemberUpdatePayloadConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkMemberUpdatePayloadConfig {
    #[serde(default)]
    pub activeBridge: bool,

    #[serde(default)]
    pub authorized: bool,

    #[serde(default)]
    pub ipAssignments: Option<Vec<String>>,

    #[serde(default)]
    pub noAutoAssignIps: bool,
}
