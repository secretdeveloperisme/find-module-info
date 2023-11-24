use std::{path::Path, sync::Arc};


use prost::{Message, bytes::BytesMut};
use regex::Regex;
use tokio::{fs::File, io::AsyncWriteExt, sync::Mutex};
use crate::{constants::{MAKEFILE_TEMPORARY_FOLDER_NAME, MAKEFILES_DB_FILE_NAME}, utils::common_functions::traversal_folder, proto};

use super::make_file::MakeFile;


const OUTPUT_BINARY_VAR_REGEX: &str = r"(?m)\$\(OBJDIR\)\/\$\((?<var>\w+)\)";
const MATCH_FILE_IN_PATH_REGEX: &str = r"(?mi)\/(?<file_name>[\w\.]+)\s{0,3}[\\\n]?$";


pub fn capure_output_binary(content: &String)->Option<String>{
  let output_regex = Regex::new(OUTPUT_BINARY_VAR_REGEX).unwrap();
  let mut captures = output_regex.captures_iter(content);
  if let Some(matcher) = captures.next(){
    if let Some(variable) = matcher.name("var") {
      let value_var_regex = Regex::new(format!(r"(?mi){}=(?<binary>[\w\.\-]+)", variable.as_str()).as_str()).unwrap();
      let mut captures = value_var_regex.captures_iter(content);
      if let Some(matcher) = captures.next(){
        let output_binary = matcher.name("binary").unwrap().as_str();
        return Some(output_binary.to_string());
      }
    }
  }
  None
}
pub fn caputre_dependencies(content: &String) -> Option<Vec<String>>{
  let mut line_iter = content.lines().into_iter();
  let file_in_path_regex = Regex::new(MATCH_FILE_IN_PATH_REGEX).unwrap();
  let mut dependecies = Vec::new();
  while let Some(mut line) = line_iter.next(){
    if line.contains("SHARED_LIBS") || line.contains("STATIC_LIBS"){
      loop {
        let mut captures = file_in_path_regex.captures_iter(&line);
        if let Some(capture) = captures.next(){
          let file_name = capture.name("file_name").unwrap();
          dependecies.push(file_name.as_str().to_string());
        }
        let line_opt  = line_iter.next();
        if line_opt.is_none() {
          break;
        }
        else{
          line = line_opt.unwrap();
        }
      }
    }
  }  
  if dependecies.is_empty() { None } else{ Some(dependecies) }
}

pub async fn find_output_binary()-> Result<(), String>{
  let temp_make_folder = Path::new(MAKEFILE_TEMPORARY_FOLDER_NAME);
  if !temp_make_folder.exists(){
    return Err("Makefile Temporary folder does not exist".into())
  }
  let makefiles_db_path = Path::new(MAKEFILES_DB_FILE_NAME);
  let makefiles_db_file;
  if !makefiles_db_path.exists(){
    makefiles_db_file = File::create(makefiles_db_path).await.unwrap();
  }else{
    makefiles_db_file = File::options().write(true).truncate(true).open(makefiles_db_path).await.unwrap();
  }
  
  let db_file_arc = Arc::new(Mutex::new(makefiles_db_file));
  traversal_folder(temp_make_folder, |file, db_file_arc: Arc<Mutex<File>> | async move{
    if let Ok(mut make_file) =  MakeFile::deserialize(file).await{
      if let Some(output_binary) = capure_output_binary(make_file.get_content()){
        println!("output binary = {}", &output_binary);
        make_file.set_output_binary(output_binary);
        let mut dependencies = caputre_dependencies(make_file.get_content()).unwrap_or(Vec::new());
        make_file.set_dependencies(&mut dependencies);
          let makefile_message: proto::model::MakeFile = MakeFile::into(make_file);
          let length: usize = makefile_message.encoded_len();  
          let mut buffer = BytesMut::with_capacity(length);
          if let Ok(_) = makefile_message.encode(&mut buffer){
            let mut db_file  = db_file_arc.lock().await;
            if let Ok(_) = db_file.write_u64(length as u64).await{
              let _ = db_file.write_all(&buffer[0..length]).await;
              println!("write the file to database: {}", makefile_message.path);
            }
          }
      }
    }
    // return Option::<model::MakeFile>::None;
  },db_file_arc).await;
  return Ok(())
}
