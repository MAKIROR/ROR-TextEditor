# ROR_TextEditor
ROR TextEditor has some updates
+ Fixed some weird issues
+ Added some hotkeys:
```
Ctrl v:Paste the contents of the clipboard
Ctrl x:Cut the contents of a line
```
+ Added a new command
```
rename [filename]
// The name of an existing file cannot be changed, only unsaved files can be renamed. If you open an existing file and use this command, an additional file will be save.I will try to fix this issue in a later version.
```
+ Changed a command

original:
```
find [string]
```
new:
```
find
//Enter find mode and type what to find.
```
Hotkeys for find mode:
```
Esc:exit find mode
Right or Down:go to the next search result
Left or Up: go to previous search result
```




