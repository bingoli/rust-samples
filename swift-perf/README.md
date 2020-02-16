
rust需要添加iOS的相关平台为编译目标

> rustup target add aarch64-apple-ios x86_64-apple-ios 

需要安装cbindgen生成C/C++的头文件

> cargo install cbindgen