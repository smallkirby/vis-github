
/*
  This files impls commands directly executed by `main` function.

  These functions exits by theirselves when an error happens.
*/

use super::context::*;
use super::github::{ratelimit::*, user::*, repo::*, commit::*};
use super::analyzer::executer::*;
use super::visualizer::*;

use std::process;
use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressStyle};

static EMOJI_ROBOT: Emoji<'_, '_> = Emoji("🤖", "");
static EMOJI_COMPUTER: Emoji<'_, '_> = Emoji("💻", "");
static EMOJI_BOOKS: Emoji<'_, '_> = Emoji("📚", "");

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
  let mut fetched_repos = vec!();

  if context.owner.is_empty() {
    println!("[ERROR] username not specified.");
    process::exit(1);
  }

  // fetch user information
  println!("{} {} Fetching user information...", style("[1/3]").bold().dim(), EMOJI_ROBOT);
  match fetch_user_from_net(context) {
    Ok(user) => {
      if let Err(err) = save_user(context, &user) {
        println!("{}", err);
        process::exit(1);
      }
    }
    Err(err) => {
      println!("{}", err);
      process::exit(1);
    }
  };

  // fetch repositories
  println!("{} {} Fetching repositories overview...", style("[2/3]").bold().dim(), EMOJI_COMPUTER);
  match fetch_repositories(context) {
    Ok(repos) => {
      if let Err(err) = save_repos(context, &repos) {
        println!("{}", err);
        process::exit(1);
      }
      fetched_repos = repos;
    }
    Err(err) => {
      println!("{}", err);
      process::exit(1);
    }
  }

  // fetch commits of each repos
  println!("{} {} Fetching commit data for {} repos, which may takes several time...",
    style("[3/3]").bold().dim(),
    EMOJI_BOOKS,
    fetched_repos.len());
  let style = ProgressStyle::default_bar()
      .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
      .progress_chars("##-");
  let progress = ProgressBar::new(fetched_repos.len() as u64);
  progress.set_style(style);

  for repo in fetched_repos {
    progress.set_message(repo.name.clone());

    if !repo.is_target(context) {
      progress.inc(1);
      continue;
    }
    match fetch_commits_from_net(context, &repo.name) {
      Ok(commits) => if let Err(err) = save_commits(context, &repo.name, &commits) {
        println!("{}", err);
        process::exit(1);
      }
      Err(err) => {
        println!("{}", err);
        process::exit(1);
      }
    }

    progress.inc(1);
    progress.abandon_with_message("Complete");
  }
}

pub fn visualize(context: &Context) {
  let time_map = match analyze_by_time(context) {
    Ok(map) => map,
    Err(err) => {
      println!("{}", err);
      process::exit(1);
    }
  };

  visualize_by_time(context, time_map);
}
