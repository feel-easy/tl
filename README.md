# TL - 终端翻译工具

一个简单实用的命令行翻译工具，支持中英互译（基于有道翻译API）。

## 功能特点

- 支持命令行参数传递文本进行翻译
- 支持管道输入，可与其他命令配合使用
- 自动识别源语言，进行对应翻译
- 简洁的输出，专注于翻译结果

## 安装

使用 Cargo 安装（需要安装 Rust 环境）：

```shell
git clone https://github.com/yourusername/tl.git
cd tl
cargo install --path .
```

## 使用方法

### 基本用法

```shell
# 英译中
tl Today is a good day.
# 输出: 今天是个好日子

# 中译英
tl 今天是个好日子
# 输出: Today is a good day
```

### 管道用法

```shell
# 使用管道传递输入
echo "Hello world" | tl
# 输出: 你好世界

# 结合其他命令
cat file.txt | tl
```

### 帮助信息

```shell
tl --help
```

## 环境要求

- Rust 1.54.0 或更高版本

## 依赖项

- clap - 命令行参数解析
- reqwest - HTTP 客户端
- tokio - 异步运行时
- serde - JSON 序列化/反序列化
- urlencoding - URL 编码
- atty - 终端检测

## 许可证

MIT

## 致谢

感谢有道翻译提供的 API 服务。
