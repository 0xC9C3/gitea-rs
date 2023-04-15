/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RepoCollaboratorPermission : RepoCollaboratorPermission to get repository permission for a collaborator



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RepoCollaboratorPermission {
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(rename = "role_name", skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
}

impl RepoCollaboratorPermission {
    /// RepoCollaboratorPermission to get repository permission for a collaborator
    pub fn new() -> RepoCollaboratorPermission {
        RepoCollaboratorPermission {
            permission: None,
            role_name: None,
            user: None,
        }
    }
}


