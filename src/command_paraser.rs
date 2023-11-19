use std::{path::PathBuf, fs};
use clap::Parser;



#[derive(Parser,Debug)]
#[command(author = "hoanglinh", version="1.0.0", about, long_about = "find module information")]
pub struct Args{

  #[arg(short = 'a', long = "action", value_name = "Input a action: makefile, dependants, servers, all ")]
  action: String,
  
  #[arg(short = 'm', long ="module_name", value_name = "target module", default_value = "")]
  module_name: String,

  #[arg(short = 's', long = "source_path", value_name = "source code path", default_value = "no path")]
  source_path: PathBuf
}

impl Args{
  pub fn get_action(&self) -> &str{
    &self.action
  }
  pub fn get_module_name(&self) -> &str{
    &self.module_name
  }
  pub fn get_source_path(&self) ->&PathBuf{
    &self.source_path
  }
  pub fn set_source_path(&mut self, sourc_path: PathBuf){
    self.source_path = sourc_path
  }

  pub fn check(&self)->Result<(), String>{
    let actions: Vec<&str>= vec!["makefiles", "dependants", "servers", "all"];
    if !actions.into_iter().any(|action|{
      action.eq(&self.action)
    }){
      return Err(String::from("No such action"));
    }
    if !self.get_source_path().is_dir(){
      return Err(String::from("Source Path is not valid"));
    }
    Ok(())
  }
  
  pub fn build() ->Result<Self,String> {
    let mut args = Self::parse();
    if args.get_source_path().is_relative(){
      args.set_source_path(fs::canonicalize(args.get_source_path().clone()).unwrap());
    }
    args.check()?;
    Ok(args)
  }
}