use base64::{engine::general_purpose, Engine};
use futures::{stream, StreamExt};
use reqwest::{header, Client};
use serde_json::json;

use super::dto::*;
use super::JiraConfig;
use crate::api::models::{JiraTicket, JiraUser};
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

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self { client, config }
    }

    fn url(&self, path: &str) -> String {
        format!("{}rest/api/3{}", self.config.base_url, path)
    }

    pub async fn fetch_assigned_tickets(&self) -> Result<Vec<JiraTicket>> {
        let res = self
            .client
            .get(self.url("/search?jql=assignee=currentUser()"))
            .send()
            .await?
            .error_for_status()?;

        let data: JiraSearchResponse = res.json().await?;

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
}
