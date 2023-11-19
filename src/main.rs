pub use core::find_make_files;
use core::{find_make_files::find_makefiles, find_output_binary::find_output_binary};

use clap::Parser;
use command_paraser::Args;
use find_module_info::utils::{ProgramResult, Error as ProgramErr};

mod command_paraser;
mod core;
mod utils;
mod constants;
#[tokio::main]
async fn main() {
  let args = Args::parse();
  let mut result = ProgramResult::Ok(());
  
  match args.get_action() {
      "makefile" =>{
        if let Err(e) = find_makefiles(args).await{
          result = ProgramResult::Err(ProgramErr::new( e.to_string().as_str()));
        }
      },
      "binary" =>{
        if let Err(e) = find_output_binary().await{
          result = ProgramResult::Err(ProgramErr::new(e.as_str()));
        }
      }
      _ => {
        result = ProgramResult::Err(ProgramErr::new("An error occurs when running program"));
      },
  }
  
  if !result.is_ok(){
    println!("{}", result.get_err().get_message());
  }
  else {
    println!("Runnning program successfully");
  }
}
