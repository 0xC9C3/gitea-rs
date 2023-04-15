/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Branch : Branch represents a repository branch



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Branch {
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<Box<crate::models::PayloadCommit>>,
    #[serde(rename = "effective_branch_protection_name", skip_serializing_if = "Option::is_none")]
    pub effective_branch_protection_name: Option<String>,
    #[serde(rename = "enable_status_check", skip_serializing_if = "Option::is_none")]
    pub enable_status_check: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "protected", skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
    #[serde(rename = "required_approvals", skip_serializing_if = "Option::is_none")]
    pub required_approvals: Option<i64>,
    #[serde(rename = "status_check_contexts", skip_serializing_if = "Option::is_none")]
    pub status_check_contexts: Option<Vec<String>>,
    #[serde(rename = "user_can_merge", skip_serializing_if = "Option::is_none")]
    pub user_can_merge: Option<bool>,
    #[serde(rename = "user_can_push", skip_serializing_if = "Option::is_none")]
    pub user_can_push: Option<bool>,
}

impl Branch {
    /// Branch represents a repository branch
    pub fn new() -> Branch {
        Branch {
            commit: None,
            effective_branch_protection_name: None,
            enable_status_check: None,
            name: None,
            protected: None,
            required_approvals: None,
            status_check_contexts: None,
            user_can_merge: None,
            user_can_push: None,
        }
    }
}


