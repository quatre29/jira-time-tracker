use base64::{engine::general_purpose, Engine};
use chrono::{Datelike, Local, NaiveDate};
use futures::{stream, StreamExt};
use reqwest::{header, Client};
use serde_json::json;

use super::dto::*;
use super::JiraConfig;
use crate::api::models::{JiraTicket, JiraUser, WorklogEntry};
use crate::Result;

#[derive(Clone)]
pub struct JiraClient {
    client: Client,
    config: JiraConfig,
}

impl JiraClient {
    pub fn new(config: JiraConfig) -> Self {
        let auth = format!("{}:{}", config.email, config.api_token);
        let encoded = general_purpose::STANDARD.encode(auth);

        let mut headers = header::HeaderMap::new();

        headers.insert(
            header::AUTHORIZATION,
            format!("Basic {}", encoded).parse().unwrap(),
        );
        headers.insert(
            header::ACCEPT,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client, config }
    }

    fn url(&self, path: &str) -> String {
        format!("{}rest/api/3{}", self.config.base_url, path)
    }

    /// Performs a JQL search using the new POST /search/jql endpoint.
    /// Returns full issue data (requires fields that include summary etc).
    async fn jql_search(&self, jql: &str, fields: &[&str], max_results: u32) -> Result<JiraSearchResponse> {
        let body = json!({
            "jql": jql,
            "maxResults": max_results,
            "fields": fields,
        });

        let res = self
            .client
            .post(self.url("/search/jql"))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let data: JiraSearchResponse = res.json().await?;
        Ok(data)
    }

    /// Performs a lightweight JQL search — only returns issue keys/ids.
    async fn jql_search_keys(&self, jql: &str, max_results: u32) -> Result<JiraSearchLightResponse> {
        let body = json!({
            "jql": jql,
            "maxResults": max_results,
            "fields": ["key"],
        });

        let res = self
            .client
            .post(self.url("/search/jql"))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let data: JiraSearchLightResponse = res.json().await?;
        Ok(data)
    }

    pub async fn fetch_assigned_tickets(&self) -> Result<Vec<JiraTicket>> {
        let data = self.jql_search(
            "assignee=currentUser()",
            &["summary", "issuetype", "status", "priority", "assignee", "reporter", "labels", "created", "updated", "timetracking", "subtasks"],
            50,
        ).await?;

        Ok(data.issues.into_iter().map(JiraTicket::from).collect())
    }

    pub async fn fetch_tickets(
        &self,
        keys: Vec<String>,
    ) -> (Vec<JiraTicket>, Vec<(String, anyhow::Error)>) {
        if keys.is_empty() {
            return (vec![], vec![]);
        }

        let client = self.clone();

        let results = stream::iter(keys)
            .map(|key| {
                let client = client.clone();

                async move {
                    let result = client.fetch_ticket(&key).await;
                    (key, result)
                }
            })
            .buffer_unordered(5)
            .collect::<Vec<_>>()
            .await;

        let mut tickets = Vec::new();
        let mut errors = Vec::new();

        for (key, result) in results {
            match result {
                Ok(ticket) => tickets.push(ticket),
                Err(e) => errors.push((key, e))
            }
        }

        (tickets, errors)
    }


    pub async fn fetch_ticket(&self, ticket_key: &str) -> Result<JiraTicket> {
        let res = self
            .client
            .get(self.url(&format!("/issue/{}", ticket_key)))
            .send()
            .await?
            .error_for_status()?;

        let issue: JiraIssueDto = res.json().await?;

        Ok(issue.into())
    }

    pub async fn fetch_user(&self) -> Result<JiraUser> {
        let res = self
            .client
            .get(self.url("/myself"))
            .send()
            .await?
            .error_for_status()?;
        let user: JiraUserDto = res.json().await?;

        Ok(JiraUser::from(user))
    }

    pub async fn log_time(
        &self,
        ticket_id: String,
        time_spent_seconds: u64,
        started: String,
        description: String,
    ) -> Result<String> {
        let body = json!({
            "comment": {
                "content": [
                    {
                        "content": [
                            {
                                "text": description,
                                "type": "text"
                            }
                        ],
                        "type": "paragraph"
                    }
                ],
                "type": "doc",
                "version": 1
            },
            "started": started,
            "timeSpentSeconds": time_spent_seconds
        });

        let res = self
            .client
            .post(self.url(&format!("/issue/{}/worklog", ticket_id)))
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let text = res.text().await?;

        Ok(text)
    }

