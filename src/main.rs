use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize};

#[derive(Serialize)]
struct OracleData {
    slot: u64,
    block_root: String,
    timestamp: u64,
}

async fn get_oracle_data() -> impl Responder {
    let data = OracleData {
        slot: 123456,
        block_root: "0xabcdef1234567890".to_string(),
        timestamp: 1710000000,
    };
    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Beacon Chain Oracle running on http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route("/oracle", web::get().to(get_oracle_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
