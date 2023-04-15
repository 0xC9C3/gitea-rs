/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Package : Package represents a package



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Option<Box<crate::models::User>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<crate::models::User>>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<crate::models::Repository>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl Package {
    /// Package represents a package
    pub fn new() -> Package {
        Package {
            created_at: None,
            creator: None,
            id: None,
            name: None,
            owner: None,
            repository: None,
            r#type: None,
            version: None,
        }
    }
}


