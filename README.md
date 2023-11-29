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

Collect makefiels from source path to a single place with the many options: hidden, exclude folders.

```bash
find_module_info.exe --action collect --source_path path --is_hidden --exclude_folder folders
```

The arguments are:

- **--action** *collect*: This tells the tool to collect makefiles from the source path and copy them to a folder named **ouput/makefile_temp** in the current directory.
- **--source_path** *path*: This specifies the source path where the tool will look for makefiles. You can use absolute or relative paths. For example, C:\Users\source_code.
- **--is_hidden**: This is an optional argument that tells the tool to include hidden files and folders in the search. By default, the tool will ignore hidden items.
- **--exclude_folder** *folders*:  This is an optional argument that tells the tool to exclude certain folders from the search. You can specify one or more folder names, separated by commas. For example, build,test,docs.

## 2. Process Makefies

Find the binary output, dependencies and dependants of the modules, then store the information into a database file with the name *makefiles.db*.

```bash
find_module_info.exe --action binary # find output binary name
find_module_info.exe --action depend # find dependencis and dependants
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

Retreive information from makefile database and show the output

```bash
find_module_info.exe --action find --module_name name
```

The arguments are:

- **--action** *find*: This tells the tool to find the module information.
- **--module_name** *name*: This specify the module output name. You can input a part or fullname of the module. For example: libvnmedif.so.

### Ouput Example

Here it's a result of module libvnmedif.so

```bash
====================libvnmedif.so====================
PATH: new_main_dev\vs\vitalnet\vnMedIf\Makefile
OUTPUT_BINARY: libvnmedif.so
DEPENDENCIES:
        libntport.so
        libvssrvrmanager.so
        libvsdbdll.so
DEPENDANTS:
        vsvoip
        vswebsql.exe
        acollector
        CCMCDR
        ICE_VoIP
        MSLyncCollector
        OXE_VoIP
        OXE_VoIPStream
        SIPServer
        adminGenesys.cgi
        vnsubmittomed.exe
        vncdrfilter.cgi
        vncdrvoiceqos.cgi
        vncodectype.cgi
        vnmosconst.cgi
        vnphonecalldesc.cgi
        startcollectors
        voipstreamctrl
        libvoipstreamjni.so
```