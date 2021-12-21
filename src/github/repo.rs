/*
  Thsi file defines Repository related types and functions.
*/

use super::{client::GithubClient, license::License};
use crate::context::Context;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
  pub license: Option<License>,
}

impl Repository {
  pub fn is_target(&self, context: &Context) -> bool {
    let fork = context.ignore_fork && self.fork;
    let private = context.ignore_private && self.private;

    !(fork || private)
  }
}

pub fn fetch_repositories_from_file(
  owner: &str,
  cache_dir: &str,
) -> Result<Vec<Repository>, String> {
  let mut result_repos = vec![];
  let basedir = PathBuf::from(format!("{}/{}/repos", cache_dir, owner));
  if !basedir.exists() || !basedir.is_dir() {
    return Err(format!(
      "Cache path for repo({}) not found.",
      basedir.to_string_lossy()
    ));
  }
  for _repo_dir in fs::read_dir(&basedir).unwrap() {
    match _repo_dir {
      Ok(repo_dir) => {
        let path = PathBuf::from(format!("{}/repo.json", repo_dir.path().to_string_lossy()));
        if path.is_file() {
          let jstring = fs::read_to_string(&path).unwrap();
          let repo: Repository = serde_json::from_str(&jstring).unwrap();
          result_repos.push(repo);
        }
      }
      Err(err) => return Err(err.to_string()),
    }
  }
  Ok(result_repos)
}

pub fn save_repos(context: &Context, repos: &Vec<Repository>) -> Result<(), String> {
  let mut save_dir = PathBuf::from(&context.cache_path);
  let user = context.owner.as_str();
  if !save_dir.exists() || !save_dir.is_dir() {
    if fs::create_dir(&save_dir).is_err() {
      return Err(
        format!(
          "Failed to create cache directory: {}",
          save_dir.to_string_lossy()
        )
        .into(),
      );
    }
  }
  save_dir.push(user);
  if !save_dir.exists() || !save_dir.is_dir() {
    if fs::create_dir(&save_dir).is_err() {
      return Err(
        format!(
          "Failed to create cache directory: {}",
          save_dir.to_string_lossy()
        )
        .into(),
      );
    }
  }
  save_dir.push("repos");
  if !save_dir.exists() || !save_dir.is_dir() {
    if fs::create_dir(&save_dir).is_err() {
      return Err(
        format!(
          "Failed to create cache directory: {}",
          save_dir.to_string_lossy()
        )
        .into(),
      );
    }
  }

  for repo in repos {
    let mut repo_path = save_dir.clone();
    repo_path.push(&repo.name);
    if !repo_path.exists() || !repo_path.is_dir() {
      if fs::create_dir(&repo_path).is_err() {
        return Err(
          format!(
            "Failed to create cache directory: {}",
            repo_path.to_string_lossy()
          )
          .into(),
        );
      }
    }
    repo_path.push("repo.json");
    match fs::write(&repo_path, serde_json::to_string(repo).unwrap()) {
      Ok(()) => continue,
      Err(err) => {
        return Err(format!(
          "Failed to create repo cache: {} : {}",
          repo_path.to_string_lossy(),
          err.to_string()
        ))
      }
    }
  }

  Ok(())
}

pub fn fetch_repositories_from_net(context: &Context) -> Result<Vec<Repository>, String> {
  let per_page = 100;
  let mut all_repos = vec![];
  for ix in 0..context.repo_limit_per_user / per_page + 1 {
    let client = GithubClient::new(
      &format!(
        "users/{}/repos?per_page={}&page={}",
        &context.owner,
        per_page,
        ix + 1
      ),
      &context.apitoken,
    );
    let response = client.get()?;
    let mut repos: Vec<Repository> = response.json().unwrap();
    let fetched_size = repos.len();
    all_repos.append(&mut repos);
    if fetched_size < per_page as usize {
      break;
    }
  }
  Ok(all_repos)
}

pub fn fetch_repositories(context: &Context) -> Result<Vec<Repository>, String> {
  if context.force_use_cache {
    fetch_repositories_from_file(&context.owner, &context.cache_path)
  } else {
    match fetch_repositories_from_net(&context) {
      Ok(repos) => match save_repos(&context, &repos) {
        Ok(()) => Ok(repos),
        Err(err) => Err(err.to_string()),
      },
      Err(err) => Err(err.to_string()),
    }
  }
}
