mod controllers;
mod middleware;
mod models;
mod routes;
mod error;
mod services;
mod config;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use middleware::auth::Auth;
use config::CONFIG;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("soli deo gloria")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();

    let server_config = &CONFIG.server;
    let address = format!("{}:{}", server_config.host, server_config.port);

    println!("Server starting at http://{}", address);
    println!("Environment: {}", CONFIG.environment);

    HttpServer::new(|| {
        // Configure CORS
        let mut cors = Cors::default();
        
        // Add allowed origins
        for origin in &CONFIG.server.cors_allowed_origins {
            cors = cors.allowed_origin(origin);
        }
        
        // Add other CORS settings
        cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .route("/", web::get().to(index))
            .service(
                web::scope("/bible")
                    .wrap(Auth)
                    .configure(routes::bible::config),
            )
            // .service(
            //     web::scope("/reflection")
            //         .wrap(Auth)
            //         .configure(routes::reflection::config),
            // )
            // .service(
            //     web::scope("/song")
            //         .wrap(Auth)
            //         .configure(routes::song::config),
            // )
    })
    .workers(4)  // Set number of worker threads
    .backlog(2048)  // Increase connection backlog
    .max_connections(10000)  // Increase max connections
    .max_connection_rate(1000)  // Set max connection rate
    .bind(address)?
    .run()
    .await
}