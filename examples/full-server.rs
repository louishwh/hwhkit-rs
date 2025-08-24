//! 前后端不分离架构示例
//! 
//! 这个示例展示了如何使用 HwhKit 创建一个包含模板渲染的全栈应用

use axum::{
    extract::{Json, Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use hwhkit::{WebServerBuilder, Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: T,
    message: String,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data,
            message: "操作成功".to_string(),
        }
    }
}

// 模拟数据库
fn get_mock_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "张三".to_string(),
            email: "zhangsan@example.com".to_string(),
        },
        User {
            id: 2,
            name: "李四".to_string(),
            email: "lisi@example.com".to_string(),
        },
        User {
            id: 3,
            name: "王五".to_string(),
            email: "wangwu@example.com".to_string(),
        },
    ]
}

// 页面路由处理器
async fn index_page() -> impl IntoResponse {
    let template_data = json!({
        "title": "首页",
        "app_name": "HwhKit 演示应用",
        "version": "1.0.0",
        "users_count": get_mock_users().len(),
        "requests_count": 1024,
        "uptime": "7天"
    });

    render_template("index.html", &template_data).await
}

async fn users_page() -> impl IntoResponse {
    let users = get_mock_users();
    let template_data = json!({
        "title": "用户管理",
        "app_name": "HwhKit 演示应用",
        "version": "1.0.0",
        "users": users
    });

    render_template("users.html", &template_data).await
}

async fn about_page() -> impl IntoResponse {
    let template_data = json!({
        "title": "关于我们",
        "app_name": "HwhKit 演示应用",
        "version": "1.0.0",
        "description": "HwhKit 是一个强大且易用的 Rust Web 框架",
        "features": [
            "高性能的异步架构",
            "灵活的中间件系统",
            "丰富的模板支持",
            "简单的配置管理"
        ]
    });

    render_template("about.html", &template_data).await
}

// API 路由处理器
async fn api_get_users() -> impl IntoResponse {
    let users = get_mock_users();
    Json(ApiResponse::success(users))
}

async fn api_get_user(Path(id): Path<u64>) -> impl IntoResponse {
    let users = get_mock_users();
    
    if let Some(user) = users.into_iter().find(|u| u.id == id) {
        (StatusCode::OK, Json(ApiResponse::success(user))).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "error": "用户不存在",
                "code": 404
            }))
        ).into_response()
    }
}

async fn api_create_user(Json(user): Json<User>) -> impl IntoResponse {
    // 简单验证
    if user.name.is_empty() || user.email.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "用户名和邮箱不能为空",
                "code": 400
            }))
        ).into_response();
    }

    // 模拟保存用户
    let new_user = User {
        id: 100, // 模拟生成的 ID
        name: user.name,
        email: user.email,
    };

    (StatusCode::CREATED, Json(ApiResponse::success(new_user))).into_response()
}

async fn api_health() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "HwhKit 全栈服务器",
        "version": "1.0.0",
        "features": {
            "templates": true,
            "static_files": true,
            "api": true,
            "middleware": true
        }
    }))
}

// 简化的模板渲染函数
async fn render_template(template_name: &str, data: &serde_json::Value) -> impl IntoResponse {
    let html_content = match template_name {
        "index.html" => generate_index_html(data),
        "users.html" => generate_users_html(data),
        "about.html" => generate_about_html(data),
        _ => format!("<h1>模板 {} 不存在</h1>", template_name),
    };
    
    Html(html_content)
}

fn generate_index_html(data: &serde_json::Value) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - HwhKit</title>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <header>
        <nav class="navbar">
            <div class="nav-brand">
                <h1>{}</h1>
            </div>
            <ul class="nav-links">
                <li><a href="/">首页</a></li>
                <li><a href="/users">用户</a></li>
                <li><a href="/about">关于</a></li>
            </ul>
        </nav>
    </header>
    <main class="container">
        <div class="hero">
            <h1>🚀 欢迎使用 HwhKit</h1>
            <p class="subtitle">一个强大且易用的 Rust Web 框架</p>
            <div class="stats">
                <div class="stat">
                    <span class="stat-number">{}</span>
                    <span class="stat-label">注册用户</span>
                </div>
                <div class="stat">
                    <span class="stat-number">{}</span>
                    <span class="stat-label">处理请求</span>
                </div>
                <div class="stat">
                    <span class="stat-number">{}</span>
                    <span class="stat-label">运行时间</span>
                </div>
            </div>
            <div class="actions">
                <a href="/users" class="btn btn-primary">查看用户</a>
                <a href="/api/v1/health" class="btn btn-secondary">API 状态</a>
            </div>
        </div>
    </main>
    <footer>
        <p>&copy; 2024 HwhKit. 版本: {}</p>
    </footer>
    <script src="/static/script.js"></script>
</body>
</html>"#,
        data["title"].as_str().unwrap_or("首页"),
        data["app_name"].as_str().unwrap_or("HwhKit"),
        data["users_count"].as_u64().unwrap_or(0),
        data["requests_count"].as_u64().unwrap_or(0),
        data["uptime"].as_str().unwrap_or("0d"),
        data["version"].as_str().unwrap_or("1.0.0")
    )
}

