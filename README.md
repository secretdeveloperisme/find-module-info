# Find Module Information

Find module information tool facilitates finding dependencies and dependants easily.

```text
  ______ _           _   __  __           _       _        _____        __      
 |  ____(_)         | | |  \/  |         | |     | |      |_   _|      / _|     
 | |__   _ _ __   __| | | \  / | ___   __| |_   _| | ___    | |  _ __ | |_ ___  
 |  __| | | '_ \ / _` | | |\/| |/ _ \ / _` | | | | |/ _ \   | | | '_ \|  _/ _ \ 
 | |    | | | | | (_| | | |  | | (_) | (_| | |_| | |  __/  _| |_| | | | || (_) |
 |_|    |_|_| |_|\__,_| |_|  |_|\___/ \__,_|\__,_|_|\___| |_____|_| |_|_| \___/ 
```

## 1. Collect makefiles

Collect makefiles from source path to a single place with the many options: hidden, exclude folders.

```bash
find_module_info.exe --action collect --source_path path --is_hidden --exclude_folder folders
```

The arguments are:

- **--action** *collect*: This tells the tool to collect makefiles from the source path and copy them to a folder named **output/makefile_temp** in the current directory.
- **--source_path** *path*: This specifies the source path where the tool will look for makefiles. You can use absolute or relative paths. For example, C:\Users\source_code.
- **--is_hidden**: This is an optional argument that tells the tool to include hidden files and folders in the search. By default, the tool will ignore hidden items.
- **--exclude_folder** *folders*:  This is an optional argument that tells the tool to exclude certain folders from the search. You can specify one or more folder names, separated by commas. For example, build,test,docs.

## 2. Process Makefiles

Find the binary output, dependencies and dependants of the modules, then store the information into a database file with the name *makefiles.db*.

```bash
find_module_info.exe --action binary # find output binary name
find_module_info.exe --action depend # find dependencies and dependants
```

## 3. Combine action collect, binary and depend

You can combine the collect, find output binary name and depends into a single command.

```bash
find_module_info.exe --action update --source_path path
```

The arguments are:

- **--action** *collect*: This tells the tool to collect make files from the source path, then find output binary, dependencies and dependant.
- **--source_path** *path*: This specifies the source path where the tool will look for makefiles. You can use absolute or relative paths. For example, C:\Users\source_code.

## Find module

Retrieve information from makefile database and show the output

```bash
find_module_info.exe --action find --module_name name
```

The arguments are:

- **--action** *find*: This tells the tool to find the module information.
- **--module_name** *name*: This specifies the module output name. You can input a part or fullname of the module. For example: libmedif.so.

### Output Example

Here it's a result of module libvnmedif.so

```bash
====================libservice.so====================
PATH: develop\utils\service\Makefile 
OUTPUT_BINARY: libservice.so
DEPENDENCIES:
        libstring.so
        libkernel.so
        libsignal.so
DEPENDANTS:
        service_manager
        webapi.cgi
```
