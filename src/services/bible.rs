use regex::Regex;
use scraper::{Html, Selector};
use crate::models::bible::{BibleChapter, BibleMetadata, Verse};
use crate::error::AppError;
use crate::config::CONFIG;
use crate::models::book_translations::{translate_to_indonesian, get_short_name, SHORT_TO_INDONESIAN, INDONESIAN_TO_SHORT};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::time::Duration;

lazy_static! {
    static ref VERSE_NUMBER_REGEX: Regex = Regex::new(r"^(\d+)[\.\s]").unwrap();
    static ref STRONGS_NUMBER_REGEX: Regex = Regex::new(r"< \d+ >").unwrap();
    static ref MORPHOLOGICAL_TAG_REGEX: Regex = Regex::new(r"\(\d+\)").unwrap();
    static ref CONTENT_NUMBER_REGEX: Regex = Regex::new(r"\s+\d+\s+").unwrap();
    static ref VERSION_NAMES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("tb", "Alkitab Terjemahan Baru (TB)");
        m.insert("ayt", "Alkitab Yang Terbuka (AYT)");
        m.insert("kjv", "King James Version");
        m.insert("nkjv", "New King James Version");
        m.insert("niv", "New International Version");
        m.insert("esv", "English Standard Version");
        m.insert("nasb", "New American Standard Bible");
        m.insert("nlt", "New Living Translation");
        m
    };
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::builder()
        .pool_idle_timeout(Some(Duration::from_secs(30)))
        .pool_max_idle_per_host(10)
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap();
}

pub struct BibleService;

impl BibleService {

    pub fn new() -> Self {
        BibleService
    }


