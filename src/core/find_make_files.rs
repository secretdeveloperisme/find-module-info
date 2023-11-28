
use std::{path::Path, io::{Error, self}};
use async_recursion::async_recursion;
use bytes::BytesMut;
use prost::Message;
use tokio::{fs::{File, self, create_dir}, io::{AsyncWriteExt, AsyncReadExt}};
use crate::{command_paraser::FindOptions, constants::{MAKEFILE_TEMPORARY_FOLDER_NAME, MAKEFILE_NAME_LOWER_CASE, MAKEFILES_DB_FILE_NAME}};
use crate::{utils::common_functions::path_to_name, proto::model::MakeFile};

pub async fn collect_makefiles(find_options: &FindOptions)->Result<(), Box<dyn std::error::Error>>{
  if !(find_options.get_source_path().is_dir()) {
    return Err(Error::new(std::io::ErrorKind::NotFound, "source path is not valid").into());
  }
  let path = find_options.get_source_path();
  let makefile_folder_path = Path::new(MAKEFILE_TEMPORARY_FOLDER_NAME);
  if !makefile_folder_path.exists(){
    create_dir(makefile_folder_path).await?;
  }
  handle_file_recursive(&path, &find_options , &makefile_folder_path).await;
  Ok(())
}

#[async_recursion]
async fn handle_file_recursive(path: &Path, find_option: &FindOptions, makefile_folder_path: &Path){

  let target_name = path.file_name().unwrap().to_str().unwrap();
  if !find_option.is_hidden(){
    if target_name.starts_with("."){
      println!("Ignore: {}", path.to_str().unwrap());
      return;
    }
  }

  if path.is_file() {
    if let Some(os_str) = path.file_name() {
      if os_str.eq_ignore_ascii_case(MAKEFILE_NAME_LOWER_CASE){
        if let Ok(mut file) = File::open(&path).await{
          println!("Match file: {}", path.to_str().unwrap());
          let _ = process_makefile(&mut file, path, makefile_folder_path).await;
        }
      }
    }
  }else
  {
    let folder_name = path.file_name().unwrap().to_str().unwrap();
    if !find_option.get_exclude_folder().is_empty(){
      for exclude_folder_name in find_option.get_exclude_folder().iter(){
        if exclude_folder_name.eq_ignore_ascii_case(folder_name){
          return;
        }
      }
    }
    
    if let Ok(mut dir_entry) = fs::read_dir(path).await{
      while let Ok(Some(dir)) = dir_entry.next_entry().await{
        let dir_path = Path::join(&path, dir.path());
        handle_file_recursive(&dir_path, find_option, makefile_folder_path).await;
      }
    }
    
  }
}

async fn process_makefile(source_file: &mut File, path: &Path, makefile_folder_path: &Path)-> Result<(), Box<dyn std::error::Error>>{
  let _ = source_file.metadata().await?;
  let file_name = path_to_name(&path);
  let mut dest_file: File =  File::create(Path::join(makefile_folder_path, &file_name)).await?;
  let mut buf = [0u8;256];
  let path_bytes = path.to_str().unwrap().as_bytes();
  buf[0..path_bytes.len()].copy_from_slice(path_bytes);
  dest_file.write_u32(path_bytes.len() as u32).await?;
  dest_file.write(&mut buf[0..=path_bytes.len()]).await?;
  let mut buf_1 = [0u8;1024];
  while let Ok(n) =  source_file.read(&mut buf_1).await{
    if n <= 0{
      break;
    }
    dest_file.write(&buf_1[0..n]).await?;
    
  }
  Ok(())
}

pub async fn find_make_file_iter(output : &str)->Result<Vec<MakeFile>, io::Error>{
  let mut makefiles =  Vec::new();
  let mut makefile_db_file = File::open(MAKEFILES_DB_FILE_NAME).await?;
  while let Ok(message_size) = makefile_db_file.read_u64().await{
    let mut byte_buf = BytesMut::with_capacity(message_size as usize);
    if let Ok (n) = makefile_db_file.read_buf(&mut byte_buf).await{
      if n > 0{
        if let Ok(makefile) = MakeFile::decode(byte_buf){
          if makefile.output_binary.contains(output){
            makefiles.push(makefile);
          }
        }
      }
    }
  }
  Ok(makefiles)
}