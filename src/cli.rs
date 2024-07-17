use clap::{Arg, Command};

pub fn command_line() -> clap::ArgMatches {
  let matches = Command::new("dotenv.rs")
    .about("Rust-based, speedy, secure and versatile dotenv.")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .author("Yazalde Filimone <yazaldefilimon@gmail.com>")
    .subcommand(
      Command::new("run")
        .about("run a script")
        .arg(Arg::new("script").help("The script to run").required(true).index(1)),
    )
    .get_matches();

  return matches;
}
