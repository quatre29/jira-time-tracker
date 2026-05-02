use crate::api::dto::{JiraIssueDto, JiraUserDto};

pub struct JiraUser {
    pub account_id: String,
    pub display_name: String,
    pub email_address: String,
    pub time_zone: String,
    pub locale: String,
}

/// Stats about the user's workload and time logging.
#[derive(Debug, Clone)]
pub struct UserStats {
    pub in_progress_count: u64,
    pub weekly_worklogs: Vec<WorklogEntry>,
}

/// A single worklog entry from Jira.
#[derive(Debug, Clone)]
pub struct WorklogEntry {
    pub ticket_key: String,
    pub time_spent: String,
    pub time_spent_seconds: u64,
    pub started: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IssueType {
    Bug,
    Story,
    Task,
    Epic,
    Subtask,
    Other(String),
}

use ratatui::style::{Color, Style};

impl IssueType {
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "bug" => IssueType::Bug,
            "story" => IssueType::Story,
            "task" => IssueType::Task,
            "epic" => IssueType::Epic,
            "sub-task" | "subtask" => IssueType::Subtask,
            other => IssueType::Other(other.to_string()),
        }
    }

    /// Colored icon matching Jira's issue type colors.
    /// Returns (symbol, color) to render in the terminal.
    pub fn icon(&self) -> &str {
        match self {
            IssueType::Bug =>      "●",  // red circle — Jira bug
            IssueType::Story =>    "▲",  // green triangle — Jira story
            IssueType::Task =>     "■",  // blue square — Jira task
            IssueType::Epic =>     "⚡",  // purple bolt — Jira epic
            IssueType::Subtask =>  "◆",  // blue diamond — Jira subtask
            IssueType::Other(_) => "○",  // grey circle — unknown
        }
    }

    /// The color Jira uses for this issue type icon.
    pub fn color(&self) -> Color {
        match self {
            IssueType::Bug =>      Color::Rgb(0xe5, 0x49, 0x3a), // Jira red
            IssueType::Story =>    Color::Rgb(0x36, 0xb3, 0x7e), // Jira green
            IssueType::Task =>     Color::Rgb(0x42, 0x8f, 0xdc), // Jira blue
            IssueType::Epic =>     Color::Rgb(0x90, 0x4e, 0xe2), // Jira purple
            IssueType::Subtask =>  Color::Rgb(0x42, 0x8f, 0xdc), // Jira blue
            IssueType::Other(_) => Color::Rgb(0x6b, 0x77, 0x8d), // grey
        }
    }

    pub fn style(&self) -> Style {
        Style::default().fg(self.color())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct JiraTicket {
    pub id: String,
    pub key: String,
    pub title: String,
    pub issue_type: IssueType,

    pub status: String,
    pub status_category: String,
    pub priority: String,
    pub assignee: String,
    pub reporter: String,
    pub labels: Vec<String>,
    pub created: String,
    pub updated: String,

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

        let issue_type = dto
            .fields
            .issuetype
            .and_then(|t| t.name)
            .map(|n| IssueType::from_name(&n))
            .unwrap_or(IssueType::Other("unknown".to_string()));

        let status = dto.fields.status
            .as_ref()
            .and_then(|s| s.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let status_category = dto.fields.status
            .and_then(|s| s.status_category)
            .and_then(|c| c.key)
            .unwrap_or_else(|| "undefined".to_string());

        let priority = dto.fields.priority
            .and_then(|p| p.name)
            .unwrap_or_else(|| "None".to_string());

        let assignee = dto.fields.assignee
            .and_then(|a| a.display_name)
            .unwrap_or_else(|| "Unassigned".to_string());

        let reporter = dto.fields.reporter
            .and_then(|r| r.display_name)
            .unwrap_or_else(|| "Unknown".to_string());

        let created = dto.fields.created
            .map(|c| format_jira_date(&c))
            .unwrap_or_default();

        let updated = dto.fields.updated
            .map(|u| format_jira_date(&u))
            .unwrap_or_default();

        Self {
            id: dto.id,
            key: dto.key,
            title: dto.fields.summary,
            issue_type,
            status,
            status_category,
            priority,
            assignee,
            reporter,
            labels: dto.fields.labels,
            created,
            updated,

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

/// Formats a Jira ISO datetime string to a short readable date.
fn format_jira_date(iso: &str) -> String {
    // Jira sends "2024-01-15T10:30:00.000+0000" — just take the date part
    if iso.len() >= 10 {
        iso[..10].to_string()
    } else {
        iso.to_string()
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

