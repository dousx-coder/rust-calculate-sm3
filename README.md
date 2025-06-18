# 计算sm3

## 一. Rust开发者使用 cargo install

安装到$HOME/.cargo/bin目录下,方便Rust开发者使用

1. 当前目录下执行
```
cargo install --path .
```

2. 安装完成后，命令行中可直接使用`rust-calculate-sm3`

```shell
rust-calculate-sm3 --file=README.md
```

## 二.构建&发布

### 1.构建

```powershell
cargo build --release
```

### 2.配置环境变量

构建的可执行程序放入指定目录下，配置环境变量

### 3.运行

`--file`支持绝对路径和相对路径

```powershell
rust-calculate-sm3 --file=README.md
```

![image-20250617145502052](https://cruder-figure-bed.oss-cn-beijing.aliyuncs.com/markdown/2025/06/17/02-55-02-892.png)