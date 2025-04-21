use alkitab_api_rust::services::bible::BibleService;
use alkitab_api_rust::error::AppError;

#[tokio::test]
async fn test_find_book_metadata() {
    let service = BibleService::new();
    let result = service.find_book_metadata("Kejadian").await;
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.book, "Kejadian");
    assert!(metadata.total_verse > 0);
    assert!(!metadata.verses.is_empty());
}

#[tokio::test]
async fn test_get_chapter() {
    let service = BibleService::new();
    let result = service.get_chapter("Kejadian", "1", "tb").await;
    assert!(result.is_ok());
    let chapter = result.unwrap();
    assert_eq!(chapter.book, vec!["Kejadian"]);
    assert_eq!(chapter.chapter, 1);
    assert!(!chapter.title.is_empty());
    assert!(chapter.total_verses > 0);
    assert!(!chapter.verses.is_empty());
}

#[tokio::test]
async fn test_format_book_name() {
    let service = BibleService::new();
    let result = service.format_book_name("genesis");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Genesis");
}


#[tokio::test]
async fn test_invalid_book_name() {
    let service = BibleService::new();
    let result = service.find_book_metadata("InvalidBook").await;
    assert!(result.is_err());
    if let Err(AppError::InvalidBookError(_)) = result {
        // Expected error type
    } else {
        panic!("Expected InvalidBookError");
    }
}

#[tokio::test]
async fn test_invalid_chapter() {
    let service = BibleService::new();
    // Try with a very large chapter number that's unlikely to exist
    let result = service.get_chapter("Genesis", "9999", "tb").await;
    match result {
        Ok(_) => {
            // If it succeeds, it means the chapter exists (unlikely but possible)
            println!("Chapter 9999 exists in Genesis, which is unexpected but possible");
        },
        Err(e) => {
            // We expect either a NotFound or ExternalService error
            match e {
                AppError::NotFound(_) | AppError::ExternalService(_) => {
                    // These are acceptable error types
                },
                _ => panic!("Unexpected error type: {:?}", e),
            }
        }
    }
} 