use std::path::PathBuf;

pub struct TemplateFileHandler {
  repository_full_name: String,
  repository_default_branch: String,
}

impl TemplateFileHandler {
  pub fn new(
    repository_full_name: String,
    repository_default_branch: String,
  ) -> Self {
    Self {
      repository_full_name,
      repository_default_branch,
    }
  }

  pub fn download_template_repository(&self, output_path: &PathBuf) {
    let url = format!(
      "https://github.com/{}/archive/refs/heads/{}.zip",
      self.repository_full_name, self.repository_default_branch
    );
    let mut resp =
      reqwest::blocking::get(&url).expect("Failed to get template");
    let mut file =
      std::fs::File::create(output_path).expect("Failed to create file");
    std::io::copy(&mut resp, &mut file).expect("Failed to copy file");
  }

  pub fn get_base_structure_folder(
    &self,
    zip_path: &PathBuf,
    output_base_path: &PathBuf,
    base_structure_folder: String,
  ) -> PathBuf {
    let mut zip = zip::ZipArchive::new(
      std::fs::File::open(zip_path).expect("Failed to open zip file"),
    )
    .expect("Failed to open zip file");

    let folder_name = format!(
      "{}-{}",
      self.repository_full_name.split("/").last().unwrap(),
      self.repository_default_branch
    );

    let base_structure_folder = PathBuf::from(format!(
      "{}/{}/{}",
      output_base_path.as_os_str().to_str().unwrap(),
      folder_name,
      base_structure_folder
    ));

    zip
      .extract(output_base_path)
      .expect("Failed to extract zip file");

    base_structure_folder
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn download_template_repository() {
    let template_file_handler = TemplateFileHandler::new(
      "dtemplate/Hello-World".to_string(),
      "master".to_string(),
    );
    template_file_handler
      .download_template_repository(&PathBuf::from("./Hello-World.zip"));
  }

  #[test]
  fn get_base_structure_folder() {
    let template_file_handler = TemplateFileHandler::new(
      "dtemplate/Hello-World".to_string(),
      "master".to_string(),
    );
    let base_path = template_file_handler.get_base_structure_folder(
      &PathBuf::from("./Hello-World.zip"),
      &PathBuf::from("./"),
      "structure".to_string(),
    );
    assert_eq!(base_path, PathBuf::from("./Hello-World-master/structure"));
  }
}
