
use std::path::PathBuf;


#[test]
fn path_to_name_test() {
  use find_module_info::utils::common_functions::path_to_name;

  let path1 = PathBuf::from("/this/is/the/path/for/linux");
  let file_name1: String = path_to_name(&path1);
  assert!(file_name1.eq("this_is_the_path_for_linux"));
  
  let path2 = PathBuf::from(r"D:\this\is\the\path\for\window");
  let file_name2: String = path_to_name(&path2);
  assert!(file_name2.eq("D_this_is_the_path_for_window"));
}

#[test]
fn test_capture_output_binary() {
  use find_module_info::core::find_output_binary::capure_output_binary;
  let content = String::from(r"
    LIBSTATIC=libsysteminfo.a
    all: $(OBJDIR)/$(LIBSTATIC)
  ");
  let result = capure_output_binary(&content).unwrap();
  assert_eq!(result, "libsysteminfo.a")
}

#[test]
fn test_capture_dependencies() {
  use find_module_info::core::find_output_binary::caputre_dependencies;
  let content = String::from(r"
  SHARED_LIBS = \
	$(PROD)/common/I18NWrapper/$(cfg)/libi18nwrapper.so \
	$(PROD)/srvrcommon/ntport/$(cfg)/libntport.so \

  STATIC_LIBS = \
	$(PROD)/thdparty/cgic205/$(cfg)/libcgic.a \
  ");
  let result = caputre_dependencies(&content).unwrap();
  assert_eq!(result[0],"libi18nwrapper.so");
  assert_eq!(result[1],"libntport.so");
  assert_eq!(result[2],"libcgic.a");

}