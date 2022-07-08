use core_dt::TemplateHandler;
use std::path::PathBuf;

fn main() {
  let template_handler = TemplateHandler::new("Hello-World".to_string());
  let output_path = PathBuf::from("Hello_World");
  template_handler.download_base_structure(output_path.clone());
  template_handler.run_commands(output_path);
}
