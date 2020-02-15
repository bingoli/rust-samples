### 安装diesel命令行工具

> cargo install diesel_cli

 如果出现mysqlclient安装错误，可以只安装sqlite相关模块。

> cargo install diesel_cli --no-default-features --features sqlite

可参考：http://diesel.rs/guides/getting-started/


### 设置配置文件

> echo DATABASE_URL=demo.db > .env

### 创建数据库

> diesel setup

### 编译

> cargo build

### 运行

> cargo run --bin perf_test