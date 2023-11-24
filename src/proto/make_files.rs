
use std::{io::Error, collections::HashMap};

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
      let mut buffer = [0u8;2048];
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
  pub fn find_makefile_by_output_binary(&mut self, output_binary_name: &str)->Option<&mut MakeFile>{
    for item in &mut self.items{
      if item.output_binary == output_binary_name{
        return Some(item);
      }
    }
    None
  }

  pub fn process_dependants(&mut self){
    println!("===============================find dependencies=============================");
    
    let mut dependencies_map = HashMap::new();
    for makefile in &self.items{
      let current_binary = &makefile.output_binary; 
      if dependencies_map.get(current_binary).is_none(){
        dependencies_map.insert(makefile.output_binary.to_string(), Vec::new());
      }
      for dependency in makefile.dependencies.iter(){
        let dependant_opt = dependencies_map.get_mut(dependency);
        let mut dependants; 
        if dependant_opt.is_none(){
          dependants = Vec::new();
          dependants.push(makefile.clone());
          dependencies_map.insert(dependency.to_string(), dependants);
        }
        else{
          let dependant = dependant_opt.unwrap();
          dependant.push(makefile.clone());
        }
      }
    }
    if !dependencies_map.is_empty(){
      for output_binary in dependencies_map.keys(){
        let makefile_opt = self.find_makefile_by_output_binary(&output_binary);
        let mut make_file;
        if makefile_opt.is_none(){
          make_file = MakeFile::default();
          make_file.output_binary = output_binary.to_string();
          make_file.dependants = dependencies_map.get(output_binary).unwrap()
          .iter().map(|item| item.output_binary.to_string()).collect();
          self.items.push(make_file);
        }else{
          let makefile_in = makefile_opt.unwrap();
    
          let dependants  :Vec<String> = dependencies_map.get(output_binary).unwrap()
          .iter().map(|item| item.output_binary.to_string()).collect();
          makefile_in.dependants = dependants;
        }
      }
    }
  }
}
