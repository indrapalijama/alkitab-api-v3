use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BibleMetadata {
    pub book: String,
    pub total_verse: usize,
    pub verses: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Verse {
    pub verse: i32,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BibleChapter {
    pub book: Vec<String>,
    pub chapter: i32,
    pub title: Vec<String>,
    pub total_verses: usize,
    pub version: Option<String>,
    pub verses: Vec<Verse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}