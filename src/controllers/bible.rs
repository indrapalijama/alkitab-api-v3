use actix_web::{get, web, HttpResponse};
#[allow(unused_imports)]
use crate::models::bible::{BibleMetadata, BibleChapter, ErrorResponse};
use crate::services::bible;
use crate::error::AppError;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct QueryParams {
    version: Option<String>,
}

#[utoipa::path(
    get,
    path = "/bible/find/{book}",
    responses(
        (status = 200, description = "Get Bible book metadata", body = BibleMetadata),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    params(
        ("book" = String, Path, description = "Book name")
    ),
    security(
        ("accesskey" = [])
    )
)]
#[get("/find/{book}")]
pub async fn find(path: web::Path<String>) -> Result<HttpResponse, AppError> {
    let book = path.into_inner().trim().to_string();
    let result = bible::find(&book).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    get,
    path = "/bible/read/{book}/{chapter}",
    responses(
        (status = 200, description = "Get Bible chapter content", body = BibleChapter),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    params(
        ("book" = String, Path, description = "Book name"),
        ("chapter" = i32, Path, description = "Chapter number")
    ),
    security(
        ("accesskey" = [])
    )
)]
#[get("/read/{book}/{chapter}")]
pub async fn read(
    path: web::Path<(String, i32)>,
    query: web::Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    let (book, chapter) = path.into_inner();
    let book = book.trim().to_string();
    let version = query.version.as_deref().unwrap_or("tb");
    let result = bible::read(&book, chapter, version).await?;
    Ok(HttpResponse::Ok().json(result))
}