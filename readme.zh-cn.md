<center>

## gtl

[English](readme.md) | [简体中文](readme.zh-cn.md)

</center>

`gtl` 是一个基于 Git 的工具，旨在简化多远程仓库的管理。它扩展了 Git 的功能，提供了一键初始化和推送到多个远程仓库的功能，特别适合需要同时维护多个远程仓库的开发者。

## 特性

- **多远程仓库管理**：支持为一个本地仓库配置多个远程仓库。
- **一键初始化远程仓库**：通过简单的命令，一次性初始化并配置多个远程仓库。
- **一键推送到多个远程仓库**：可以通过一条命令将代码推送到所有已配置的远程仓库，节省时间和精力。
- **Git 命令扩展**：为 Git 提供了更多便捷的操作，提升工作效率。

## 安装

通过 `cargo` 安装 `gtl`：

```bash
cargo install gtl
```

## 使用

### 配置文件

> 路径: /home/.git_helper/config.json

```json
{
  "D:\\code\\gtl": [
    { "name": "gitee", "url": "git@gitee.com:ltpp-universe/gtl.git" },
    { "name": "origin", "url": "git@github.com:ltpp-universe/gtl.git" }
  ]
}
```

### 初始化多个远程仓库

假设你已经有一个本地 Git 仓库，并希望将其与多个远程仓库关联，使用以下命令：

```bash
gtl init
```

### 一键推送到所有远程仓库

配置好多个远程仓库后，使用以下命令将代码推送到所有已配置的远程仓库：

```bash
gtl push
```

### Git 添加 & 提交 & 推送

```bash
gtl acp
```

### 版本

```bash
gtl -v
gtl version
gtl --version
```

### 帮助

```bash
gtl help
```

## 赞赏

**如果你觉得 `hyperlane` 对你有所帮助，欢迎捐赠。**

<img src="https://docs.ltpp.vip/img/wechat-pay.png" width="200">  
<img src="https://docs.ltpp.vip/img/alipay-pay.jpg" width="200">

## 许可证

此项目基于 MIT 许可证授权。详细信息请查看 [license](license) 文件。

## 贡献

欢迎贡献！请提交 issue 或创建 pull request。

## 联系方式

如有任何疑问，请联系作者：[ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip)。
