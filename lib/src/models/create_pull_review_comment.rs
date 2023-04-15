/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreatePullReviewComment : CreatePullReviewComment represent a review comment for creation api



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreatePullReviewComment {
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// if comment to new file line or 0
    #[serde(rename = "new_position", skip_serializing_if = "Option::is_none")]
    pub new_position: Option<i64>,
    /// if comment to old file line or 0
    #[serde(rename = "old_position", skip_serializing_if = "Option::is_none")]
    pub old_position: Option<i64>,
    /// the tree path
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl CreatePullReviewComment {
    /// CreatePullReviewComment represent a review comment for creation api
    pub fn new() -> CreatePullReviewComment {
        CreatePullReviewComment {
            body: None,
            new_position: None,
            old_position: None,
            path: None,
        }
    }
}

