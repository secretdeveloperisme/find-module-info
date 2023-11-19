pub mod common_functions;


pub enum ProgramResult{
  Ok(()),
  Err(Error)
}
impl ProgramResult {
  pub fn is_ok(&self)->bool{
    match *self {
      ProgramResult::Ok(_) => true,
      ProgramResult::Err(_) => false
    }
  }
  pub fn get_err(&self)->&Error{
    if let ProgramResult::Err(e) = self{
      return e;
    }
    panic!("The Result is Ok")
    
  }
}
pub struct Error{
  message: String
}
impl Error {
  pub fn new(message: &str) -> Self{
    Error { message: message.into()}
  }
  pub fn get_message(&self)-> &String{
    &self.message
  }
}

