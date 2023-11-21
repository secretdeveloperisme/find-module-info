
use std::fmt::Display;


use self::model::MakeFile;


pub mod model;
pub mod make_files;


impl Display for MakeFile{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut result_str = String::new();
    result_str.push_str(format!("PATH: {}\n", self.path).as_str());
    result_str.push_str(format!("OUTPUT_BINARY: {}\n", self.output_binary).as_str());
    result_str.push_str(format!("DEPENDENCIES: {}", self.dependencies.join(", ")).as_str());
  
    write!(f,"{}", result_str)
  }
}