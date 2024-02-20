use super::*;

/// Main entry point into just binary.
#[allow(clippy::missing_errors_doc)]
pub fn run() -> Result<(), i32> {
  #[cfg(windows)]
  ansi_term::enable_ansi_support().ok();

  env_logger::Builder::from_env(
    env_logger::Env::new()
      .filter("JUST_LOG")
      .write_style("JUST_LOG_STYLE"),
  )
  .init();

  let command = Config::command();

  info!("Parsing command line arguments…");
  let mut matches = command.get_matches();

  let config = Config::from_matches(&mut matches).map_err(Error::from);

  let (color, verbosity) = config
    .as_ref()
    .map(|config| (config.color, config.verbosity))
    .unwrap_or((Color::auto(), Verbosity::default()));

  let loader = Loader::new();

  config
    .and_then(|config| config.run(&loader))
    .map_err(|error| {
      if !verbosity.quiet() && error.print_message() {
        eprintln!("{}", error.color_display(color.stderr()));
      }
      error.code().unwrap_or(EXIT_FAILURE)
    })
}
