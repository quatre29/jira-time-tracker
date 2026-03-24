use std::env;

#[derive(Clone)]
pub struct JiraConfig {
    pub base_url: String,
    pub email: String,
    pub api_token: String,
}

impl JiraConfig {
    pub fn from_env() -> Self {
        Self {
            base_url: env::var("JIRA_BASE_URL").expect("Missing JIRA_BASE_URL"),
            email: env::var("JIRA_EMAIL").expect("Missing JIRA_EMAIL"),
            api_token: env::var("JIRA_API_TOKEN").expect("Missing JIRA_TOKEN"),
        }
    }
}
