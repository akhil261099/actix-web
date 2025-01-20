use actix_web::{web, App, HttpServer, HttpResponse, Error}; 
use actix_cors::Cors;
use actix_web::middleware::Logger;
use execute_query::execute_query;
use create_table::create_table_in_snowflake;
mod execute_query;
mod create_table;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())  // Allow all origins
            .wrap(Logger::default())   // Optional: enable logging for debugging
            .route("/execute", web::post().to(execute_query))
            .route("/create", web::post().to(create_table_in_snowflake))
    })
    .bind("127.0.0.1:8080")?   // Ensure backend is running on this port
    .run()
    .await
}
