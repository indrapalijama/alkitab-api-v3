use actix_web::web;
use crate::controllers::bible::{find, read};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/find/{book}", web::get().to(find))
            .route("/read/{book}/{chapter}", web::get().to(read))
    );
}