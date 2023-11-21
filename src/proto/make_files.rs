
use std::io::Error;

use bytes::BytesMut;
use prost::Message;
use tokio::{fs::File, io::AsyncReadExt};

use crate::{proto::model::MakeFile, constants::MAKEFILES_DB_FILE_NAME};

pub struct Makefiles{
  pub items: Vec<MakeFile>
}

impl Makefiles{
  pub fn new()->Makefiles{
    Makefiles { items: Vec::new()}
  }
  pub async fn read_from_db_file(&mut self)->Result<(), Error>{
    let mut makefile_db_file = File::open(MAKEFILES_DB_FILE_NAME).await?;
    while let Ok(message_size) = makefile_db_file.read_u64().await{
      let mut buffer = [0u8;1024];
      if let Ok (n) = makefile_db_file.read(&mut buffer[0..message_size as usize]).await{
        if n > 0{
          let bytes_buffer = BytesMut::from(&buffer[0..message_size as usize]);
          if let Ok(makefile) = MakeFile::decode(bytes_buffer){
            self.items.push(makefile);
          }
        }
      }
    }
    Ok(())
  }
  pub fn get_makefiles(&self)->&Vec<MakeFile>{
    &self.items
  }
}
