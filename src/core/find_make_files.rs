
use std::{path::Path, io::Error};
use async_recursion::async_recursion;
use tokio::{fs::{File, self, create_dir}, io::{AsyncWriteExt, AsyncReadExt}};
use crate::{command_paraser::Args, constants::{MAKEFILE_TEMPORARY_FOLDER_NAME, MAKEFILE_NAME_LOWER_CASE}, utils::common_functions::path_to_name};



pub async fn find_makefiles(args: Args)->Result<(), Box<dyn std::error::Error>>{
  if !(args.get_source_path().is_dir()) {
    return Err(Error::new(std::io::ErrorKind::NotFound, "source path is not valid").into());
  }
  let path = args.get_source_path();
  let makefile_folder_path = Path::new(MAKEFILE_TEMPORARY_FOLDER_NAME);
  if !makefile_folder_path.exists(){
    create_dir(makefile_folder_path).await?;
  }
  handle_file_recursive(&path, &args , &makefile_folder_path).await;
  Ok(())
}

#[async_recursion]
async fn handle_file_recursive(path: &Path, args: &Args, makefile_folder_path: &Path){
  println!("handle file recursive: {}",path.to_string_lossy());
  if path.is_file() {
    if let Some(os_str) = path.file_name() {
      println!("os_string: {}",os_str.to_ascii_lowercase().to_str().unwrap());
      if os_str.eq_ignore_ascii_case(MAKEFILE_NAME_LOWER_CASE){
        if let Ok(mut file) = File::open(&path).await{
          let _ = process_makefile(&mut file, path, makefile_folder_path).await;
        }
       
      }
    }
  }else{
    if let Ok(mut dir_entry) = fs::read_dir(path).await{
      while let Ok(Some(dir)) = dir_entry.next_entry().await{
        let dir_path = Path::join(&path, dir.path());
        handle_file_recursive(&dir_path, args, makefile_folder_path).await;
      }
    }
  }
}

async fn process_makefile(source_file: &mut File, path: &Path, makefile_folder_path: &Path)-> Result<(), Box<dyn std::error::Error>>{
  println!("process file: {}", path.to_str().unwrap());
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
  println!("After process file");
  Ok(())
}