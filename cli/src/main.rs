use core_dt::TemplateHandler;

fn main() {
  let template_handler = TemplateHandler::new();
  let template =
    template_handler.get_template_by_name(String::from("Hello-World"));
  println!("{}", template.template_configuration.name);
}
