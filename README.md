# ROR_TextEditor
A terminal text editor.


## Project structure
This is a simple project, many places are done by watching hento's tutorial
```
./
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    ├── main.rs
    └── modules
        ├── document.rs
        ├── rorlib.rs
        ├── row.rs
        └── terminal.rs
```
Cargo.toml includes some libraries we need to use, version information, etc.    
main.rs is the main file.It determines what is executed when the program starts.The specific implementation code is in the modules file.    
I abstracted the process of program startup into the run() function of modules/rorlib.rs. The specific implementation of documents, terminal operations, etc. is in other files of modules, and abstracted into corresponding functions in rorlib.rs

## How to use
