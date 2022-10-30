# ROR_TextEditor
A terminal text editor.


## Project structure
This is a simple project, Just be a Rust exercise:)
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

## Install on your Linux
```
sudo su
```
Download rortext.tar from 'release' branch. Put it in the directory you want to install, for example:
```
/home/makiror/ror/rortext.tar
```
unzip it:
```
tar xvf rortext.tar
```
You will see a directory like this
```
ls
build  deps  examples  incremental  ror  ror.d  set.sh
```
Run set.sh to set environment variables:
```
./set.sh
or
bash ./set.sh
```
try to start it when done.If successful you will see a screen like this
```
ror [filename]
```
![](images/001.png)

## How to use
