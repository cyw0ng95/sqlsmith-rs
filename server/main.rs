use sqlsmith_rs_common::profile::read_profile;
use actix_web;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::App;
use actix_web::web;
use actix_cors::Cors;
use sqlsmith_rs_common::profile::write_profile;
use sqlsmith_rs_common::profile::Profile; // Import CORS middleware
mod fork_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    sqlsmith_rs_common::logger::init(); // Configure logging
    let _ = read_profile();

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors)
            .route("/profile/get", web::get().to(show_profile)) // 新增路由
            .route("/profile/put", web::post().to(handle_put_profile)) // 改为POST路由
            .route("/run", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn manual_hello() -> impl Responder {
    let profile = read_profile();
    profile.print();

    fork_server::fork_server_main(&profile);

    HttpResponse::Ok().body("Done!")
}

// 新增处理函数，用于保存 profile.json 内容
async fn handle_put_profile(profile: web::Json<Profile>) -> impl Responder {
    match write_profile(&profile) {
        Ok(_) => HttpResponse::Ok().body("Profile saved successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to save profile: {}", e)),
    }
}

// 新增处理函数，用于显示 profile.json 内容
async fn show_profile() -> impl Responder {
    let profile = read_profile();
    match serde_json::to_string_pretty(&profile) {
        Ok(json_str) => HttpResponse::Ok().body(json_str),
        Err(_) => HttpResponse::InternalServerError().body("Failed to serialize profile"),
    }
}