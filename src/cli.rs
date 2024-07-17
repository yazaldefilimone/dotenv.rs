use clap::{arg, Command};

pub fn command_line() -> clap::ArgMatches {
  let matches = Command::new("dotenv.rs")
    .about("Rust-based, speedy, secure and versatile dotenv.")
    .author("Yazalde Filimone <yazaldefilimon@gmail.com>")
    .subcommand(
      Command::new("run")
        .about("run a script")
        .arg(arg!(<COMMAND> ... "the script to run")),
    )
    .get_matches();

  return matches;
}
