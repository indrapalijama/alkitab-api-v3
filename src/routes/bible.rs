use actix_web::web;
use crate::controllers::bible;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(bible::find)
        .service(bible::read);
}