fn generate_users_html(_data: &serde_json::Value) -> String {
    let users = get_mock_users();
    let users_html: String = users.iter().map(|user| {
        format!(
            r#"<div class="user-card">
                <div class="user-avatar">
                    <span>{}</span>
                </div>
                <div class="user-info">
                    <h3>{}</h3>
                    <p>{}</p>
                    <small>ID: {}</small>
                </div>
                <div class="user-actions">
                    <button class="btn btn-sm btn-edit">编辑</button>
                    <button class="btn btn-sm btn-delete">删除</button>
                </div>
            </div>"#, 
            user.name.chars().next().unwrap_or('?').to_uppercase(),
            user.name,
            user.email,
            user.id
        )
    }).collect();

    format!(
        r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>用户管理 - HwhKit</title>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <header>
        <nav class="navbar">
            <div class="nav-brand">
                <h1>HwhKit 演示应用</h1>
            </div>
            <ul class="nav-links">
                <li><a href="/">首页</a></li>
                <li><a href="/users">用户</a></li>
                <li><a href="/about">关于</a></li>
            </ul>
        </nav>
    </header>
    <main class="container">
        <div class="page-header">
            <h1>👥 用户管理</h1>
            <p>管理系统中的所有用户</p>
        </div>
        <div class="users-section">
            <div class="section-header">
                <h2>用户列表</h2>
                <button class="btn btn-primary">添加用户</button>
            </div>
            <div class="users-grid">
                {}
            </div>
        </div>
    </main>
    <footer>
        <p>&copy; 2024 HwhKit. 版本: 1.0.0</p>
    </footer>
    <script src="/static/script.js"></script>
</body>
</html>"#, 
        users_html
    )
}

fn generate_about_html(data: &serde_json::Value) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - HwhKit</title>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <header>
        <nav class="navbar">
            <div class="nav-brand">
                <h1>{}</h1>
            </div>
            <ul class="nav-links">
                <li><a href="/">首页</a></li>
                <li><a href="/users">用户</a></li>
                <li><a href="/about">关于</a></li>
            </ul>
        </nav>
    </header>
    <main class="container">
        <div class="page-header">
            <h1>📖 关于 HwhKit</h1>
            <p>{}</p>
        </div>
        <div class="users-section">
            <h2>主要特性</h2>
            <div class="features">
                <div class="feature-card">
                    <h3>⚡ 高性能</h3>
                    <p>基于 Axum 和 Tokio，提供出色的性能表现</p>
                </div>
                <div class="feature-card">
                    <h3>🛠️ 灵活的中间件</h3>
                    <p>丰富的中间件支持，满足各种开发需求</p>
                </div>
                <div class="feature-card">
                    <h3>🎨 模板支持</h3>
                    <p>内置模板引擎，轻松构建动态页面</p>
                </div>
                <div class="feature-card">
                    <h3>⚙️ 简单配置</h3>
                    <p>基于 TOML 的配置管理，简单易用</p>
                </div>
            </div>
        </div>
    </main>
    <footer>
        <p>&copy; 2024 HwhKit. 版本: {}</p>
    </footer>
    <script src="/static/script.js"></script>
</body>
</html>"#,
        data["title"].as_str().unwrap_or("关于我们"),
        data["app_name"].as_str().unwrap_or("HwhKit"),
        data["description"].as_str().unwrap_or("HwhKit 是一个强大且易用的 Rust Web 框架"),
        data["version"].as_str().unwrap_or("1.0.0")
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 构建页面路由
    let page_routes = Router::new()
        .route("/", get(index_page))
        .route("/users", get(users_page))
        .route("/about", get(about_page));

    // 构建 API 路由
    let api_routes = Router::new()
        .route("/users", get(api_get_users).post(api_create_user))
        .route("/users/:id", get(api_get_user))
        .route("/health", get(api_health));

    // 合并所有路由
    let app_routes = Router::new()
        .merge(page_routes)
        .nest("/api/v1", api_routes);

    // 创建服务器
    let server = WebServerBuilder::new()
        .config_from_file("examples/full-config.toml")
        .routes(app_routes)
        .build()
        .await?;

    println!("🚀 全栈服务器启动中...");
    println!("🏠 首页: http://localhost:8080/");
    println!("👥 用户管理: http://localhost:8080/users");
    println!("📖 关于页面: http://localhost:8080/about");
    println!("💾 API 健康检查: http://localhost:8080/api/v1/health");
    println!("👤 API 用户列表: http://localhost:8080/api/v1/users");
    
    server.serve().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_users() {
        let users = get_mock_users();
        assert_eq!(users.len(), 3);
        assert_eq!(users[0].name, "张三");
    }

    #[test]
    fn test_template_generation() {
        let data = json!({
            "title": "测试页面",
            "app_name": "测试应用",
            "version": "1.0.0",
            "users_count": 5,
            "requests_count": 100,
            "uptime": "1天"
        });

        let html = generate_index_html(&data);
        assert!(html.contains("测试页面"));
        assert!(html.contains("测试应用"));
    }
}