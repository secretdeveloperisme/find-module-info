use std::{path::Path, future::Future};

use regex::Regex;
use tokio::fs::{File, read_dir};

pub fn path_to_name(path: &Path) -> String{
  let path_separator = Regex::new(r"[\\/]").unwrap();
  let mut path_str: String = path.to_str().unwrap().into();

  
  if path_separator.is_match(path_str.get(0..1).unwrap()){
    path_str.remove(0);
  }
  
  path_str = path_str.replace(":", "");
  let output = path_separator.replace_all(&path_str, "_");
  output.to_string()
}

pub async fn traversal_folder<H>(folder_path: &Path, f: impl Fn(File) -> H) 
where H: Future<Output = ()>
{
  if folder_path.exists() && folder_path.is_dir(){
    if let Ok(mut dir_entry) = read_dir(folder_path).await {
      while let Ok(Some(dir_entry)) = dir_entry.next_entry().await {
        let dir_path = dir_entry.path();
        if dir_path.is_file(){
          if let Ok(file) = File::open(&dir_path).await{
            println!("process file: {}", dir_path.to_string_lossy());
            f(file).await;
          }
        }
      }
    }
  }
  
}