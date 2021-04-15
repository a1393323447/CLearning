# CLearning
一个帮助C语言初学者熟悉C语言基本语法的命令行程序。
## 第一步 clone 到本地
```
git clone https://github.com/a1393323447/CLearning.git
``` 
## 第二步 
打开 CLearning 文件夹
## 第三步 
打开终端(cmd 或 powershell)，输入:
```
.\clearning
```
## 最后
根据提示信息，开始练习。

# CLearning 现有功能
## verify
按照推荐顺序检验所有的练习
## watch
每当当前练习文件被修改, 就会自动检验, 当通过练习后, 就会按照推荐顺序进行下一个练习
## run
运行单个练习文件
## hint
给出当前练习的提示
# 如何贡献题目
在 `exercise` 文件夹中创建你的习题, 注意习题文件中必须包含 `// I AM NOT DONE` , 并且要有题目描述。最后将你的题目对应的信息添加到 `info.toml` 文件中, 最后提交。题目信息格式如下:
```
[[exercises]]
name = "HelloWorld"
path = "exercises/HelloWorld.c"
hint = """
Hint: 你好世界！！！
"""
```
# 如何编译 CLearning
## 安装 rust 编辑器
```
rust官网 https://www.rust-lang.org/learn/get-started
```
## 打开 CLearning 文件夹
```
cd ./CLearning
```
## 构建 CLearning 
在终端中输入:
```
cargo build --release
```
在 `target/release` 中可以找到编译完成的 CLearning
