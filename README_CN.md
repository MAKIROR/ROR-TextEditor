# ROR_TextEditor
一个终端文本编辑器。


## 项目结构
这是一个简单的项目，只是一个Rust练习:)
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
Cargo.toml 包含一些我们需要使用的库，版本信息等。    
main.rs 是主文件。它决定了程序启动时执行了什么。具体的实现代码在 modules 文件中。    
我将程序启动过程抽象到了 modules/rormod.rs 的run()函数中。 文档、终端操作等的具体实现在 modules 文件中。

## 安装到您的Linux上
```
sudo su
```
下载rortext.tar。把它放在你相要安装的目录中
http://anan.i234.me:7777/ror/list.html
```
/home/makiror/ror/rortext.tar
```
解压缩：
```
tar xvf rortext.tar
```
你将看到这样的一个目录
```
ls
build  deps  examples  incremental  ror  ror.d  set.sh
```
执行 set.sh 命令设置环境变量：
```
./set.sh
or
bash ./set.sh
```
试着在完成后启动它。如果成功，您将看到如下屏幕所示
```
ror [filename]
```
![](images/001.png)

## 如何使用(v0.1.1)

### 移动
您可以使用方向键移动光标。
```
PageUp: 向上滚动页面
PageDown: 向下滚动页面
Home: 转到行首
End: 转到行尾
```

### 快捷键
```
Ctrl+s: 保存，如果文件名为空，它将允许您键入
Ctrl+q: 退出程序，按三次强制退出无需保持
Ctrl+d: 命令板
Ctrl+v: 粘贴剪切板内容
Ctrl+x: 剪切一行内容
```

### 命令
```
quit //退出命令板
rename [string] //不能修改已存在的文件名称，只能重命名未保存的文件
find //进入搜索模式，Esc退出
```
译：J.Anderson
