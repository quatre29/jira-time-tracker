use base64::{Engine, engine::general_purpose};
use futures::{StreamExt, stream};
use reqwest::{Client, header};

use super::JiraConfig;
use super::dto::*;
use crate::Result;
use crate::api::models::{JiraTicket, JiraUser};

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

    pub async fn fetch_tickets(&self, keys: Vec<String>) -> Result<Vec<JiraTicket>> {
        if keys.is_empty() {
            return Ok(vec![]);
        }

        let client = self.clone();

        let tickets = stream::iter(keys)
            .map(|key| {
                let client = client.clone();

                async move {
                    match client.fetch_ticket(&key).await {
                        Ok(ticket) => Some(ticket),
                        Err(e) => {
                            eprintln!("Failed {}: {}", key, e);
                            None
                        }
                    }
                }
            })
            .buffer_unordered(5)
            .filter_map(|opt| async move { opt })
            .collect::<Vec<_>>()
            .await;

        Ok(tickets)
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
        let res = self.client.get(self.url("/myself")).send().await?.error_for_status()?;
        let user: JiraUserDto = res.json().await?;

        Ok(JiraUser::from(user))
    }

    pub async fn log_time(&self, ticket_id: String, time: u32) -> Result<JiraTicket> {
        todo!()
    }
}
