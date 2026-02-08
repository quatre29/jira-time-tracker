use chrono::{DateTime, Duration, Utc};

pub struct JiraTicket {
    pub branch_name: String,
    pub description: String,
    pub logged_time: Duration,
    pub total_estimate: Duration,
    pub last_updated: DateTime<Utc>,
}

impl JiraTicket {
    // NOTE:  we will update the rest of the fields when we get a response from jira api
    pub fn new(branch_name: &str) -> Self {
        Self {
            branch_name: branch_name.to_string(),
            description: String::new(),
            logged_time: Duration::zero(),
            total_estimate: Duration::zero(),
            last_updated: Utc::now(),
        }
    }
}
