/*
  This file defines User related types and functions.
*/

use crate::context::Context;
use super::client::GithubClient;

use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub login: String,            // username
  pub name: Option<String>,     // display name
  pub id: u64,
  pub url: String,
  pub blog: Option<String>,
  pub location: Option<String>,
  pub email: Option<String>,
  pub public_repos: u64,
  pub bio: Option<String>,
  pub followers: u64,
  pub following: u64,
  pub created_at: DateTime<Utc>,
}

fn fetch_user_from_file(owner: &str, cache_dir: &str) -> Result<User, String> {
  let jstring = match fs::read_to_string(format!("{}/{}/user.json", cache_dir, owner)) {
    Ok(s) => s,
    Err(err) => return Err(err.to_string())
  };
  let json: User = serde_json::from_str(&jstring).unwrap();
  Ok(json)
}

pub fn fetch_user_from_net(context: &Context) -> Result<User, String> {
  let client = GithubClient::new(&format!("users/{}", &context.owner), &context.apitoken);
  let response = client.get()?;
  let user: User = response.json().unwrap();
  Ok(user)
}

pub fn save_user(context: &Context, user: &User) -> Result<(), String> {
  let mut save_dir = PathBuf::from(&context.cache_path);
  if !save_dir.exists() || !save_dir.is_dir() {
    if fs::create_dir(&save_dir).is_err() {
      return Err(format!("Failed to create cache directory: {}", save_dir.to_string_lossy()).into());
    }
  }
  save_dir.push(&context.owner);
  if !save_dir.exists() || !save_dir.is_dir() {
    if fs::create_dir(&save_dir).is_err() {
      return Err(format!("Failed to create cache directory: {}", save_dir.to_string_lossy()).into());
    }
  }
  save_dir.push("user.json");
  match fs::write(&save_dir, serde_json::to_string(user).unwrap()) {
    Ok(()) => Ok(()),
    Err(err) => Err(format!("Failed to create user cache: {} : {}", save_dir.to_string_lossy(), err.to_string())),
  }
}

pub fn fetch_user(context: &Context) -> Result<User, String> {
  if context.force_use_cache {
    fetch_user_from_file(&context.owner, &context.cache_path)
  } else {
    match fetch_user_from_net(context) {
      Ok(user) => {
        save_user(context, &user)?;
        Ok(user)
      },
      Err(err) => Err(err),
    }
  }
}
