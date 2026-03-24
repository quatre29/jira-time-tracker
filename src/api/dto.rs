use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JiraSearchResponse {
    pub issues: Vec<JiraIssueDto>,
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
    pub timetracking: Option<TimeTrackingDto>,
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
