pub use core::find_make_files;
use core::{find_make_files::{collect_makefiles, find_make_file_iter}, find_output_binary::find_output_binary};
use std::{env, path::Path};

use clap::Parser;
use command_paraser::Args;
use constants::OUTPUT_DIR_NAME;
use tokio::fs::create_dir;
use utils::result::{ProgramResult, Error as ProgramErr};
use proto::{make_files::Makefiles, store_makefiles};

mod command_paraser;
mod core;
mod utils;
mod constants;
mod proto;

#[tokio::main]
async fn main() {
  let args = Args::parse();
  let mut result = ProgramResult::Ok(());
  setup_output().await;
  match args.get_action() {
      "collect" =>{
        if let Err(e) = collect_makefiles(args).await{
          result = ProgramResult::Err(ProgramErr::new( e.to_string().as_str()));
        }
      },
      "binary" =>{
        if let Err(e) = find_output_binary().await{
          result = ProgramResult::Err(ProgramErr::new(e.as_str()));
        }
      }
      "depend" => {
        if let Ok(mut makefiles) = Makefiles::read_from_db_file().await{
          makefiles.process_dependants();
          store_makefiles(&makefiles).await;
        }else{
          result = ProgramResult::Err(ProgramErr::new("Cannot read makefiles"));
        }
      }
      "find" => {
        if let Ok(makefiles) = find_make_file_iter(args.get_module_name()).await{
          if makefiles.is_empty(){
            println!("Not found binary infomation");
          }else{
            println!("Found Dependencies: ");
            for makefile in makefiles{
              println!("{}", makefile);
            }
          }
          
        }else{
          result = ProgramResult::Err(ProgramErr::new("Cannot find make file"));
        }
      }
      _ => {
        result = ProgramResult::Err(ProgramErr::new("No such action"));
      },
  }
  
  if !result.is_ok(){
    println!("{}", result.get_err().get_message());
  }
  else {
    println!("Runnning program successfully");
  }
}

async fn setup_output(){
  if !Path::new(OUTPUT_DIR_NAME).exists(){
    create_dir(OUTPUT_DIR_NAME).await.unwrap();
  }
  env::set_current_dir(OUTPUT_DIR_NAME).unwrap();
}