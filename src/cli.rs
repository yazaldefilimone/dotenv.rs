use clap::{arg, Arg, Command};

pub fn command_line() -> clap::ArgMatches {
  let matches = Command::new("dotenv.rs")
    .about("An efficient, secure, shareable, cross-platform dotenv.")
    .author("Yazalde Filimone <yazaldefilimon@gmail.com>")
    .subcommand(
      Command::new("set")
        .about("Set and encrypt one or more variables")
        .arg_required_else_help(true)
        .arg(arg!(<VARIABLE> ... "The key-value pairs of the variables in the format KEY VALUE").required(true))
        .arg(
          Arg::new("file")
            .short('f')
            .long("file")
            .help("The .env file to update")
            .required(false),
        ),
    )
    .subcommand(Command::new("generate-keys").about("Generate a new pair of keys"))
    .subcommand(
      Command::new("run")
        .about("Run a command with variables loaded from .env file or automatically from the current directory")
        .arg(
          Arg::new("FILE")
            .short('f')
            .long("file")
            .help("The .env file to load")
            .required(false),
        )
        .arg(Arg::new("COMMAND").help("The command to run").required(true)),
    )
    .subcommand(
      Command::new("encrypt")
        .about("Generate a production encryption envs file")
        .arg(
          Arg::new("FILE")
            .short('f')
            .long("file")
            .help("The .env file to load")
            .required(false),
        )
        .arg(
          Arg::new("PUBLIC_KEY")
            .short('p')
            .long("public-key")
            .help("The public key to use for encryption")
            .required(false),
        ),
    )
    .get_matches();

  return matches;
}
