use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateHandler {
  dt_base_api_url: &'static str,
  pub template_name: String,
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
pub struct TemplateBaseStructure {
  pub name: String,
  pub children: Vec<TemplateBaseStructure>,
  pub content: String,
  #[serde(rename = "type")]
  pub entety_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateBaseStructureResponse {
  pub result: Vec<TemplateBaseStructure>,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateResponse {
  pub result: Template,
}

impl TemplateHandler {
  pub fn new(template_name: String) -> Self {
    Self {
      dt_base_api_url: "https://dtemplate.org/api",
      template_name,
    }
  }

  pub fn get_by_name(&self) -> Template {
    let url =
      format!("{}/templates/{}", self.dt_base_api_url, self.template_name);
    let text = reqwest::blocking::get(&url)
      .expect("Failed to get template")
      .text()
      .expect("Error to get template data");
    let template_response: TemplateResponse =
      serde_json::from_str(&text).expect("Error to parse template data");
    template_response.result
  }

  pub fn download_base_structure(&self, output_path: PathBuf) {
    if !output_path.exists() {
      fs::create_dir_all(output_path.clone())
        .expect("Error to create output path");
    }
    let base_structure = self.request_base_structure();
    self.save_file(output_path, base_structure);
  }

  fn save_file(
    &self,
    output_base_path: PathBuf,
    structures: Vec<TemplateBaseStructure>,
  ) {
    for structure in structures {
      let output_path = output_base_path.join(structure.name);

      if structure.entety_type == "blob" {
        let content_bytes = base64::decode(structure.content).unwrap();
        let mut file = File::create(output_path).unwrap();
        file.write_all(&content_bytes).expect("Error to write file");
      } else if structure.entety_type == "tree" {
        fs::create_dir_all(&output_path).expect("Error to create directory");
        self.save_file(output_path, structure.children);
      }
    }
  }

  fn request_base_structure(&self) -> Vec<TemplateBaseStructure> {
    let url = format!(
      "{}/templates/{}/code?base=true",
      self.dt_base_api_url, self.template_name
    );
    let text = reqwest::blocking::get(&url)
      .expect("Failed to get template")
      .text()
      .expect("Error to get template data");
    let template_response: TemplateBaseStructureResponse =
      serde_json::from_str(&text).expect("Error to parse template data");

    template_response.result
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn get_by_name() {
    let template_handler = TemplateHandler::new("Hello-World".to_string());
    let template = template_handler.get_by_name();
    assert_eq!(template.id, "62c767ba1b961bc697a7ce58");
  }

  #[test]
  fn download_base_structure() {
    let template_handler = TemplateHandler::new("Hello-World".to_string());
    let output_path = PathBuf::from("Hello_World");
    template_handler.download_base_structure(output_path);
  }
}
