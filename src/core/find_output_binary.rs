use std::{path::{Path, PathBuf}, io::Error, str::from_utf8};

use regex::Regex;
use tokio::{fs::File, io::{BufReader, AsyncReadExt}};

use crate::{constants::MAKEFILE_TEMPORARY_FOLDER_NAME, utils::common_functions::traversal_folder};

const OUTPUT_BINARY_VAR_REGEX: &str = r"(?m)\$\(OBJDIR\)\/\$\((?<var>\w+)\)";
const MATCH_FILE_IN_PATH_REGEX: &str = r"(?mi)\/(?<file_name>[\w\.]+)\s?[\\\n]?$";

struct MakeFile{
  path: PathBuf, 
  content: String, 
  output_binary: String,
  dependencies: Vec<String>
}

impl MakeFile {
  pub fn new(path: PathBuf, content: String) -> MakeFile{
    MakeFile{path, content, output_binary: String::new(), dependencies: Vec::new()}
  }
  pub async fn deserialize(file: File) -> Result<MakeFile, Error>{
    let mut file_buf_reader = BufReader::new (file);
    let path_size = file_buf_reader.read_u32().await?;
    let mut buffer = [0u8; 1024];
    file_buf_reader.read(&mut buffer[0..path_size as usize]).await?;
    let path: String = String::from_utf8(buffer[0..path_size as usize].to_vec()).unwrap();
    let mut content = String::new();

    buffer.fill(0);

    while let Ok(n) = file_buf_reader.read(&mut buffer).await {
      if n <= 0{
        break;
      }
      let content_chunk = from_utf8(&buffer[0..n]).unwrap();
      content.push_str(content_chunk);
    }
    Ok(MakeFile::new(path.into(), content))
    
  }
  pub fn set_path(&mut self, path: &str){
    self.path = Path::new(path).to_path_buf();  
  }
  pub fn get_path(&self)->&PathBuf{
    &self.path
  }
  pub fn get_content(&self)->&String{
    &self.content
  }
  pub fn set_output_binary(&mut self, output_binary: String){
    self.output_binary = output_binary;
  }
  pub fn set_dependecies(&mut self, dependencies: &mut Vec<String>){
    self.dependencies.append(dependencies);
  }
  pub fn get_dependencies(&self) -> &Vec<String>{
    &self.dependencies
  }

}

fn capure_output_binary(content: &String)->Option<String>{
  let output_regex = Regex::new(OUTPUT_BINARY_VAR_REGEX).unwrap();
  let mut captures = output_regex.captures_iter(content);
  if let Some(matcher) = captures.next(){
    if let Some(variable) = matcher.name("var") {
      println!("Variable: {}", variable.as_str());
      let value_var_regex = Regex::new(format!(r"(?mi){}=(?<binary>[\w\.\-]+)", variable.as_str()).as_str()).unwrap();
      let mut captures = value_var_regex.captures_iter(content);
      if let Some(matcher) = captures.next(){
        let output_binary = matcher.name("binary").unwrap().as_str();
        println!("output binary = {}", &output_binary);
        return Some(output_binary.to_string());
      }
    }
  }
  None
}


pub async fn find_output_binary()-> Result<(), String>{
  let temp_make_folder = Path::new(MAKEFILE_TEMPORARY_FOLDER_NAME);
  if !temp_make_folder.exists(){
    return Err("Makefile Temporary folder does not exist".into())
  }
  
  traversal_folder(temp_make_folder, |file| async {
    if let Ok(mut make_file) =  MakeFile::deserialize(file).await{
      if let Some(output_binary) = capure_output_binary(make_file.get_content()){
        make_file.set_output_binary(output_binary);
        let mut line_iter = make_file.get_content().lines().into_iter();
        let file_in_path_regex = Regex::new(MATCH_FILE_IN_PATH_REGEX).unwrap();
        let mut dependecies = Vec::new();
        while let Some(line) = line_iter.next(){
          if line.contains("SHARED_LIBS") || line.contains("STATIC_LIBS"){
            while let Some(line) = line_iter.next(){
              let mut captures = file_in_path_regex.captures_iter(&line);
              if let Some(capture) = captures.next(){
                let file_name = capture.name("file_name").unwrap();
                dependecies.push(file_name.as_str().to_string());
              }
              else {
                break;
              }
            }
          }
        }   
        if dependecies.len() > 0{
          make_file.set_dependecies(&mut dependecies);
        }
        for dependency in make_file.get_dependencies(){
          println!("dependencies: {}", dependency);
        }
      }
    }
  }).await;
  return Ok(())
}
