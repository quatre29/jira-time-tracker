use crate::api::dto::{JiraIssueDto, JiraUserDto};

pub struct JiraUser {
    pub account_id: String,
    pub display_name: String,
    pub email_address: String,
    pub time_zone: String,
    pub locale: String,
}

#[derive(Debug, Clone, PartialEq)]
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

    pub subtask_keys: Vec<String>,
    pub subtasks: Vec<JiraTicket>,
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

            subtask_keys: dto.fields.subtasks.into_iter().map(|s| s.key).collect(),
            subtasks: vec![],
        }
    }
}

impl From<JiraUserDto> for JiraUser {
    fn from(dto: JiraUserDto) -> Self {
        Self {
            account_id: dto.account_id.unwrap_or_default(),
            display_name: dto.display_name.unwrap_or_default(),
            email_address: dto.email_address.unwrap_or_default(),
            locale: dto.locale.unwrap_or_default(),
            time_zone: dto.time_zone.unwrap_or_default(),
        }
    }
}