    pub async fn find_book_metadata(&self, book_name: &str) -> Result<BibleMetadata, AppError> {
        let indonesian_book_name = translate_to_indonesian(book_name)
            .ok_or_else(|| AppError::InvalidBookError(format!("Could not translate book name: {}", book_name)))?;
        
        let short_name = get_short_name(&indonesian_book_name)
            .ok_or_else(|| AppError::InvalidInput(format!("Could not get short name for book: {}", indonesian_book_name)))?;
        
        // Special handling for Mazmur
        let (url, pattern) = if indonesian_book_name == "Mazmur" {
            (
                format!("{}/tb/maz", CONFIG.bible.base_url),
                r#"href="[^"]*?/tb/Mzm/(\d+)/"#.to_string()
            )
        } else {
            (
                format!("{}/tb/{}", CONFIG.bible.base_url, short_name.to_lowercase()),
                format!(r#"href="[^"]*?/tb/{}/(\d+)/"#, short_name)
            )
        };
        
        let response = HTTP_CLIENT.get(&url)
            .send()
            .await
            .map_err(|e| AppError::ExternalApiError(e.to_string()))?
            .text()
            .await
            .map_err(|e| AppError::ExternalApiError(e.to_string()))?;
        
        let re = Regex::new(&pattern).unwrap();
        let mut verses: Vec<i32> = re.captures_iter(&response)
            .filter_map(|cap| cap.get(1))
            .filter_map(|m| m.as_str().parse::<i32>().ok())
            .collect();
        verses.sort_unstable();
        verses.dedup();

        Ok(BibleMetadata {
            book: indonesian_book_name.to_string(),
            total_verse: verses.len(),
            verses,
        })
    }


    fn clean_content(&self, content: &str, version: &str) -> String {
        let mut cleaned = content
            .replace("&quot;", "\"")
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&nbsp;", " ")
            .replace("&apos;", "'")
            .replace(" _", "")
            .replace("_ ", "")
            .replace("_", "");

        // Remove verse number from the beginning of the content
        cleaned = VERSE_NUMBER_REGEX.replace(&cleaned, "").to_string();

        // For non-TB versions, clean up numbers and tags
        if version != "tb" {
            cleaned = STRONGS_NUMBER_REGEX.replace_all(&cleaned, "").to_string();
            cleaned = MORPHOLOGICAL_TAG_REGEX.replace_all(&cleaned, "").to_string();
            cleaned = CONTENT_NUMBER_REGEX.replace_all(&cleaned, " ").to_string();
        }

        // Clean up any double spaces that might have been created
        cleaned.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    pub async fn get_chapter(&self, book: &str, chapter: &str, version: &str) -> Result<BibleChapter, AppError> {
        let book_lower = book.to_lowercase();
        let indonesian_book_name = {
            if let Some(name) = INDONESIAN_TO_SHORT.iter()
                .find(|(ind, _)| ind.to_lowercase() == book_lower)
                .map(|(ind, _)| ind.to_string()) {
                name
            }
            else if let Some(ind) = SHORT_TO_INDONESIAN.iter()
                .find(|(_, short)| short.to_lowercase() == book_lower)
                .map(|(ind, _)| ind.clone()) {
                ind
            }
            else if let Some(indonesian) = match book_lower.as_str() {
                "maz" => Some("Mazmur"),
                "kej" => Some("Kejadian"),
                "kel" => Some("Keluaran"),
                "im" => Some("Imamat"),
                "bil" => Some("Bilangan"),
                "ula" => Some("Ulangan"),
                "yos" => Some("Yosua"),
                "hak" => Some("Hakim-hakim"),
                "rut" => Some("Rut"),
                "1sa" => Some("1 Samuel"),
                "2sa" => Some("2 Samuel"),
                "1ra" => Some("1 Raja-raja"),
                "2ra" => Some("2 Raja-raja"),
                "1ta" => Some("1 Tawarikh"),
                "2ta" => Some("2 Tawarikh"),
                "ezr" => Some("Ezra"),
                "neh" => Some("Nehemia"),
                "est" => Some("Ester"),
                "ayb" => Some("Ayub"),
                "ams" => Some("Amsal"),
                "pkh" => Some("Pengkhotbah"),
                "kid" => Some("Kidung Agung"),
                "yes" => Some("Yesaya"),
                "yer" => Some("Yeremia"),
                "rat" => Some("Ratapan"),
                "yeh" => Some("Yehezkiel"),
                "dan" => Some("Daniel"),
                "hos" => Some("Hosea"),
                "yoe" => Some("Yoel"),
                "amo" => Some("Amos"),
                "oba" => Some("Obaja"),
                "yun" => Some("Yunus"),
                "mik" => Some("Mikha"),
                "nah" => Some("Nahum"),
                "hab" => Some("Habakuk"),
                "zef" => Some("Zefanya"),
                "hag" => Some("Hagai"),
                "zak" => Some("Zakharia"),
                "mal" => Some("Maleakhi"),
                "mat" => Some("Matius"),
                "mar" => Some("Markus"),
                "luk" => Some("Lukas"),
                "yoh" => Some("Yohanes"),
                "kis" => Some("Kisah Para Rasul"),
                "rom" => Some("Roma"),
                "1ko" => Some("1 Korintus"),
                "2ko" => Some("2 Korintus"),
                "gal" => Some("Galatia"),
                "efe" => Some("Efesus"),
                "fip" => Some("Filipi"),
                "kol" => Some("Kolose"),
                "1te" => Some("1 Tesalonika"),
                "2te" => Some("2 Tesalonika"),
                "1ti" => Some("1 Timotius"),
                "2ti" => Some("2 Timotius"),
                "tit" => Some("Titus"),
                "fim" => Some("Filemon"),
                "ibr" => Some("Ibrani"),
                "yak" => Some("Yakobus"),
                "1pe" => Some("1 Petrus"),
                "2pe" => Some("2 Petrus"),
                "1yo" => Some("1 Yohanes"),
                "2yo" => Some("2 Yohanes"),
                "3yo" => Some("3 Yohanes"),
                "yud" => Some("Yudas"),
                "why" => Some("Wahyu"),
                _ => None
            } {
                indonesian.to_string()
            }
            else if let Some(translated) = translate_to_indonesian(book) {
                translated.to_string()
            }
            else {
                return Err(AppError::InvalidInput(format!("Could not translate book name: {}", book)));
            }
        };
            
        let short_name = match get_short_name(&indonesian_book_name) {
            Some(short) => short,
            None => {
                return Err(AppError::InvalidInput(format!("Could not get short name for book: {}", indonesian_book_name)));
            }
        };

        let version = if version.is_empty() { "tb" } else { version };
        let version_name = if version == "tb" {
            Some("Alkitab Terjemahan Baru (TB)".to_string())
        } else {
            VERSION_NAMES.get(version).map(|&s| s.to_string())
        };
        let url = format!("{}/{}/{}/{}", CONFIG.bible.base_url, version, short_name, chapter);
        println!("Requesting URL: {}", url);

        let response = reqwest::get(&url).await.map_err(|_e| {
            AppError::ExternalService("Failed to fetch chapter".to_string())
        })?;

        if !response.status().is_success() {
            return Err(AppError::ExternalService(format!(
                "Failed to fetch chapter: HTTP {}",
                response.status()
            )));
        }

        let html = response.text().await.map_err(|_e| {
            AppError::ExternalService("Failed to get response text".to_string())
        })?;

        let document = Html::parse_document(&html);
        
        let mut titles_with_ranges: Vec<String> = Vec::new();
        let mut verses: Vec<Verse> = Vec::new();
        
        // Different versions have different HTML structures
        let (verse_selector, ref_selector, content_selector) = match version {
            "tb" | "ayt" => ("p", "span.reftext", "span[data-dur]"),
            "kjv" | "niv" | "esv" | "nasb" | "nlt" => ("p", "span.reftext", "span[data-dur]"),
            _ => ("p", "span.reftext", "span[data-dur]") // Default to the same selectors
        };

        if let Some(selector) = Selector::parse(verse_selector).ok() {
            let mut current_title: Option<String> = None;
            let mut current_start_verse: Option<i32> = None;
            let mut last_verse_number: Option<i32> = None;
            
            for element in document.select(&selector) {
                // Try to find paragraph titles if they exist
                if let Some(title_span) = element.select(&Selector::parse("span.paragraphtitle").unwrap()).next() {
                    if let Some(title_text) = title_span.text().next() {
                        let title_text = title_text.trim();
                        if !title_text.is_empty() {
                            if let (Some(prev_title), Some(start_verse)) = (current_title.take(), current_start_verse.take()) {
                                if let Some(last_verse) = last_verse_number {
                                    titles_with_ranges.push(format!("{} ({}-{})", prev_title, start_verse, last_verse));
                                }
                            }
                            current_title = Some(title_text.to_string());
                        }
                    }
                }

                // Try to find verse references
                if let Some(ref_span) = element.select(&Selector::parse(ref_selector).unwrap()).next() {
                    if let Some(verse_num_text) = ref_span.text().next() {
                        let verse_number = if let Some(captures) = VERSE_NUMBER_REGEX.captures(verse_num_text) {
                            captures.get(1)
                                .map(|m| m.as_str().parse::<i32>().unwrap_or(1))
                                .unwrap_or(1)
                        } else {
                            verse_num_text
                                .chars()
                                .filter(|c| c.is_digit(10))
                                .collect::<String>()
                                .parse::<i32>()
                                .unwrap_or(1)
                        };

                        if current_start_verse.is_none() {
                            current_start_verse = Some(verse_number);
                        }
                        
                        // Try to find verse content
                        let content = if let Some(content_span) = element.select(&Selector::parse(content_selector).unwrap()).next() {
                            if let Some(content) = content_span.text().next() {
                                Some(content.trim().to_string())
                            } else {
                                None
                            }
                        } else {
                            // If the specific selector doesn't work, try to get the text directly from the paragraph
                            // This is a fallback for versions with different HTML structures
                            let text = element.text().collect::<Vec<&str>>().join(" ");
                            if !text.trim().is_empty() {
                                Some(text.trim().to_string())
                            } else {
                                None
                            }
                        };

                        if let Some(content) = content {
                            if !content.is_empty() {
                                let content = self.clean_content(&content, version);

                                verses.push(Verse {
                                    verse: verse_number,
                                    content,
                                });
                                last_verse_number = Some(verse_number);
                            }
                        }
                    }
                }
            }

            if let (Some(title), Some(start_verse)) = (current_title, current_start_verse) {
                if let Some(last_verse) = last_verse_number {
                    titles_with_ranges.push(format!("{} ({}-{})", title, start_verse, last_verse));
                }
            }
        }

        if verses.is_empty() {
            return Err(AppError::NotFound("No verses found in chapter".to_string()));
        }

        if titles_with_ranges.is_empty() {
            titles_with_ranges.push(format!("{} {}", indonesian_book_name, chapter));
        }

        Ok(BibleChapter {
            book: vec![indonesian_book_name],
            chapter: chapter.parse().unwrap_or(1),
            title: titles_with_ranges,
            total_verses: verses.len(),
            version: version_name,
            verses,
        })
    }


    pub fn format_book_name(&self, book_name: &str) -> Result<String, AppError> {
        if book_name.is_empty() {
            return Err(AppError::InvalidBookError("Empty book name".to_string()));
        }
        
        let first_char = book_name.chars().next()
            .ok_or_else(|| AppError::InvalidBookError("Empty book name".to_string()))?
            .to_uppercase()
            .to_string();
        let rest = book_name.chars().skip(1).collect::<String>().to_lowercase();
        
        Ok(format!("{}{}", first_char, rest))
    }

}

pub async fn find(book: &str) -> Result<BibleMetadata, AppError> {
    let bible_service = BibleService::new();
    let formatted_book_name = bible_service.format_book_name(book.trim())?;
    bible_service.find_book_metadata(&formatted_book_name).await
}

pub async fn read(book: &str, chapter: i32, version: &str) -> Result<BibleChapter, AppError> {
    let bible_service = BibleService::new();
    let formatted_book_name = bible_service.format_book_name(book.trim())?;
    bible_service.get_chapter(&formatted_book_name, &chapter.to_string(), version).await
} 