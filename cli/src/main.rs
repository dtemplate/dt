use clap::{App, Arg, SubCommand};
use core_dt::TemplateHandler;
use indicatif::ProgressBar;
use std::env;

fn main() {
  let matches = App::new("dt")
    .version("0.1.0")
    .author("Theryston Santos")
    .about("Build your entire project with one command 🚀")
    .subcommand(
      SubCommand::with_name("new")
        .about("create a new entity based on an input, the inputs can be templates for example")
          .arg(
            Arg::with_name("template_name")
            .short('t')
            .long("template")
            .takes_value(true)
            .value_name("template_name")
            .help("Generate files/folders from a template")
          )
        )
        .get_matches();

  match matches.subcommand() {
    Some(("new", sub_matches)) => {
      let template_name = sub_matches
        .value_of("template_name")
        .expect("No template name")
        .to_string();

      use_template(template_name);
    }
    _ => unreachable!(),
  }
}

fn use_template(template_name: String) {
  println!();
  let current_dir = env::current_dir().unwrap();
  let template_handler = TemplateHandler::new(template_name);
  let get_file_progress_bar = ProgressBar::new(100);
  get_file_progress_bar.set_message("🚀  Generating files from template...");
  get_file_progress_bar.set_style(
    indicatif::ProgressStyle::default_spinner()
      .template("{spinner:.green} {msg}"),
  );
  get_file_progress_bar.enable_steady_tick(80);
  template_handler.download_base_structure(current_dir.clone());
  get_file_progress_bar.finish();

  println!("  🚀  Running template commands...");
  println!();
  template_handler.run_commands(current_dir);
}
