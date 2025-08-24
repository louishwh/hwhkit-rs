# HwhKit 🚀

一个用于快速构建 Web 服务的 Rust 工具库，支持前后端分离和不分离两种架构。

[![Crates.io](https://img.shields.io/crates/v/hwhkit.svg)](https://crates.io/crates/hwhkit)
[![Documentation](https://docs.rs/hwhkit/badge.svg)](https://docs.rs/hwhkit)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

## ✨ 特性

- 🚀 **一键构建** - 通过简单的构建器模式快速创建 Web 服务
- 🏗️ **双架构支持** - 支持前后端分离(API)和不分离(全栈)两种架构
- ⚙️ **配置驱动** - 基于 TOML 配置文件的中间件装载机制
- 🔧 **丰富中间件** - 内置 CORS、JWT、静态文件、模板渲染等中间件
- ⚡ **高性能** - 基于 Axum 和 Tokio 构建，性能卓越
- 🎨 **模板支持** - 内置 Tera 模板引擎支持（可选）
- 📝 **类型安全** - 完全的 Rust 类型安全保障

## 🛠️ 安装

将以下内容添加到你的 `Cargo.toml`：

```toml
[dependencies]
hwhkit = "0.1.0"
tokio = { version = "1.0", features = ["full"] }

# 可选特性
hwhkit = { version = "0.1.0", features = ["templates", "jwt"] }
```

### 可用特性

- `templates` - 启用 Tera 模板引擎支持
- `jwt` - 启用 JWT 认证支持
- `full` - 启用所有特性

## 📚 快速开始

### 前后端分离架构（API 模式）

```rust
use hwhkit::{WebServerBuilder, get, Json, Serialize};

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    status: String,
}

async fn hello_world() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Hello, World!".to_string(),
        status: "success".to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = hwhkit::Router::new()
        .route("/", get(hello_world));

    let server = WebServerBuilder::new()
        .listen("0.0.0.0", 3000)
        .cors(vec!["http://localhost:3000".to_string()])
        .routes(app)
        .build()
        .await?;

    server.serve().await?;
    Ok(())
}
```

### 前后端不分离架构（全栈模式）

```rust
use hwhkit::{WebServerBuilder, get, Html, ArchitectureType};

async fn index() -> Html<&'static str> {
    Html("<h1>欢迎使用 HwhKit!</h1>")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = hwhkit::Router::new()
        .route("/", get(index));

    let server = WebServerBuilder::new()
        .listen("0.0.0.0", 8080)
        .architecture(ArchitectureType::Full)
        .static_files("static", "/static")
        .templates("templates", "html")
        .routes(app)
        .build()
        .await?;

    server.serve().await?;
    Ok(())
}
```

### 使用配置文件

创建 `config.toml`：

```toml
[server]
host = "0.0.0.0"
port = 3000
architecture = "api"

[middleware.cors]
enabled = true
origins = ["*"]
methods = ["GET", "POST", "PUT", "DELETE"]
headers = ["Content-Type", "Authorization"]

[middleware.jwt]
enabled = true
secret = "your-secret-key"
expires_in = 3600

[middleware.static_files]
enabled = true
dir = "public"
prefix = "/static"

[middleware.logging]
level = "info"
requests = true
```

然后在代码中使用：

```rust
use hwhkit::WebServerBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = WebServerBuilder::new()
        .config_from_file("config.toml")
        .build()
        .await?;

    server.serve().await?;
    Ok(())
}
```

## 🎯 架构类型

### API 架构（前后端分离）

适用于构建 RESTful API 服务：

- ✅ CORS 支持
- ✅ JSON 响应
- ✅ JWT 认证
- ✅ 静态文件服务
- ❌ 模板渲染

### Full 架构（前后端不分离）

适用于构建传统的全栈 Web 应用：

- ✅ 模板渲染
- ✅ 静态文件服务
- ✅ 表单处理
- ✅ 会话管理
- ⚠️ CORS（通常不需要）

## 🔧 中间件配置

### CORS

```toml
[middleware.cors]
enabled = true
origins = ["http://localhost:3000", "https://yourdomain.com"]
methods = ["GET", "POST", "PUT", "DELETE"]
headers = ["Content-Type", "Authorization"]
```

### JWT 认证

```toml
[middleware.jwt]
enabled = true
secret = "your-super-secure-secret-key"
expires_in = 3600  # 1小时
```

### 静态文件

```toml
[middleware.static_files]
enabled = true
dir = "public"  # 静态文件目录
prefix = "/static"  # URL 前缀
```

### 模板引擎

```toml
[middleware.templates]
enabled = true
dir = "templates"  # 模板文件目录
extension = "html"  # 模板文件扩展名
```

### 日志

```toml
[middleware.logging]
level = "info"  # trace, debug, info, warn, error
requests = true  # 启用请求日志
```

## 📖 示例

查看 `examples/` 目录获取完整示例：

- [`api-server.rs`](examples/api-server.rs) - 前后端分离架构示例
- [`full-server.rs`](examples/full-server.rs) - 前后端不分离架构示例

运行示例：

```bash
# API 服务器示例
cargo run --example api-server

# 全栈服务器示例
cargo run --example full-server
```

## 🧪 测试

运行所有测试：

```bash
cargo test
```

运行特定特性的测试：

```bash
cargo test --features "templates,jwt"
```

## 📋 路线图

- [x] 基本 Web 服务器构建
- [x] 配置文件支持
- [x] CORS 中间件
- [x] 静态文件服务
- [x] 基础模板支持
- [x] JWT 认证框架
- [ ] 数据库集成
- [ ] WebSocket 支持
- [ ] 请求限流
- [ ] 缓存中间件
- [ ] 监控和指标
- [ ] 热重载开发模式

## 🤝 贡献

欢迎贡献代码！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详细信息。

### 开发环境设置

```bash
# 克隆仓库
git clone https://github.com/yourusername/hwhkit.git
cd hwhkit

# 安装依赖
cargo build

# 运行测试
cargo test

# 运行示例
cargo run --example api-server
```

## 📄 许可证

本项目使用 [MIT](LICENSE-MIT) 或 [Apache-2.0](LICENSE-APACHE) 许可证。

## 🙋 支持

- 📖 [文档](https://docs.rs/hwhkit)
- 🐛 [问题反馈](https://github.com/yourusername/hwhkit/issues)
- 💬 [讨论](https://github.com/yourusername/hwhkit/discussions)

## 🌟 致谢

感谢以下项目为 HwhKit 提供了灵感和基础：

- [Axum](https://github.com/tokio-rs/axum) - 现代异步 Web 框架
- [Tower](https://github.com/tower-rs/tower) - 模块化网络服务
- [Tera](https://github.com/Keats/tera) - 模板引擎
- [Serde](https://github.com/serde-rs/serde) - 序列化框架

---

<div align="center">
  <p>用 ❤️ 和 🦀 制作</p>
  <p>如果这个项目对你有帮助，请给我们一个 ⭐</p>
</div>