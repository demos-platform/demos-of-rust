# demos-of-rust

该项目为阅读《Rust权威指南》过程中出现的相关实践案例。

### 安装

> https://www.rust-lang.org/tools/install

rustup: Rust 命令行工具，用于下载安装 Rust，还用来管理不同的 Rust 发行版本。

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完 rustup 后，可以执行以下命令

```
// 检查 Rust 是否已经安装
rustc --version

// 更新 Rust 版本
rustup update

// 卸载 rustup 及 Rust 工具链
rustup self uninstall

// 生成一份可以在网页中进行访问的离线的 Rust 文档
rustup doc
```

### Cargo

Cargo 是 Rust 项目中的构建系统与包依赖管理工具。

```
// 创建一个新 Rust 项目
cargo new xx

// 检查是否可以编译通过
cargo check

// 编译项目，构建产物位于 target/debug 目录下
cargo build
// release 模式用更长的编译时间，以优化运行时的性能。构建产物位于 target/release 目录下
cargo build --release

// 编译并运行项目
cargo run
// 运行 target/release 下的二进制文件
cargo run --release

// 打开文档，查看依赖包文档(经验证，执行以下命令，暂未看到依赖包相关信息，如果后续验证结果有变，再进行修改)
cargo doc --open
```