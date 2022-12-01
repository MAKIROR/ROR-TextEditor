# ROR_TextEditor
ROR TextEditor fixed some problems
### The file size is greatly reduced ()
Without compromising essential functionality, removed some dependent libraries, or replaced it with something simpler (see Cargo.toml)
```
// Cargo.toml
...

[dependencies]
termion = "2.0.1" 
unicode-segmentation = "1.10.0"
clipboard = "0.5.0"

...
```
original size: 128.3MB    
now: 26.5MB
### You can create a new file when you start it directly
In the previous version you couldn't do this if this file does not exist(
it doesn't create new file):
```
ror a/b.txt
```
