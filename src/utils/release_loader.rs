use crate::types::types::{Release, ReleaseVersion, ReleaseSource, ReleaseSourceType};
use reqwest::Client;
use semver::Version;
use uuid::Uuid;
use std::collections::HashMap;

pub struct ReleaseLoader {
    pub releases: Vec<Release>,
    pub sources: Vec<ReleaseSource>,
    pub release_sources_url: String,
    pub releases_loaded: bool,
    client: Client,
}

impl ReleaseLoader {
    pub fn new(release_sources_url: &str) -> Self {
        Self {
            releases: Vec::new(),
            sources: Vec::new(),
            release_sources_url: release_sources_url.to_string(),
            releases_loaded: false,
            client: Client::new(),
        }
    }

    pub async fn load_release_sources(&mut self) -> anyhow::Result<()> {
        let resp = self.client.get(&self.release_sources_url).send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("Failed to fetch release sources: {}", resp.status());
        }
        self.sources = resp.json().await?;
        Ok(())
    }

    async fn process_direct_release_source(&self, source: &ReleaseSource) -> anyhow::Result<Vec<Release>> {
        let resp = self.client.get(&source.url).send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("Failed to fetch direct release: {}", resp.status());
        }
        let mut releases: Vec<Release> = resp.json().await?;
        for release in &mut releases {
            release.id = Uuid::new_v4().to_string();
        }
        Ok(releases)
    }

    fn extract_channel_name(tag: &str) -> String {
        let clean = tag.trim_start_matches('v');

        if let Ok(parsed) = Version::parse(clean) {
            if !parsed.pre.is_empty() {
                let ver_str = parsed.to_string();
                let prerelease = ver_str.split('-').nth(1).unwrap_or("unknown");
                return prerelease.split('.').next().unwrap_or("unknown").to_string();
            }
            return "stable".to_string();
        }

        if let Some(caps) = regex::Regex::new(r"^([a-zA-Z]+)[-_]\d").unwrap().captures(clean) {
            return caps.get(1).unwrap().as_str().to_string();
        }

        clean.to_string()
    }

    async fn process_github_release_source(&self, source: &ReleaseSource) -> anyhow::Result<Vec<Release>> {
        let release_url = format!("https://api.github.com/repos/{}/releases", source.url);
        let resp = self.client
            .get(&release_url)
            .header("User-Agent", "tidaluna-installer")
            .send()
            .await?;

        if !resp.status().is_success() {
            anyhow::bail!("Failed to fetch GitHub release: {}", resp.status());
        }

        let data: serde_json::Value = resp.json().await?;
        let mut grouped: HashMap<String, Release> = HashMap::new();

        if let Some(array) = data.as_array() {
            for release in array {
                if let Some(tag) = release.get("tag_name").and_then(|v| v.as_str()) {
                    let channel_name = Self::extract_channel_name(tag);
                    let version = ReleaseVersion {
                        version: tag.to_string(),
                        download: format!("https://github.com/{}/releases/download/{}/luna.zip", source.url, tag),
                    };

                    grouped.entry(channel_name.clone())
                        .and_modify(|r| r.versions.push(version.clone()))
                        .or_insert(Release {
                            id: Uuid::new_v4().to_string(),
                            name: channel_name,
                            github_url: Some(format!("https://github.com/{}", source.url)),
                            versions: vec![version],
                        });
                }
            }
        }

        Ok(grouped.into_values().collect())
    }

    pub async fn load_releases(&mut self) -> anyhow::Result<&Vec<Release>> {
        if self.releases_loaded {
            return Ok(&self.releases);
        }

        self.load_release_sources().await?;

        for source in &self.sources {
            match source.source_type {
                ReleaseSourceType::Github => {
                    let github_releases = self.process_github_release_source(source).await?;
                    self.releases.extend(github_releases);
                }
                ReleaseSourceType::Direct => {
                    let direct_releases = self.process_direct_release_source(source).await?;
                    self.releases.extend(direct_releases);
                }
            }
        }

        self.releases_loaded = true;
        Ok(&self.releases)
    }
}
