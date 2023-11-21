use std::{path::PathBuf, io::Error, str::from_utf8};

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
    let mut buffer = [0u8; 1024];
    file_buf_reader.read_exact(&mut buffer[0..path_size as usize]).await?;
    let path: String = String::from_utf8(buffer[0..path_size as usize].to_vec()).unwrap();
    let mut content = String::new();

    buffer.fill(0);

    while let Ok(n) = file_buf_reader.read(&mut buffer).await {
      if n == 0{
        break;
      }
      let content_chunk = from_utf8(&buffer[0..n]).unwrap();
      content.push_str(content_chunk);
    }
    Ok(MakeFile::new(path.into(), content))
    
  }
  pub fn get_content(&self)->&String{
    &self.content
  }
  pub fn set_output_binary(&mut self, output_binary: String){
    self.output_binary = output_binary;
  }
  pub fn set_dependencies(&mut self, dependencies: &mut Vec<String>){
    self.dependencies.append(dependencies);
  }

  pub fn get_dependencies(&self) -> &Vec<String>{
    &self.dependencies
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