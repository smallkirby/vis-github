use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::context::Context;

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
  pub id: u64,
  pub name: String,
  pub private: bool,
  pub fork: bool,
  pub description: Option<String>,
  pub url: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub homepage: Option<String>,
  pub language: Option<String>,
  pub forks_count: u64,
  pub archived: bool,
  pub open_issues_count: u64,
  pub watchers: u64,
  pub default_branch: String,
}

fn fetch_repositories_from_file(owner: &str, cache_dir: &str) -> Vec<Repository> {
  let jstring = fs::read_to_string(format!("{}/{}/repos.json", cache_dir, owner)).unwrap();
  let json: Vec<Repository> = serde_json::from_str(&jstring).unwrap();
  json
}

fn fetch_repositories_from_net(_owner: &str) -> Vec<Repository> {
  unimplemented!();
}

pub fn fetch_repositories(context: &Context) -> Vec<Repository> {
  if context.force_use_cache {
    fetch_repositories_from_file(&context.owner, &context.cache_path)
  } else {
    unimplemented!();
  }
}
