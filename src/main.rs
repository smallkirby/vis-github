use vis_github::github::repo::fetch_repositories;
use vis_github::context::Context;

fn main() {
  let context: Context = Context{
    owner: "smallkirby".into(),
    cache_path: "./vis-cache".into(),
    force_use_cache: true,
    apitoken: None,
    ignore_fork: true,
    ignore_private: true,
    commit_limit_per_repo: 999,
  };

  let skb = fetch_repositories(&context);
  println!("{:?}", skb);
}