use std::io;

use bytes::BytesMut;

pub mod make_file;
pub mod find_make_files;
pub mod find_output_binary;

pub trait Serializable {
  fn serialize(&self)->Result<BytesMut, io::Error>;
}