use octocrab::{Octocrab, models::pulls::PullRequest};
use anyhow::Result;
use std::borrow::Cow;

pub struct GitHubClient {
    octocrab: Octocrab,
    owner: String,
    repo: String,
}

impl GitHubClient {
    pub fn new(token: String, owner: String, repo: String) -> Result<Self> {
        let octocrab = Octocrab::builder()
            .personal_token(token)
            .build()?;
        Ok(Self { octocrab, owner, repo })
    }

    pub async fn create_pull_request(
        &self,
        title: String,
        head: String, // Source branch
        base: String, // Target branch
        body: Option<String>,
    ) -> Result<PullRequest> {
        let pr = self.octocrab
            .pulls(&self.owner, &self.repo)
            .create(title, head, base)
                        .body(body.map(Into::into))
            .send()
            .await?;
        Ok(pr)
    }
}
