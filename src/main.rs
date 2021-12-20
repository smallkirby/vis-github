use vis_github::context::{Context, Command};
use vis_github::executer::*;

mod cli;

fn main() {
  let context = parse_args();

  match context.command {
    Command::RATE => show_ratelimit(&context),
    Command::FETCH => fetch_information(&context),
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
  } else if let Some(ref matches) = matches.subcommand_matches("fetch") {
    context.command = Command::FETCH;
    context.owner = matches.value_of("owner").unwrap().into();
    context.cache_path = matches.value_of("cache-dir").unwrap().into();
    context.force_use_cache = matches.value_of("cache").is_some();
  } else {
    context.command = Command::UNKNOWN;
  }

  context
}
