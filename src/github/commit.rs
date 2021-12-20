/*
  This file defines Commit related types and functions.
*/

use crate::context::Context;
use super::client::*;

use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use reqwest::StatusCode;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitAuthor {
  pub name: String,
  pub email: String,
  pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitData {
  pub message: String,
  pub author: CommitAuthor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
  pub sha: String,
  pub commit: CommitData,
  pub url: String,
}

impl Commit {
  pub fn commit_date(&self) -> DateTime<Local> {
    let date = self.commit.author.date;
    let local = Local.from_utc_datetime(&date.naive_utc());
    local
  }
}

pub fn fetch_commits_from_file(cache_dir: &str, owner: &str, repo_name: &str) -> Result<Vec<Commit>, String> {
  let file_path = format!("{}/{}/repos/{}/commits.json", cache_dir, owner, repo_name);
  let jstring = match fs::read_to_string(file_path) {
    Ok(jstring) => jstring,
    Err(_err) => return Err(format!("Failed to read cache file of repository ({}).", repo_name)),
  };
  let json: Vec<Commit> = serde_json::from_str(&jstring).unwrap();
  Ok(json)
}

pub fn save_commits(context: &Context, repo_name: &str, commits: &Vec<Commit>) -> Result<(), String> {
  let mut save_dir = PathBuf::from(&context.cache_path);
  let user = context.owner.as_str();
  if !save_dir.exists() || !save_dir.is_dir() {
    if fs::create_dir(&save_dir).is_err() {
      return Err(format!("Failed to create cache directory: {}", save_dir.to_string_lossy()).into());
    }
  }
  save_dir.push(user);
  if !save_dir.exists() || !save_dir.is_dir() {
    if fs::create_dir(&save_dir).is_err() {
      return Err(format!("Failed to create cache directory: {}", save_dir.to_string_lossy()).into());
    }
  }
  save_dir.push("repos");
  if !save_dir.exists() || !save_dir.is_dir() {
    if fs::create_dir(&save_dir).is_err() {
      return Err(format!("Failed to create cache directory: {}", save_dir.to_string_lossy()).into());
    }
  }
  save_dir.push(repo_name);
  if !save_dir.exists() || !save_dir.is_dir() {
    if fs::create_dir(&save_dir).is_err() {
      return Err(format!("Failed to create cache directory: {}", save_dir.to_string_lossy()).into());
    }
  }

  let mut commit_path = save_dir.clone();
  commit_path.push("commits.json");
  if let Err(err) = fs::write(&commit_path, serde_json::to_string(commits).unwrap()) {
    Err(format!("Failed to write commits cache: {} : {}", commit_path.to_string_lossy(), err.to_string()))
  } else {
    Ok(())
  }
}

// Fetch commits for specified repository.
// This function ignores `context.ignore_private` and `context.ignore_fork`,
// hence caller should decide to call this.
pub fn fetch_commits_from_net(context: &Context, repo_name: &str) -> Result<Vec<Commit>, String> {
  let per_page = 100;
  let mut all_commits = vec!();
  for ix in 0 .. context.commit_limit_per_repo / per_page + 1 {
    let client = GithubClient::new(&format!("repos/{}/{}/commits?per_page={}&page={}", &context.owner, repo_name, per_page, ix + 1), &context.apitoken);
    let response = client.get()?;
    let mut commits: Vec<Commit> = match response.status() {
      StatusCode::CONFLICT  => vec![],
      _ => response.json().unwrap(),
    };
    commits = commits.into_iter().filter(|commit| commit.commit.author.name == context.owner).collect();
    let fetched_size = commits.len();
    all_commits.append(&mut commits);
    if fetched_size < per_page as usize {
      break;
    }
  }

  Ok(all_commits)
}

pub fn fetch_commits(context: &Context, repo_name: &str) -> Result<Vec<Commit>, String> {
  if context.force_use_cache {
    fetch_commits_from_file(&context.cache_path, &context.owner, repo_name)
  } else {
    unimplemented!();
  }
}
