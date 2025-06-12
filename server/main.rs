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
use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

mod fork_server;

// Define ExecutionStats locally since sqlsmith_rs_executor is not available
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecutionStats {
    pub elapsed_ms: u64,
    pub success_count: usize,
    pub failed_expected_count: usize,
    pub failed_new_count: usize,
    pub total_queries: usize,
    pub thread_count: usize,
    pub queries_per_second: f64,
    pub error_rate: f64,
    pub stmt_type_counts: HashMap<String, usize>,
    pub executor_id: String,
    pub timestamp: String,
}

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

    // Get aggregated statistics from handle_stat_submission
    let aggregated = AGGREGATED_STATS.lock().unwrap();
    
    if let Some(agg) = aggregated.as_ref() {
        // Calculate metrics from aggregated data
        let overall_qps = if agg.total_elapsed_ms > 0 {
            (agg.total_queries as f64) / (agg.total_elapsed_ms as f64 / 1000.0)
        } else {
            0.0
        };
        
        let overall_error_rate = if agg.total_queries > 0 {
            (agg.total_failed_new_count as f64 / agg.total_queries as f64) * 100.0
        } else {
            0.0
        };

        let stats = json!({
            "timestamp": agg.last_updated,
            "executor_stats": {
                "total_executors": agg.total_executors,
                "active_executors": 0, // Could be enhanced with process monitoring
                "completed_executors": agg.total_executors,
            },
            "execution_results": {
                "total_queries": agg.total_queries,
                "successful_queries": agg.total_success_count,
                "failed_expected_queries": agg.total_failed_expected_count,
                "failed_new_queries": agg.total_failed_new_count,
                "error_rate": overall_error_rate,
                "stmt_type_counts": agg.combined_stmt_type_counts,
            },
            "performance": {
                "max_execution_time_ms": agg.total_elapsed_ms,
                "queries_per_second": overall_qps,
                "total_thread_count": agg.total_thread_count,
            },
        });

        HttpResponse::Ok()
            .content_type("application/json")
            .json(stats)
    } else {
        // No data collected yet
        let empty_stats = json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "executor_stats": {
                "total_executors": 0,
                "active_executors": 0,
                "completed_executors": 0,
            },
            "execution_results": {
                "total_queries": 0,
                "successful_queries": 0,
                "failed_expected_queries": 0,
                "failed_new_queries": 0,
                "error_rate": 0.0,
                "stmt_type_counts": {},
            },
            "performance": {
                "max_execution_time_ms": 0,
                "queries_per_second": 0.0,
                "total_thread_count": 0,
            },
            "message": "No executor statistics collected yet"
        });

        HttpResponse::Ok()
            .content_type("application/json")
            .json(empty_stats)
    }
}

// Global state to store aggregated statistics
static AGGREGATED_STATS: Mutex<Option<AggregatedStats>> = Mutex::new(None);

#[derive(Debug, Clone)]
struct AggregatedStats {
    total_executors: usize,
    total_elapsed_ms: u64,
    total_success_count: usize,
    total_failed_expected_count: usize,
    total_failed_new_count: usize,
    total_queries: usize,
    total_thread_count: usize,
    combined_stmt_type_counts: HashMap<String, usize>,
    last_updated: String,
}

// 新增处理函数，用于接收执行器统计提交
async fn handle_stat_submission(stats: web::Json<ExecutionStats>) -> impl Responder {
    log::info!("Received executor statistics from: {}", stats.executor_id);

    // Check if executor_id is a valid number
    if stats.executor_id.parse::<u32>().is_err() {
        return HttpResponse::BadRequest().body("executor_id must be a valid number");
    }

    // Update aggregated statistics
    let mut aggregated = AGGREGATED_STATS.lock().unwrap();
    
    match aggregated.as_mut() {
        Some(agg) => {
            // Update existing aggregated stats
            agg.total_executors += 1;
            agg.total_elapsed_ms = agg.total_elapsed_ms.max(stats.elapsed_ms); // Use max elapsed time
            agg.total_success_count += stats.success_count;
            agg.total_failed_expected_count += stats.failed_expected_count;
            agg.total_failed_new_count += stats.failed_new_count;
            agg.total_queries += stats.total_queries;
            agg.total_thread_count += stats.thread_count;
            
            // Merge statement type counts
            for (stmt_type, count) in &stats.stmt_type_counts {
                *agg.combined_stmt_type_counts.entry(stmt_type.clone()).or_insert(0) += count;
            }
            
            agg.last_updated = chrono::Utc::now().to_rfc3339();
        }
        None => {
            // Initialize aggregated stats
            *aggregated = Some(AggregatedStats {
                total_executors: 1,
                total_elapsed_ms: stats.elapsed_ms,
                total_success_count: stats.success_count,
                total_failed_expected_count: stats.failed_expected_count,
                total_failed_new_count: stats.failed_new_count,
                total_queries: stats.total_queries,
                total_thread_count: stats.thread_count,
                combined_stmt_type_counts: stats.stmt_type_counts.clone(),
                last_updated: chrono::Utc::now().to_rfc3339(),
            });
        }
    }

    let agg = aggregated.as_ref().unwrap();
    
    // Calculate aggregated metrics
    let overall_qps = if agg.total_elapsed_ms > 0 {
        (agg.total_queries as f64) / (agg.total_elapsed_ms as f64 / 1000.0)
    } else {
        0.0
    };
    
    let overall_error_rate = if agg.total_queries > 0 {
        (agg.total_failed_new_count as f64 / agg.total_queries as f64) * 100.0
    } else {
        0.0
    };

    let summary = format!(
        "Statistics updated successfully!\n\nAggregated Results:\n\
        Executors: {}\n\
        Total Queries: {}\n\
        Success: {}\n\
        Failed (expected): {}\n\
        Failed (new): {}\n\
        Total Threads: {}\n\
        Overall QPS: {:.2}\n\
        Overall Error Rate: {:.2}%\n\
        Max Execution Time: {}ms\n\
        Last Updated: {}",
        agg.total_executors,
        agg.total_queries,
        agg.total_success_count,
        agg.total_failed_expected_count,
        agg.total_failed_new_count,
        agg.total_thread_count,
        overall_qps,
        overall_error_rate,
        agg.total_elapsed_ms,
        agg.last_updated
    );

    HttpResponse::Ok().body(summary)
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