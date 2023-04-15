/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateOrgOption : CreateOrgOption options for creating an organization



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateOrgOption {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "full_name", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "repo_admin_change_team_access", skip_serializing_if = "Option::is_none")]
    pub repo_admin_change_team_access: Option<bool>,
    #[serde(rename = "username")]
    pub username: String,
    /// possible values are `public` (default), `limited` or `private`
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl CreateOrgOption {
    /// CreateOrgOption options for creating an organization
    pub fn new(username: String) -> CreateOrgOption {
        CreateOrgOption {
            description: None,
            full_name: None,
            location: None,
            repo_admin_change_team_access: None,
            username,
            visibility: None,
            website: None,
        }
    }
}

/// possible values are `public` (default), `limited` or `private`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "limited")]
    Limited,
    #[serde(rename = "private")]
    Private,
}

impl Default for Visibility {
    fn default() -> Visibility {
        Self::Public
    }
}

