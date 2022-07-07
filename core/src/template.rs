use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateHandler<'a> {
  dt_base_api_url: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct GithubRepository {
  pub full_name: String,
  pub default_branch: String,
  pub id: u32,
  pub url: String,
  #[serde(rename = "treeUrl")]
  pub tree_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateConfiguration {
  pub name: String,
  pub description: String,
  pub commands: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
  #[serde(rename = "_id")]
  pub id: String,
  pub github_repository: GithubRepository,
  pub base_template_repository_tree_url: String,
  pub configuration_file_tree_url: String,
  pub template_configuration: TemplateConfiguration,
  pub user_own_id: String,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateResponse {
  pub result: Template,
}

impl<'a> TemplateHandler<'a> {
  pub fn new() -> Self {
    Self {
      dt_base_api_url: "https://dtemplate.org/api",
    }
  }

  pub fn get_template_by_name(&self, name: String) -> Template {
    let url = format!("{}/templates/{}", self.dt_base_api_url, name);
    let text = reqwest::blocking::get(&url)
      .expect("Failed to get template")
      .text()
      .expect("Error to get template data");
    let template_response: TemplateResponse =
      serde_json::from_str(&text).expect("Error to parse template data");
    template_response.result
  }
}

impl<'a> Default for TemplateHandler<'a> {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  pub fn get_template_by_name() {
    let template_handler = TemplateHandler::new();
    let template =
      template_handler.get_template_by_name(String::from("Hello-World"));
    assert_eq!(template.id, "62c0e2be370a7647e09158a4");
  }
}
