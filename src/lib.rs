//! # HwhKit
//!
//! 一个用于快速构建 Web 服务的 Rust 工具库，支持前后端分离和不分离两种架构。
//!
//! ## 特性
//!
//! - 🚀 一键构建 Web 服务
//! - 🔧 灵活的中间件系统
//! - 📝 支持模板渲染（前后端不分离）
//! - 🌐 支持 API 服务（前后端分离）
//! - ⚙️ 基于配置的中间件装载
//!
//! ## 快速开始
//!
//! ```rust
//! use hwhkit::WebServerBuilder;
//!
//! #[tokio::main]
//! async fn main() {
//!     let app = WebServerBuilder::new()
//!         .config_from_file("config.toml")
//!         .build()
//!         .await
//!         .expect("Failed to build server");
//!
//!     app.serve().await;
//! }
//! ```

pub mod builder;
pub mod config;
pub mod error;
pub mod middleware;
pub mod server;

#[cfg(feature = "templates")]
pub mod templates;

pub use builder::WebServerBuilder;
pub use config::Config;
pub use error::{Error, Result};
pub use server::WebServer;

// 重新导出常用的类型
pub use axum::{
    extract::{Json, Path, Query, State},
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{delete, get, patch, post, put, Router},
};

pub use serde::{Deserialize, Serialize};
pub use tokio;
pub use tower_http::cors::CorsLayer;
