/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NodeInfoServices : NodeInfoServices contains the third party sites this server can connect to via their application API



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NodeInfoServices {
    #[serde(rename = "inbound", skip_serializing_if = "Option::is_none")]
    pub inbound: Option<Vec<String>>,
    #[serde(rename = "outbound", skip_serializing_if = "Option::is_none")]
    pub outbound: Option<Vec<String>>,
}

impl NodeInfoServices {
    /// NodeInfoServices contains the third party sites this server can connect to via their application API
    pub fn new() -> NodeInfoServices {
        NodeInfoServices {
            inbound: None,
            outbound: None,
        }
    }
}


