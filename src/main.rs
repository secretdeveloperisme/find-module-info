pub use core::find_make_files;
use core::{find_make_files::{collect_makefiles, find_make_file_iter}, find_output_binary::find_output_binary};
use std::{env, path::Path};

use clap::Parser;
use command_paraser::{Arguments, FindOptions};
use constants::OUTPUT_DIR_NAME;
use tokio::fs::create_dir;
use utils::{result::{ProgramResult, Error as ProgramErr}, remove_children_items};
use proto::{make_files::Makefiles, store_makefiles};

mod command_paraser;
mod core;
mod utils;
mod constants;
mod proto;

#[tokio::main]
async fn main() {
  let args = Arguments::parse();
  let mut result = ProgramResult::Ok(());
  setup_output().await;
  match args.get_action() {
      "collect" =>{
        if let Err(e) = collect_makefiles(args.get_find_options()).await{
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
      "update" => {
        let source_path = args.get_find_options().get_source_path();
        let mut find_option = FindOptions::new();
        find_option.set_exclude_folder(vec!["thirdparty".to_string(), "pegasus".to_string(), "thdparty".to_string()]);
        println!("Update database for make files from {}", &source_path.to_string_lossy());
        if let Err(e) = collect_makefiles(args.get_find_options()).await{
          result = ProgramResult::Err(ProgramErr::new( e.to_string().as_str()));
        }else if let Err(e) = find_output_binary().await{
          result = ProgramResult::Err(ProgramErr::new(e.as_str()));
        } else if let Ok(mut makefiles) = Makefiles::read_from_db_file().await{
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
      "clean" => {
        env::set_current_dir("..").unwrap();
        if let Err(e) = remove_children_items(Path::new(OUTPUT_DIR_NAME)).await{
          result = ProgramResult::Err(ProgramErr::new( e.to_string().as_str()));
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
    println!("Runnning {action} program successfully", action=args.get_action());
  }
  
}

async fn setup_output(){
  if !Path::new(OUTPUT_DIR_NAME).exists(){
    create_dir(OUTPUT_DIR_NAME).await.unwrap();
  }
  env::set_current_dir(OUTPUT_DIR_NAME).unwrap();
}