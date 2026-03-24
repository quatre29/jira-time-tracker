use chrono::{DateTime, Duration, Utc};

use crate::api::dto::JiraIssueDto;

// pub struct JiraTicket {
//     pub branch_name: String,
//     pub description: String,
//     pub logged_time: Duration,
//     pub total_estimate: Duration,
//     pub last_updated: DateTime<Utc>,
// }
//
// impl JiraTicket {
//     // NOTE:  we will update the rest of the fields when we get a response from jira api
//     pub fn new(branch_name: &str, description: impl Into<String>) -> Self {
//         Self {
//             branch_name: branch_name.to_string(),
//             //  NOTE: temp - this will be populated by Jira API
//             description: description.into(),
//             logged_time: Duration::zero(),
//             total_estimate: Duration::zero(),
//             last_updated: Utc::now(),
//         }
//     }
// }
//
pub struct JiraUser {
    pub username: String,
    pub user_id: String,
}

#[derive(Debug, Clone)]
pub struct JiraTicket {
    pub id: String,
    pub key: String,
    pub title: String,

    pub time_spent: String,
    pub original_estimate: String,
    pub remaining_estimate: String,

    pub time_spent_seconds: u64,
    pub original_estimate_seconds: u64,
    pub remaining_estimate_seconds: u64,
}

impl From<JiraIssueDto> for JiraTicket {
    fn from(dto: JiraIssueDto) -> Self {
        let time = dto.fields.timetracking;

        Self {
            id: dto.id,
            key: dto.key,
            title: dto.fields.summary,

            time_spent: time
                .as_ref()
                .and_then(|t| t.time_spent.clone())
                .unwrap_or_default(),

            original_estimate: time
                .as_ref()
                .and_then(|t| t.original_estimate.clone())
                .unwrap_or_default(),

            remaining_estimate: time
                .as_ref()
                .and_then(|t| t.remaining_estimate.clone())
                .unwrap_or_default(),

            time_spent_seconds: time
                .as_ref()
                .and_then(|t| t.time_spent_seconds)
                .unwrap_or(0),

            original_estimate_seconds: time
                .as_ref()
                .and_then(|t| t.original_estimate_seconds)
                .unwrap_or(0),

            remaining_estimate_seconds: time
                .as_ref()
                .and_then(|t| t.remaining_estimate_seconds)
                .unwrap_or(0),
        }
    }
}
