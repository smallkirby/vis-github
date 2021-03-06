use vis_github::context::{Command, Context, VisualizeType};
use vis_github::executer::*;

mod cli;

fn main() {
  let context = parse_args();

  match context.command {
    Command::RATE => show_ratelimit(&context),
    Command::FETCH => fetch_information(&context),
    Command::VIS => visualize(&context),
    Command::UNKNOWN => unimplemented!(),
  }
}

pub fn parse_args() -> Context {
  let matches = cli::build_cli().get_matches();
  let mut context = Context::default();

  if let Some(matches) = matches.value_of("token") {
    context.apitoken = Some(matches.into());
  }
  if let Some(matches) = matches.value_of("cache-dir") {
    context.cache_path = matches.into();
  }

  if let Some(ref _matches) = matches.subcommand_matches("rate") {
    context.command = Command::RATE;
  } else if let Some(ref matches) = matches.subcommand_matches("fetch") {
    context.command = Command::FETCH;
    context.owner = matches.value_of("owner").unwrap().into();
    context.force_use_cache = matches.value_of("cache").is_some();
  } else if let Some(ref matches) = matches.subcommand_matches("vis") {
    context.command = Command::VIS;
    context.owner = matches.value_of("owner").unwrap().into();
    context.vis_type = if let Some(method) = matches.value_of("by") {
      match method {
        "time" => VisualizeType::TIME,
        "license" => VisualizeType::LICENSE,
        "language" => VisualizeType::LANGUAGE,
        _ => VisualizeType::UNKNOWN,
      }
    } else {
      VisualizeType::TIME
    };
  } else {
    context.command = Command::UNKNOWN;
  }

  context
}
