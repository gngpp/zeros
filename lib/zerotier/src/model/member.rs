use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MemberConfig {
    #[serde(default)]
    activeBridge: bool,

    #[serde(default)]
    address: Option<String>,

    #[serde(default)]
    authorized: bool,

    #[serde(default)]
    creationTime: i64,

    #[serde(default)]
    id: Option<String>,

    #[serde(default)]
    identity: Option<String>,

    #[serde(default)]
    ipAssignments: Vec<String>,

    #[serde(default)]
    lastAuthorizedTime: i64,

    #[serde(default)]
    lastDeauthorizedTime: i64,

    #[serde(default)]
    noAutoAssignIps: bool,

    #[serde(default)]
    nwid: Option<String>,

    #[serde(default)]
    objtype: Option<String>,

    #[serde(default)]
    remoteTraceLevel: i64,

    #[serde(default)]
    remoteTraceTarget: Option<String>,

    #[serde(default)]
    revision: i64,
    #[serde(default)]
    vMajor: i64,
    #[serde(default)]
    vMinor: i64,
    #[serde(default)]
    vRev: i64,
    #[serde(default)]
    vProto: i64,
    #[serde(default)]
    ssoExempt: bool,
}

impl MemberConfig {
    pub fn new() -> Self {
        Self {
            activeBridge: false,
            address: None,
            authorized: false,
            creationTime: 0,
            id: None,
            identity: None,
            ipAssignments: vec![],
            lastAuthorizedTime: 0,
            lastDeauthorizedTime: 0,
            noAutoAssignIps: false,
            nwid: None,
            objtype: None,
            remoteTraceLevel: 0,
            remoteTraceTarget: None,
            revision: 0,
            vMajor: 0,
            vMinor: 0,
            vRev: 0,
            vProto: 0,
            ssoExempt: false
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MemberResult {
    #[serde(default)]
    id: Option<String>,
    
    #[serde(default)]
    #[serde(rename = "type")]
    _type: Option<String>,

    #[serde(default)]
    clock: i64,

    #[serde(default)]
    networkId: Option<String>,

    #[serde(default)]
    nodeId: Option<String>,

    #[serde(default)]
    controllerId: Option<String>,

    #[serde(default)]
    hidden: bool,

    #[serde(default)]
    name: Option<String>,

    #[serde(default)]
    online: bool,

    #[serde(default)]
    description: Option<String>,

    config: Option<MemberConfig>,

    #[serde(default)]
    lastOnline: i64,

    #[serde(default)]
    physicalAddress: Option<String>,

    #[serde(default)]
    physicalLocation: Option<String>,

    #[serde(default)]
    clientVersion: Option<String>,

    #[serde(default)]
    protocolVersion: i64,

    #[serde(default)]
    supportsRulesEngine: bool,
}

impl MemberResult {
    pub fn new() -> Self {
        Self {
            id: None,
            _type: None,
            clock: 0,
            networkId: None,
            nodeId: None,
            controllerId: None,
            hidden: false,
            name: None,
            online: false,
            description: None,
            config: None,
            lastOnline: 0,
            physicalAddress: None,
            physicalLocation: None,
            clientVersion: None,
            protocolVersion: 0,
            supportsRulesEngine: false
        }
    }
}