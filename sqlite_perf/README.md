### 安装diesel命令行工具

> cargo install diesel_cli

### 设置配置文件

> echo DATABASE_URL=demo.db > .env

### 创建数据库

> diesel setup

### 编译

> cargo build

### 运行

> cargo run --bin perf_test