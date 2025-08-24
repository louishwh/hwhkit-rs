//! Web 服务器模块

use crate::{config::Config, error::{Error, Result}};
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Web 服务器
/// 
/// 封装了配置好的 Axum 应用和服务器配置
#[derive(Debug)]
pub struct WebServer {
    app: Router,
    config: Config,
}

impl WebServer {
    /// 创建新的 Web 服务器实例
    /// 
    /// # Arguments
    /// 
    /// * `app` - 配置好的 Axum 应用
    /// * `config` - 服务器配置
    pub fn new(app: Router, config: Config) -> Self {
        Self { app, config }
    }

    /// 运行服务器
    /// 
    /// # Arguments
    /// 
    /// * `addr` - 可选的监听地址，如果为 None 则使用配置中的地址
    pub async fn run(self, addr: Option<&str>) -> Result<()> {
        let default_addr = self.config.server_address();
        let bind_addr = addr.unwrap_or(&default_addr);
        
        tracing::info!("🚀 启动 HwhKit Web 服务器");
        tracing::info!("📡 监听地址: {}", bind_addr);
        tracing::info!("🏗️  架构模式: {:?}", self.config.server.architecture);
        
        // 打印中间件信息
        self.log_middleware_status();

        // 解析地址
        let socket_addr: SocketAddr = bind_addr.parse().map_err(|e| {
            Error::ServerStart(format!("无效的地址格式 '{}': {}", bind_addr, e))
        })?;

        // 创建 TCP 监听器
        let listener = TcpListener::bind(socket_addr).await.map_err(|e| {
            Error::ServerStart(format!("无法绑定到地址 '{}': {}", bind_addr, e))
        })?;

        tracing::info!("✅ 服务器启动成功，等待连接...");

        // 启动服务器
        axum::serve(listener, self.app).await.map_err(|e| {
            Error::ServerStart(format!("服务器运行时错误: {}", e))
        })?;

        Ok(())
    }

    /// 运行服务器（使用配置中的地址）
    pub async fn serve(self) -> Result<()> {
        self.run(None).await
    }

    /// 获取服务器配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取应用路由器的引用
    pub fn app(&self) -> &Router {
        &self.app
    }

    /// 打印中间件状态信息
    fn log_middleware_status(&self) {
        tracing::info!("🔧 中间件状态:");
        
        if self.config.middleware.cors.enabled {
            tracing::info!("  ✅ CORS: 已启用");
            tracing::info!("    📋 允许的源: {:?}", self.config.middleware.cors.origins);
        } else {
            tracing::info!("  ❌ CORS: 已禁用");
        }

        if self.config.middleware.static_files.enabled {
            tracing::info!("  ✅ 静态文件: 已启用");
            tracing::info!("    📁 目录: {}", self.config.middleware.static_files.dir);
            tracing::info!("    🔗 前缀: {}", self.config.middleware.static_files.prefix);
        } else {
            tracing::info!("  ❌ 静态文件: 已禁用");
        }

        if self.config.middleware.templates.enabled {
            tracing::info!("  ✅ 模板引擎: 已启用");
            tracing::info!("    📁 目录: {}", self.config.middleware.templates.dir);
        } else {
            tracing::info!("  ❌ 模板引擎: 已禁用");
        }

        if self.config.middleware.jwt.enabled {
            tracing::info!("  ✅ JWT 认证: 已启用");
            tracing::info!("    ⏰ 过期时间: {} 秒", self.config.middleware.jwt.expires_in);
        } else {
            tracing::info!("  ❌ JWT 认证: 已禁用");
        }

        if self.config.middleware.logging.requests {
            tracing::info!("  ✅ 请求日志: 已启用");
            tracing::info!("    📊 级别: {}", self.config.middleware.logging.level);
        } else {
            tracing::info!("  ❌ 请求日志: 已禁用");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ArchitectureType, Config};
    use axum::{routing::get, Router};

    async fn test_handler() -> &'static str {
        "Hello, World!"
    }

    #[test]
    fn test_web_server_creation() {
        let app = Router::new().route("/", get(test_handler));
        let config = Config::default();
        let server = WebServer::new(app, config);
        
        assert_eq!(server.config().server.port, 3000);
        assert_eq!(server.config().server.architecture, ArchitectureType::Api);
    }

    #[test]
    fn test_server_address_parsing() {
        let app = Router::new();
        let mut config = Config::default();
        config.server.host = "127.0.0.1".to_string();
        config.server.port = 8080;
        
        let server = WebServer::new(app, config);
        assert_eq!(server.config().server_address(), "127.0.0.1:8080");
    }
}