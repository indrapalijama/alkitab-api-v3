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
use alkitab_api_rust::api_docs::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .route("/", web::get().to(index))
            .service(
                web::scope("/bible")
                    .wrap(Auth)
                    .configure(routes::bible::config),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
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