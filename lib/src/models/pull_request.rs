/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PullRequest : PullRequest represents a pull request



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PullRequest {
    #[serde(rename = "allow_maintainer_edit", skip_serializing_if = "Option::is_none")]
    pub allow_maintainer_edit: Option<bool>,
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Box<crate::models::User>>,
    #[serde(rename = "assignees", skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<crate::models::User>>,
    #[serde(rename = "base", skip_serializing_if = "Option::is_none")]
    pub base: Option<Box<crate::models::PrBranchInfo>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "closed_at", skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "diff_url", skip_serializing_if = "Option::is_none")]
    pub diff_url: Option<String>,
    #[serde(rename = "due_date", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "head", skip_serializing_if = "Option::is_none")]
    pub head: Option<Box<crate::models::PrBranchInfo>>,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "is_locked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::models::Label>>,
    #[serde(rename = "merge_base", skip_serializing_if = "Option::is_none")]
    pub merge_base: Option<String>,
    #[serde(rename = "merge_commit_sha", skip_serializing_if = "Option::is_none")]
    pub merge_commit_sha: Option<String>,
    #[serde(rename = "mergeable", skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(rename = "merged", skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    #[serde(rename = "merged_at", skip_serializing_if = "Option::is_none")]
    pub merged_at: Option<String>,
    #[serde(rename = "merged_by", skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<Box<crate::models::User>>,
    #[serde(rename = "milestone", skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Box<crate::models::Milestone>>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(rename = "patch_url", skip_serializing_if = "Option::is_none")]
    pub patch_url: Option<String>,
    /// StateType issue state type
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
}

impl PullRequest {
    /// PullRequest represents a pull request
    pub fn new() -> PullRequest {
        PullRequest {
            allow_maintainer_edit: None,
            assignee: None,
            assignees: None,
            base: None,
            body: None,
            closed_at: None,
            comments: None,
            created_at: None,
            diff_url: None,
            due_date: None,
            head: None,
            html_url: None,
            id: None,
            is_locked: None,
            labels: None,
            merge_base: None,
            merge_commit_sha: None,
            mergeable: None,
            merged: None,
            merged_at: None,
            merged_by: None,
            milestone: None,
            number: None,
            patch_url: None,
            state: None,
            title: None,
            updated_at: None,
            url: None,
            user: None,
        }
    }
}


