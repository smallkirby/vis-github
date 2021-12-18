use vis_github::github::{user::fetchUser, repo::fetchRepositories, commit::fetchCommits};
use vis_github::context::Context;

fn main() {
  let context: Context = Context{
    owner: "smallkirby".into(),
    cache_path: "./vis-cache".into(),
    force_use_cache: true,
  };

  let skb = fetchUser(&context);
  println!("{:?}", skb);
  let repos = fetchRepositories(&context);
  println!("{:?}", repos);
  let commits = fetchCommits(&context, "lysithea");
  println!("{:?}", commits);
}