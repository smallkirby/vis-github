/*
 * This file defines subcommands of `vis-github` and their command-line arguments.
 */

use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
  App::new("vis-github")
    .version(env!("CARGO_PKG_VERSION"))
    .author("(c) 2021 smallkirby")
    .subcommands(vec![
      SubCommand::with_name("rate")
        .about("Show ratelimit for Github API")
        .arg(
          Arg::with_name("owner")
          .short("O")
          .long("owner")
          .takes_value(true)
          .help("Target owner of Github")
        ),
      SubCommand::with_name("fetch")
        .about("Fetch user and repos information for specified owner.")
        .arg(
          Arg::with_name("owner")
          .short("O")
          .long("owner")
          .takes_value(true)
          .required(true)
          .help("Target owner of Github")
        ),
      SubCommand::with_name("vis")
        .about("Visualize Github history.")
        .arg(
          Arg::with_name("owner")
          .short("O")
          .long("owner")
          .takes_value(true)
          .required(true)
          .help("Target owner of Github")
        )
    ])
    .arg(
      Arg::with_name("token")
      .short("t")
      .long("token")
      .takes_value(true)
      .help("Your API token of Github")
      .required(false),
    )
    .arg(
      Arg::with_name("cache-dir")
        .short("C")
        .long("cache-dir")
        .takes_value(true)
        .help("Cache directory")
        .required(false),
    )
}