use std::{path::PathBuf, fs};
use clap::{Parser, Args};



#[derive(Parser, Debug)]
#[command(author = "hoanglinh", version="1.0.0", about, long_about = "find module information")]
pub struct Arguments{

  #[arg(short = 'a', long = "action", value_name = "Input a action: makefile, dependants, servers, all ")]
  action: String,
  
  #[arg(short = 'm', long ="module_name", value_name = "target module", default_value = "")]
  module_name: String,

  #[clap(flatten)]
  find_options: FindOptions
}

#[derive(Debug, Args)]
pub struct FindOptions{

  #[arg(short = 's', long = "source_path", value_name = "source code path", default_value = "empty path")]
  source_path: PathBuf,

  #[arg(short = 'i', long = "is_hidden", value_name = "find hidden files", default_value = "false")]
  is_hidden: bool,

  #[arg(short = 'e', long = "exclude_folder", value_name = "exclude folders", value_parser, value_parser, value_delimiter = ',' , default_value = "empty exclude folders list")]
  exclude_folder: Vec<String>
}
#[allow(unused)]
impl Arguments{
  pub fn get_action(&self) -> &str{
    &self.action
  }
  pub fn get_module_name(&self) -> &str{
    &self.module_name
  }
  pub fn get_source_path(&self) ->&PathBuf{
    &self.find_options.source_path
  }
  pub fn set_source_path(&mut self, sourc_path: PathBuf){
    self.find_options.source_path = sourc_path
  }
  
  pub fn get_find_options(&self)->&FindOptions{
    &self.find_options
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

impl FindOptions {
  // get methods

  pub fn get_source_path(&self) -> &PathBuf {
    &self.source_path
  }

  pub fn is_hidden(&self) -> bool {
    self.is_hidden
  }

  pub fn get_exclude_folder(&self) -> &Vec<String> {
    &self.exclude_folder
  }

  // set methods

  pub fn set_source_path(&mut self, source_path: PathBuf) {
    self.source_path = source_path;
  }

  pub fn set_is_hidden(&mut self, is_hidden: bool) {
    self.is_hidden = is_hidden;
  }

  pub fn set_exclude_folder(&mut self, exclude_folder: Vec<String>) {
    self.exclude_folder = exclude_folder;
  }
}