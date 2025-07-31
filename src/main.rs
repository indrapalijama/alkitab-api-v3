mod controllers;
mod middleware;
mod models;
mod routes;
mod error;
mod services;
mod config;

use actix_cors::Cors;
use actix_web::{web, HttpResponse, Responder};
use middleware::auth::Auth;
use alkitab_api_rust::api_docs::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("soli deo gloria")
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    
    
    println!("Environment: {}", std::env::var("RUST_ENV").unwrap_or_else(|_| "production".to_string()));
    
    // Get the secret for middleware
    let auth_secret = secrets.get("SECRET")
        .expect("SECRET not found in secrets store")
        .to_string();
    
    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.service(
                web::resource("/")
                    .wrap(
                        Cors::default()
                            .allow_any_origin()
                            .allow_any_method()
                            .allow_any_header()
                    )
                    .route(web::get().to(index))
            )
            .service(
                web::scope("/bible")
                    .wrap(
                        Cors::default()
                            .allow_any_origin()
                            .allow_any_method()
                            .allow_any_header()
                    )
                    .wrap(Auth::new(auth_secret.clone()))
                    .configure(routes::bible::config),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            );
            // .service(
            //     web::scope("/reflection")
            //         .wrap(
            //             Cors::default()
            //                 .allow_any_origin()
            //                 .allow_any_method()
            //                 .allow_any_header()
            //         )
            //         .wrap(Auth)
            //         .configure(routes::reflection::config),
            // )
            // .service(
            //     web::scope("/song")
            //         .wrap(
            //             Cors::default()
            //                 .allow_any_origin()
            //                 .allow_any_method()
            //                 .allow_any_header()
            //         )
            //         .wrap(Auth)
            //         .configure(routes::song::config),
            // )
    };
    Ok(config.into())
}