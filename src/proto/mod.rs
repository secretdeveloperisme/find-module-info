
use std::{fmt::Display, io, mem::size_of, path::Path};

use bytes::{BytesMut, BufMut};
use prost::Message;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{core::Serializable, constants::MAKEFILES_DB_FILE_NAME};

use self::{model::MakeFile, make_files::Makefiles};


pub mod model;
pub mod make_files;

impl Display for MakeFile{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut result_str = String::new();
    result_str.push_str(format!("{line}{out}{line}\n",line="=".repeat(20),out=self.output_binary).as_str());
    result_str.push_str(format!("PATH: {}\n", self.path).as_str());
    result_str.push_str(format!("OUTPUT_BINARY: {}\n", self.output_binary).as_str());
    if !self.dependencies.is_empty(){
      result_str.push_str(format!("DEPENDENCIES:\n\t{}\n", self.dependencies.join("\n\t")).as_str());
    }
    if !self.dependants.is_empty(){
      result_str.push_str(format!("DEPENDANTS:\n\t{}", self.dependants.join("\n\t")).as_str());
    }

    write!(f,"{}", result_str)
  }
}

impl Serializable for MakeFile {
  fn serialize(&self)-> Result<BytesMut, io::Error>{
    let length = self.encoded_len();
    let len_of_u64 = size_of::<u64>();
    let mut buffer_make_file = BytesMut::with_capacity(length);
    let mut total_buffer = BytesMut::with_capacity(len_of_u64 + length);
    total_buffer.put_u64(length as u64);
    self.encode(&mut buffer_make_file)?;
    total_buffer.put(&buffer_make_file[0..length as usize]);
    Ok(total_buffer)
  }
}

pub async fn store_makefiles(makefiles: &Makefiles){
  let makefiles_db_path = Path::new(MAKEFILES_DB_FILE_NAME);
  let mut makefiles_db_file;
  if !makefiles_db_path.exists(){
    makefiles_db_file = File::create(makefiles_db_path).await.unwrap();
  }else{
    makefiles_db_file = File::options().write(true).truncate(true).open(MAKEFILES_DB_FILE_NAME).await.unwrap();
  }
  for makefile in makefiles.get_makefiles(){
    let bytes_buf = makefile.serialize();
    if let Ok(bytes_buf) = bytes_buf{
      let _ = makefiles_db_file.write(&bytes_buf).await;
    }else{
      println!("error: when serialize");
    }
  }
}

