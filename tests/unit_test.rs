
use std::path::PathBuf;


#[test]
fn path_to_name_test() {
  use find_module_info::utils::path_to_name;

  let path1 = PathBuf::from("/this/is/the/path/for/linux");
  let file_name1: String = path_to_name(&path1);
  assert!(file_name1.eq("this_is_the_path_for_linux"));
  
  let path2 = PathBuf::from(r"D:\this\is\the\path\for\window");
  let file_name2: String = path_to_name(&path2);
  assert!(file_name2.eq("D_this_is_the_path_for_window"));
}

