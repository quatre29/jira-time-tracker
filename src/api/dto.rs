use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraSearchResponse {
    #[serde(default)]
    pub issues: Vec<JiraIssueDto>,
    #[serde(default)]
    pub is_last: bool,
}

/// Lightweight issue from search results (may not have fields).
#[derive(Debug, Deserialize)]
pub struct JiraSearchIssueDto {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub key: String,
}

/// Search response with lightweight issues (for count/key-only queries).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraSearchLightResponse {
    #[serde(default)]
    pub issues: Vec<JiraSearchIssueDto>,
    #[serde(default)]
    pub is_last: bool,
}

#[derive(Debug, Deserialize)]
pub struct JiraIssueDto {
    pub id: String,
    pub key: String,
    pub fields: JiraFieldsDto,
}

#[derive(Debug, Deserialize)]
pub struct JiraFieldsDto {
    pub summary: String,

    #[serde(default)]
    pub issuetype: Option<IssueTypeDto>,

    #[serde(default)]
    pub status: Option<StatusDto>,

    #[serde(default)]
    pub priority: Option<PriorityDto>,

    #[serde(default)]
    pub assignee: Option<AssigneeDto>,

    #[serde(default)]
    pub reporter: Option<AssigneeDto>,

    #[serde(default)]
    pub labels: Vec<String>,

    #[serde(default)]
    pub created: Option<String>,

    #[serde(default)]
    pub updated: Option<String>,

    #[serde(default)]
    pub timetracking: Option<TimeTrackingDto>,

    #[serde(default)]
    pub subtasks: Vec<SubtaskRefDto>,
}

#[derive(Debug, Deserialize)]
pub struct IssueTypeDto {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StatusDto {
    pub name: Option<String>,
    #[serde(default, rename = "statusCategory")]
    pub status_category: Option<StatusCategoryDto>,
}

#[derive(Debug, Deserialize)]
pub struct StatusCategoryDto {
    pub key: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PriorityDto {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssigneeDto {
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SubtaskRefDto {
    pub key: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeTrackingDto {
    //In Jira Format
    pub time_spent: Option<String>,
    pub original_estimate: Option<String>,
    pub remaining_estimate: Option<String>,

    // In seconds
    pub time_spent_seconds: Option<u64>,
    pub original_estimate_seconds: Option<u64>,
    pub remaining_estimate_seconds: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraUserDto {
    pub account_id: Option<String>,
    pub display_name: Option<String>,
    pub time_zone: Option<String>,
    pub locale: Option<String>,
    pub email_address: Option<String>,
}

// ── Worklog DTOs ──────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorklogSearchResponse {
    pub worklogs: Vec<WorklogEntryDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorklogEntryDto {
    pub time_spent: Option<String>,
    pub time_spent_seconds: Option<u64>,
    pub started: Option<String>,
    pub issue_id: Option<String>,
}

// (JiraSearchCountResponse removed — using JiraSearchResponse for all searches)