    /// Fetches the count of in-progress tickets assigned to the current user.
    pub async fn fetch_in_progress_count(&self) -> Result<u64> {
        let jql = "assignee = currentUser() AND statusCategory = \"In Progress\"";
        let data = self.jql_search_keys(jql, 50).await?;
        let mut count = data.issues.len() as u64;

        if !data.is_last {
            count = 50; // cap it — burnout meter maxes out anyway
        }

        Ok(count)
    }

    /// Fallback: some Jira instances use different status category names
    async fn fetch_in_progress_count_fallback(&self) -> Result<u64> {
        let jql = "assignee = currentUser() AND status != Done AND status != \"To Do\"";
        let data = self.jql_search_keys(jql, 50).await?;
        Ok(data.issues.len() as u64)
    }

    /// Fetches worklogs for the current user this week.
    pub async fn fetch_weekly_worklogs(&self) -> Result<Vec<WorklogEntry>> {
        let today = Local::now().date_naive();
        let monday = week_start(today);
        let monday_str = monday.format("%Y-%m-%d").to_string();

        // Find tickets where current user logged work this week
        let jql = format!(
            "worklogAuthor = currentUser() AND worklogDate >= \"{}\"",
            monday_str
        );

        let data = self.jql_search_keys(&jql, 50).await;

        let issues = match data {
            Ok(d) => d.issues,
            Err(_) => {
                // Fallback if worklogDate/worklogAuthor not supported
                return self.fetch_weekly_worklogs_fallback(&monday_str).await;
            }
        };

        if issues.is_empty() {
            return Ok(vec![]);
        }

        // Fetch worklogs from each ticket
        let client = self.clone();
        let monday_str_clone = monday_str.clone();

        let entries = stream::iter(issues)
            .map(|issue| {
                let client = client.clone();
                let monday_str = monday_str_clone.clone();

                async move {
                    client
                        .fetch_issue_worklogs_since(&issue.key, &monday_str)
                        .await
                        .unwrap_or_default()
                }
            })
            .buffer_unordered(5)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten()
            .collect();

        Ok(entries)
    }

    /// Fallback for weekly worklogs
    async fn fetch_weekly_worklogs_fallback(&self, since: &str) -> Result<Vec<WorklogEntry>> {
        let jql = format!(
            "assignee = currentUser() AND updated >= \"{}\"",
            since
        );

        let data = self.jql_search_keys(&jql, 50).await?;

        if data.issues.is_empty() {
            return Ok(vec![]);
        }

        let client = self.clone();
        let since_owned = since.to_string();

        let entries = stream::iter(data.issues)
            .map(|issue| {
                let client = client.clone();
                let since = since_owned.clone();

                async move {
                    client
                        .fetch_issue_worklogs_since(&issue.key, &since)
                        .await
                        .unwrap_or_default()
                }
            })
            .buffer_unordered(5)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten()
            .collect();

        Ok(entries)
    }

    /// Fetches worklogs for a specific issue since a given date.
    async fn fetch_issue_worklogs_since(
        &self,
        ticket_key: &str,
        since: &str,
    ) -> Result<Vec<WorklogEntry>> {
        let url = self.url(&format!(
            "/issue/{}/worklog?startedAfter={}",
            ticket_key,
            date_to_epoch_millis(since)
        ));

        let res = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let data: WorklogSearchResponse = res.json().await?;

        let entries = data
            .worklogs
            .into_iter()
            .filter_map(|w| {
                Some(WorklogEntry {
                    ticket_key: ticket_key.to_string(),
                    time_spent: w.time_spent.unwrap_or_else(|| "0m".to_string()),
                    time_spent_seconds: w.time_spent_seconds.unwrap_or(0),
                    started: w.started.unwrap_or_default(),
                })
            })
            .collect();

        Ok(entries)
    }
}

/// Returns the Monday of the current week.
fn week_start(date: NaiveDate) -> NaiveDate {
    let days_since_monday = date.weekday().num_days_from_monday();
    date - chrono::Duration::days(days_since_monday as i64)
}

/// Converts a "YYYY-MM-DD" date string to epoch milliseconds for Jira's API.
fn date_to_epoch_millis(date_str: &str) -> u64 {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_millis() as u64)
        .unwrap_or(0)
}
