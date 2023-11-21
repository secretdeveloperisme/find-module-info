use std::io::Result;
use prost_build::{self, compile_protos};
fn main() -> Result<()> {
  std::env::set_var("OUT_DIR", r"E:\Projects\find_module_info\src\proto");
  compile_protos(&[r"MakeFile.proto"], &[r"E:\Projects\find_module_info\rust_protobuf_builder\src\proto\"])?;
  Ok(())
}