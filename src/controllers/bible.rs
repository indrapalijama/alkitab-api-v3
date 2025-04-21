use actix_web::{web, HttpResponse};
use crate::error::AppError;
use crate::services::bible::BibleService;

pub async fn find(book: web::Path<String>) -> Result<HttpResponse, AppError> {
    let bible_service = BibleService::new();
    let formatted_book_name = bible_service.format_book_name(&book)?;
    let metadata = bible_service.find_book_metadata(&formatted_book_name).await?;    
    Ok(HttpResponse::Ok().json(metadata))
}

#[derive(serde::Deserialize)]
pub struct ReadQuery {
    version: Option<String>,
}

pub async fn read(path: web::Path<(String, i32)>, query: web::Query<ReadQuery>) -> Result<HttpResponse, AppError> {
    let (book, chapter) = path.into_inner();
    
    let bible_service = BibleService::new();
    let formatted_book_name = bible_service.format_book_name(&book)?;
    let chapter_data = bible_service.get_chapter(&formatted_book_name, &chapter.to_string(), query.version.as_deref().unwrap_or("tb")).await?;
    
    Ok(HttpResponse::Ok().json(chapter_data))
}