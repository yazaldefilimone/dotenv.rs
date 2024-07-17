mod cli;

fn main() {
  let matches = cli::command_line();

  match matches.subcommand() {
    Some(("run", sub_matches)) => {
      let script = sub_matches.get_one::<String>("script").unwrap();
      run_script(script);
    }
    _ => {}
  }
}

fn run_script(script: &str) {
  let mut script_path = std::env::current_dir().unwrap();
  script_path.push(script);
  let script_path_str = script_path.to_str().unwrap();
  println!("script_path_str: {}", script_path_str);
}
