
/*
  This files impls commands directly executed by `main` function.

  These functions exits by theirselves when an error happens.
*/

use super::context::*;
use super::github::{ratelimit::*, user::*, repo::*};

use std::process;

// show rate-limit status
pub fn show_ratelimit(context: &Context) {
  match fetch_ratelimit(context) {
    Ok(ratelimit) => {
      let authed = if context.apitoken.is_some() { "yes" } else { "no" };
      println!("Authed    : {}", authed);
      println!("Remaining : {}", ratelimit.remaining);
      println!("Used      : {}", ratelimit.used);
      println!("Reset     : {}", ratelimit.date_reset());
    }
    Err(err) => {
      println!("[ERROR] {}", err);
      process::exit(1);
    },
  }
}

// fetch information using Github API.
// this function doesn't read cache even if it exists.
pub fn fetch_information(context: &Context) {
  if context.owner.is_empty() {
    println!("[ERROR] username not specified.");
    process::exit(1);
  }

  // fetch user information
  match fetch_user_from_net(context) {
    Ok(user) => {
      if let Err(err) = save_user(context, &user) {
        println!("{}", err);
        process::exit(1);
      }
      println!("Fetched user information of {} ({}).", user.login, user.name);
    }
    Err(err) => {
      println!("{}", err);
      process::exit(1);
    }
  };

  // fetch repositories
  match fetch_repositories(context) {
    Ok(repos) => {
      if let Err(err) = save_repos(context, &repos) {
        println!("{}", err);
        process::exit(1);
      }
      println!("Fetched {} repos.", repos.len());
    }
    Err(err) => {
      println!("{}", err);
      process::exit(1);
    }
  }
}
