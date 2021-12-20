use vis_github::github::repo::fetch_repositories;
use vis_github::github::ratelimit::show_ratelimit;
use vis_github::context::{Context, Command};

mod cli;

fn main() {
  let context = parse_args();

  match context.command {
    Command::RATE => show_ratelimit(&context),
    Command::DEBUG => { // XXX
      let skb = fetch_repositories(&context);
      println!("{:?}", skb);
    },
    Command::UNKNOWN => unimplemented!(),
  }
}

pub fn parse_args() -> Context {
  let matches = cli::build_cli().get_matches();
  let mut context = Context::default();

  if let Some(matches) = matches.value_of("token") {
    context.apitoken = Some(matches.into());
  }

  if let Some(ref _matches) = matches.subcommand_matches("rate") {
    context.command = Command::RATE;
  } else if let Some(ref matches) = matches.subcommand_matches("debug") {
    context.command = Command::DEBUG;
    context.owner = matches.value_of("owner").unwrap().into();
    context.cache_path = matches.value_of("cache-dir").unwrap().into();
  } else {
    context.command = Command::UNKNOWN;
  }

  context
}
