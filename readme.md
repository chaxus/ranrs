# vscode 插件
1. rust-analyzer  Rust 语言的插件
2. Even Better TOML，支持 .toml 文件完整特性
3. Error Lens, 更好的获得错误展示
4. CodeLLDB, Debugger 程序

# cargo 初始化项目

```
cargo new [project]
```

# cargo 运行

```
cargo run
```

```
cargo build
```

```
cargo check
```

# 增加新的镜像源

```
$HOME/.cargo/config.toml
```

```
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'rsproxy'

# 科大镜像
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 清华镜像
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 字节镜像
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

# 稀疏索引，要求 cargo >= 1.68
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
```

首先，创建一个新的镜像源 [source.ustc]，然后将默认的 crates-io 替换成新的镜像源: replace-with = 'ustc'。

