use vis_github::github::user::fetch_user;
use vis_github::context::Context;

fn main() {
  let context: Context = Context{
    owner: "smallkirby".into(),
    cache_path: "./vis-cache".into(),
    force_use_cache: true,
    apitoken: None,
  };

  let skb = fetch_user(&context);
  println!("{:?}", skb);
}