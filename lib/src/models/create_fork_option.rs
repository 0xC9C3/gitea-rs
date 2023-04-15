/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateForkOption : CreateForkOption options for creating a fork



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateForkOption {
    /// name of the forked repository
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// organization name, if forking into an organization
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
}

impl CreateForkOption {
    /// CreateForkOption options for creating a fork
    pub fn new() -> CreateForkOption {
        CreateForkOption {
            name: None,
            organization: None,
        }
    }
}

