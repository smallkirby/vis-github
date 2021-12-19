use clap::{App, Arg, ArgGroup, SubCommand};

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
      SubCommand::with_name("debug")
        .about("nirugiri")
        .arg(
          Arg::with_name("owner")
          .short("O")
          .long("owner")
          .takes_value(true)
          .help("Target owner of Github")
          .required(true),
        )
        .arg(
          Arg::with_name("cache-dir")
            .short("C")
            .long("cache-dir")
            .takes_value(true)
            .help("Cache directory")
            .required(true),
        )
    ])
}