/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MergePullRequestOption : MergePullRequestForm form for merging Pull Request



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MergePullRequestOption {
    #[serde(rename = "Do")]
    pub r#do: RHashDo,
    #[serde(rename = "MergeCommitID", skip_serializing_if = "Option::is_none")]
    pub merge_commit_id: Option<String>,
    #[serde(rename = "MergeMessageField", skip_serializing_if = "Option::is_none")]
    pub merge_message_field: Option<String>,
    #[serde(rename = "MergeTitleField", skip_serializing_if = "Option::is_none")]
    pub merge_title_field: Option<String>,
    #[serde(rename = "delete_branch_after_merge", skip_serializing_if = "Option::is_none")]
    pub delete_branch_after_merge: Option<bool>,
    #[serde(rename = "force_merge", skip_serializing_if = "Option::is_none")]
    pub force_merge: Option<bool>,
    #[serde(rename = "head_commit_id", skip_serializing_if = "Option::is_none")]
    pub head_commit_id: Option<String>,
    #[serde(rename = "merge_when_checks_succeed", skip_serializing_if = "Option::is_none")]
    pub merge_when_checks_succeed: Option<bool>,
}

impl MergePullRequestOption {
    /// MergePullRequestForm form for merging Pull Request
    pub fn new(r#do: RHashDo) -> MergePullRequestOption {
        MergePullRequestOption {
            r#do,
            merge_commit_id: None,
            merge_message_field: None,
            merge_title_field: None,
            delete_branch_after_merge: None,
            force_merge: None,
            head_commit_id: None,
            merge_when_checks_succeed: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashDo {
    #[serde(rename = "merge")]
    Merge,
    #[serde(rename = "rebase")]
    Rebase,
    #[serde(rename = "rebase-merge")]
    RebaseMerge,
    #[serde(rename = "squash")]
    Squash,
    #[serde(rename = "manually-merged")]
    ManuallyMerged,
}

impl Default for RHashDo {
    fn default() -> RHashDo {
        Self::Merge
    }
}

