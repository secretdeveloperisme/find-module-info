use std::{path::Path, io};

use tokio::fs::{read_dir, remove_file, remove_dir_all};

pub mod common_functions;
pub mod result;


pub async fn remove_children_items(path : &Path) ->Result<(), io::Error>{
  let mut read_dir = read_dir(path).await?;
  while let Ok(Some(entry)) = read_dir.next_entry().await{
    let file_type = entry.file_type().await?;
    if file_type.is_dir(){
      remove_dir_all(&entry.path()).await?;
    }else if file_type.is_file() {
      remove_file(&entry.path()).await?;
    }
  }
  Ok(())
}