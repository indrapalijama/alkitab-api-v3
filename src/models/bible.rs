use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Bible book metadata
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BibleMetadata {
    /// Name of the Bible book
    pub book: String,
    /// Total number of verses in the book
    pub total_verse: usize,
    /// List of verse numbers in the book
    pub verses: Vec<i32>,
}

/// A single Bible verse
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Verse {
    /// Verse number
    pub verse: i32,
    /// Verse content
    pub content: String,
}

/// A complete Bible chapter
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BibleChapter {
    /// Book name(s)
    pub book: Vec<String>,
    /// Chapter number
    pub chapter: i32,
    /// Chapter title(s)
    pub title: Vec<String>,
    /// Total number of verses in the chapter
    pub total_verses: usize,
    /// Bible translation version (e.g., "TB", "AYT", "KJV")
    pub version: Option<String>,
    /// List of verses in the chapter
    pub verses: Vec<Verse>,
}

/// Search result
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SearchResult {
    /// Title of the search result
    pub title: String,
    /// URL of the search result
    pub url: String,
}

/// Error response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
}