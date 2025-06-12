use actix_cors::Cors;
use actix_web;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::web;
use sqlsmith_rs_common::profile::Profile;
use sqlsmith_rs_common::profile::read_profile;
use sqlsmith_rs_common::profile::write_profile; // Import CORS middleware
mod fork_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    sqlsmith_rs_common::logger::init(); // Configure logging
    let _ = read_profile();

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .route("/profile/get", web::get().to(show_profile)) // 新增路由
            .route("/profile/put", web::post().to(handle_put_profile)) // 改为POST路由
            .route("/run", web::get().to(manual_hello))
            .route("/internal/stat/collect", web::get().to(collect_executor_results)) // 新增统计收集路由
            .route("/internal/stat/submit", web::post().to(handle_stat_submission)) // 新增统计提交路由
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn manual_hello() -> impl Responder {
    let profile = read_profile();
    profile.print();

    fork_server::fork_server_main(&profile).await;

    HttpResponse::Ok().body("Done!")
}

// 新增处理函数，用于保存 profile.json 内容
async fn handle_put_profile(profile: web::Json<Profile>) -> impl Responder {
    match write_profile(&profile) {
        Ok(_) => HttpResponse::Ok().body("Profile saved successfully"),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to save profile: {}", e))
        }
    }
}

// 新增处理函数，用于显示 profile.json 内容
async fn show_profile() -> impl Responder {
    let profile = read_profile();
    match serde_json::to_string_pretty(&profile) {
        Ok(json_str) => HttpResponse::Ok()
            .content_type("application/json")
            .body(json_str),
        Err(_) => HttpResponse::InternalServerError().body("Failed to serialize profile"),
    }
}

// 新增处理函数，用于收集执行器结果
async fn collect_executor_results() -> impl Responder {
    use serde_json::json;

    let stats = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "executor_stats": {
            "total_executors": get_total_executor_count(),
            "active_executors": get_active_executor_count(),
            "completed_executors": get_completed_executor_count(),
        },
        "execution_results": {
            "total_queries": get_total_query_count(),
            "successful_queries": get_successful_query_count(),
            "failed_queries": get_failed_query_count(),
            "error_rate": calculate_error_rate(),
        },
        "performance": {
            "avg_execution_time_ms": get_avg_execution_time(),
            "queries_per_second": get_queries_per_second(),
            "uptime_seconds": get_uptime_seconds(),
        },
        "system": {
            "memory_usage_mb": get_memory_usage_mb(),
            "cpu_usage_percent": get_cpu_usage(),
        }
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .json(stats)
}

// 新增处理函数，用于接收执行器统计提交
async fn handle_stat_submission(stats: web::Json<serde_json::Value>) -> impl Responder {
    log::info!("Received executor statistics: {:?}", stats);

    // 这里可以将统计数据保存到文件、数据库或内存中
    // 目前简单记录日志

    HttpResponse::Ok().body("Statistics received successfully")
}

// 辅助函数 - 这些需要根据实际实现来完善
fn get_total_executor_count() -> u32 {
    let profile = read_profile();
    profile.executor_count.unwrap_or(0) as u32
}

fn get_active_executor_count() -> u32 {
    0 // 需要实现进程监控逻辑
}

fn get_completed_executor_count() -> u32 {
    0
}

fn get_total_query_count() -> u64 {
    0
}

fn get_successful_query_count() -> u64 {
    0
}

fn get_failed_query_count() -> u64 {
    0
}

fn calculate_error_rate() -> f64 {
    let total = get_total_query_count() as f64;
    let failed = get_failed_query_count() as f64;
    if total > 0.0 {
        (failed / total) * 100.0
    } else {
        0.0
    }
}

fn get_avg_execution_time() -> f64 {
    0.0
}

fn get_queries_per_second() -> f64 {
    0.0
}

fn get_uptime_seconds() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn get_memory_usage_mb() -> f64 {
    use std::fs;
    if let Ok(contents) = fs::read_to_string("/proc/self/status") {
        for line in contents.lines() {
            if line.starts_with("VmRSS:") {
                if let Some(kb_str) = line.split_whitespace().nth(1) {
                    if let Ok(kb) = kb_str.parse::<f64>() {
                        return kb / 1024.0; // 转换为MB
                    }
                }
            }
        }
    }
    0.0
}

fn get_cpu_usage() -> f64 {
    0.0
}
