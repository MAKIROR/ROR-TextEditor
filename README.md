# ROR_TextEditor
A terminal text editor.


## Project structure
Just be a Rust exercise:)
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

## Install on your Linux
Download rortext.tar from http://anan.i234.me:7777/ror/list.html . Put it in the directory you want to install, for example:
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
ror set.sh
```
Run set.sh to set environment variables:
```
sudo ./set.sh
or
sudo bash ./set.sh
```
try to start it when done.If successful you will see a screen like this
```
ror [filename]
```
![](images/001.png)

## How to use(v0.1.2)

### Move
You can use the arrow keys to move the cursor.

### Hot key
```
Ctrl+s: Save, if the filename is empty, it will let you type
Ctrl+q: Quit program, press three times to force quit without saving
Ctrl+d: Command board
```

### Command
```
quit:exit command board
Find: enter search mode
rename: rename current file
```


