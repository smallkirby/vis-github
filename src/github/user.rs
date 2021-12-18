use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::context::Context;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub login: String,
  pub id: u64,
  pub url: String,
  pub name: String,
  pub blog: Option<String>,
  pub location: Option<String>,
  pub email: Option<String>,
  pub public_repos: u64,
  pub bio: String,
  pub followers: u64,
  pub following: u64,
  pub created_at: DateTime<Utc>,
}

fn fetchUserFromFile(owner: &str, cache_dir: &str) -> User {
  let jstring = fs::read_to_string(format!("{}/{}/user.json", cache_dir, owner)).unwrap();
  let json: User = serde_json::from_str(&jstring).unwrap();
  json
}

fn fetchUserFromNet(owner: &str) {
  unimplemented!();
}

pub fn fetchUser(context: &Context) -> User {
  if context.force_use_cache {
    fetchUserFromFile(&context.owner, &context.cache_path)
  } else {
    unimplemented!();
  }
}
