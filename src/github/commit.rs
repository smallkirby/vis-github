use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::context::Context;


#[derive(Serialize, Deserialize, Debug)]
pub struct CommitAuthor {
  pub name: String,
  pub email: String,
  pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitData {
  message: String,
  author: CommitAuthor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
  sha: String,
  commit: CommitData,
  url: String,
}

pub fn fetchCommitsFromFile(cache_dir: &str, owner: &str, repo_name: &str) -> Vec<Commit> {
  let file_path = format!("{}/{}/repos/{}/commits.json", cache_dir, owner, repo_name);
  println!("{}", file_path);
  let jstring = fs::read_to_string(file_path).unwrap();
  let json: Vec<Commit> = serde_json::from_str(&jstring).unwrap();
  json
}

pub fn fetchCommits(context: &Context, repo_name: &str) -> Vec<Commit> {
  if context.force_use_cache {
    fetchCommitsFromFile(&context.cache_path, &context.owner, repo_name)
  } else {
    unimplemented!();
  }
}
