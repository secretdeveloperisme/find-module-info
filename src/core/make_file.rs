use std::{path::PathBuf, io::Error};

use tokio::{fs::File, io::{BufReader, AsyncReadExt}};

use crate::proto;

pub struct MakeFile{
  path: PathBuf, 
  content: String, 
  output_binary: String,
  dependencies: Vec<String>
}

#[allow(unused)]
impl MakeFile{
  pub fn new(path: PathBuf, content: String) -> MakeFile{
    MakeFile{path, content, output_binary: String::new(), dependencies: Vec::new()}
  }
  pub async fn deserialize(file: File) -> Result<MakeFile, Error>{
    let mut file_buf_reader = BufReader::new (file);
    let path_size = file_buf_reader.read_u32().await?;
    let mut path_buffer = [0u8; 255];
    file_buf_reader.read_exact(&mut path_buffer[0..path_size as usize]).await?;
    let path  = String::from_utf8_lossy(&path_buffer[0..path_size as usize]);
    let mut content = String::new();

    let mut content_buffer = [0u8;2048];
    content_buffer.fill(0);

    while let Ok(n) = file_buf_reader.read(&mut content_buffer).await {
      if n == 0{
        break;
      }
      let content_chunk = String::from_utf8_lossy(&content_buffer[0..n]);
      content.push_str(content_chunk.to_string().as_str());
      content_buffer.fill(0);
    }
    Ok(MakeFile::new(path.to_string().into(), content))
    
  }
  pub fn get_content(&self)->&String{
    &self.content
  }
  pub fn set_output_binary(&mut self, output_binary: String){
    self.output_binary = output_binary;
  }
  pub fn set_dependencies(&mut self, dependencies: &Vec<String>){
    let mut new_dependencies = dependencies.clone();
    self.dependencies.append(&mut new_dependencies);
  }

  pub fn get_dependencies(&self) -> &Vec<String>{
    &self.dependencies
  }
  pub fn get_path(&self) -> &PathBuf{
    &self.path
  }

}
impl Into<proto::model::MakeFile> for MakeFile {
  fn into(self) -> proto::model::MakeFile {
    let mut message = proto::model::MakeFile::default();
    message.path = self.path.to_str().unwrap().to_string();
    message.output_binary = self.output_binary;
    message.dependencies = self.dependencies;
    message
  }
}