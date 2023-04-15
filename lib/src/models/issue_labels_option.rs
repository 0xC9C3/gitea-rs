/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IssueLabelsOption : IssueLabelsOption a collection of labels



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueLabelsOption {
    /// list of label IDs
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<i64>>,
}

impl IssueLabelsOption {
    /// IssueLabelsOption a collection of labels
    pub fn new() -> IssueLabelsOption {
        IssueLabelsOption {
            labels: None,
        }
    }
}

