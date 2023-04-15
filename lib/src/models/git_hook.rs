/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GitHook : GitHook represents a Git repository hook



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GitHook {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GitHook {
    /// GitHook represents a Git repository hook
    pub fn new() -> GitHook {
        GitHook {
            content: None,
            is_active: None,
            name: None,
        }
    }
}

