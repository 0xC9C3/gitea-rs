/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Hook : Hook a hook is a web hook when one repository changed



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Hook {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "authorization_header", skip_serializing_if = "Option::is_none")]
    pub authorization_header: Option<String>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl Hook {
    /// Hook a hook is a web hook when one repository changed
    pub fn new() -> Hook {
        Hook {
            active: None,
            authorization_header: None,
            config: None,
            created_at: None,
            events: None,
            id: None,
            r#type: None,
            updated_at: None,
        }
    }
}


