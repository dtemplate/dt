pub struct TemplateHandlerOptions {
    pub name: String,
}

pub struct TemplateHandler {
    pub name: String,
}

impl TemplateHandler {
    pub fn new(options: TemplateHandlerOptions) -> Self {
        let template_handler = Self {
            name: options.name.clone(),
        };
        template_handler
    }

    pub fn get_template_by_name(&self, name: String) -> String {
        String::from(name)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn get_template_by_name() {
        let template_handler = TemplateHandler::new(TemplateHandlerOptions {
            name: String::from("test"),
        });
        let template = template_handler.get_template_by_name(String::from("test"));
        assert_eq!(template, String::from("test"));
    }
}
