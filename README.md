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
I abstracted the process of program startup into the run() function of modules/rormod.rs. The specific implementation of documents, terminal operations, etc. is in other files of modules.

## Install on your Linux
```
sudo su
```
Download rortext.tar . Put it in the directory you want to install.    
http://anan.i234.me:7777/ror/list.html
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

## How to use(v0.1.1)

### Move
You can use the arrow keys to move the cursor.
```
PageUp: Scroll up page
PageDown: Scroll down page
Home: Go to the beginning of line
End: Go to the End of line
```

### Hot key
```
Ctrl+s: Save, if the filename is empty, it will let you type
Ctrl+q: Quit program, press three times to force quit without saving
Ctrl+d: Command board
Ctrl+v: Paste the contents of the clipboard
Ctrl+x: Cut the contents of a line
```

### Command
```
quit //quit command board
rename [string] //The name of an existing file cannot be changed, only unsaved files can be renamed.
find //enter search mode, esc to quit
```

