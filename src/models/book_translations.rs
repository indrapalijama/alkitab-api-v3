use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ENGLISH_TO_INDONESIAN: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Genesis", "Kejadian");
        m.insert("Exodus", "Keluaran");
        m.insert("Leviticus", "Imamat");
        m.insert("Numbers", "Bilangan");
        m.insert("Deuteronomy", "Ulangan");
        m.insert("Joshua", "Yosua");
        m.insert("Judges", "Hakim-hakim");
        m.insert("Ruth", "Rut");
        m.insert("1 Samuel", "1 Samuel");
        m.insert("2 Samuel", "2 Samuel");
        m.insert("1 Kings", "1 Raja-raja");
        m.insert("2 Kings", "2 Raja-raja");
        m.insert("1 Chronicles", "1 Tawarikh");
        m.insert("2 Chronicles", "2 Tawarikh");
        m.insert("Ezra", "Ezra");
        m.insert("Nehemiah", "Nehemia");
        m.insert("Esther", "Ester");
        m.insert("Job", "Ayub");
        m.insert("Psalms", "Mazmur");
        m.insert("Proverbs", "Amsal");
        m.insert("Ecclesiastes", "Pengkhotbah");
        m.insert("Song of Solomon", "Kidung Agung");
        m.insert("Isaiah", "Yesaya");
        m.insert("Jeremiah", "Yeremia");
        m.insert("Lamentations", "Ratapan");
        m.insert("Ezekiel", "Yehezkiel");
        m.insert("Daniel", "Daniel");
        m.insert("Hosea", "Hosea");
        m.insert("Joel", "Yoel");
        m.insert("Amos", "Amos");
        m.insert("Obadiah", "Obaja");
        m.insert("Jonah", "Yunus");
        m.insert("Micah", "Mikha");
        m.insert("Nahum", "Nahum");
        m.insert("Habakkuk", "Habakuk");
        m.insert("Zephaniah", "Zefanya");
        m.insert("Haggai", "Hagai");
        m.insert("Zechariah", "Zakharia");
        m.insert("Malachi", "Maleakhi");
        m.insert("Matthew", "Matius");
        m.insert("Mark", "Markus");
        m.insert("Luke", "Lukas");
        m.insert("John", "Yohanes");
        m.insert("Acts", "Kisah Para Rasul");
        m.insert("Romans", "Roma");
        m.insert("1 Corinthians", "1 Korintus");
        m.insert("2 Corinthians", "2 Korintus");
        m.insert("Galatians", "Galatia");
        m.insert("Ephesians", "Efesus");
        m.insert("Philippians", "Filipi");
        m.insert("Colossians", "Kolose");
        m.insert("1 Thessalonians", "1 Tesalonika");
        m.insert("2 Thessalonians", "2 Tesalonika");
        m.insert("1 Timothy", "1 Timotius");
        m.insert("2 Timothy", "2 Timotius");
        m.insert("Titus", "Titus");
        m.insert("Philemon", "Filemon");
        m.insert("Hebrews", "Ibrani");
        m.insert("James", "Yakobus");
        m.insert("1 Peter", "1 Petrus");
        m.insert("2 Peter", "2 Petrus");
        m.insert("1 John", "1 Yohanes");
        m.insert("2 John", "2 Yohanes");
        m.insert("3 John", "3 Yohanes");
        m.insert("Jude", "Yudas");
        m.insert("Revelation", "Wahyu");
        m
    };

    pub static ref INDONESIAN_TO_ENGLISH: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        for (en, id) in ENGLISH_TO_INDONESIAN.iter() {
            m.insert(*id, *en);
        }
        m
    };

    pub static ref INDONESIAN_TO_SHORT: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Kejadian", "Kej");
        m.insert("Keluaran", "Kel");
        m.insert("Imamat", "Im");
        m.insert("Bilangan", "Bil");
        m.insert("Ulangan", "Ula");
        m.insert("Yosua", "Yos");
        m.insert("Hakim-hakim", "Hak");
        m.insert("Rut", "Rut");
        m.insert("1 Samuel", "1Sa");
        m.insert("2 Samuel", "2Sa");
        m.insert("1 Raja-raja", "1Ra");
        m.insert("2 Raja-raja", "2Ra");
        m.insert("1 Tawarikh", "1Ta");
        m.insert("2 Tawarikh", "2Ta");
        m.insert("Ezra", "Eza");
        m.insert("Nehemia", "Neh");
        m.insert("Ester", "Est");
        m.insert("Ayub", "Ayu");
        m.insert("Mazmur", "Maz");
        m.insert("Amsal", "Ams");
        m.insert("Pengkhotbah", "Pkh");
        m.insert("Kidung Agung", "Kid");
        m.insert("Yesaya", "Yes");
        m.insert("Yeremia", "Yer");
        m.insert("Ratapan", "Rat");
        m.insert("Yehezkiel", "Yeh");
        m.insert("Daniel", "Dan");
        m.insert("Hosea", "Hos");
        m.insert("Yoel", "Yoe");
        m.insert("Amos", "Amo");
        m.insert("Obaja", "Oba");
        m.insert("Yunus", "Yun");
        m.insert("Mikha", "Mik");
        m.insert("Nahum", "Nah");
        m.insert("Habakuk", "Hab");
        m.insert("Zefanya", "Zef");
        m.insert("Hagai", "Hag");
        m.insert("Zakharia", "Zak");
        m.insert("Maleakhi", "Mal");
        m.insert("Matius", "Mat");
        m.insert("Markus", "Mar");
        m.insert("Lukas", "Luk");
        m.insert("Yohanes", "Yoh");
        m.insert("Kisah Para Rasul", "Kis");
        m.insert("Roma", "Rom");
        m.insert("1 Korintus", "1Ko");
        m.insert("2 Korintus", "2Ko");
        m.insert("Galatia", "Gal");
        m.insert("Efesus", "Efe");
        m.insert("Filipi", "Fip");
        m.insert("Kolose", "Kol");
        m.insert("1 Tesalonika", "1Te");
        m.insert("2 Tesalonika", "2Te");
        m.insert("1 Timotius", "1Ti");
        m.insert("2 Timotius", "2Ti");
        m.insert("Titus", "Tit");
        m.insert("Filemon", "Fim");
        m.insert("Ibrani", "Ibr");
        m.insert("Yakobus", "Yak");
        m.insert("1 Petrus", "1Pe");
        m.insert("2 Petrus", "2Pe");
        m.insert("1 Yohanes", "1Yo");
        m.insert("2 Yohanes", "2Yo");
        m.insert("3 Yohanes", "3Yo");
        m.insert("Yudas", "Yud");
        m.insert("Wahyu", "Wah");
        m
    };

    pub static ref SHORT_TO_INDONESIAN: HashMap<String, &'static str> = {
        let mut m = HashMap::new();
        for (ind, short) in INDONESIAN_TO_SHORT.iter() {
            m.insert(short.to_lowercase(), *ind);
            // Add alternative abbreviations
            if *ind == "Mazmur" {
                m.insert("mzm".to_string(), *ind);
            }
        }
        m
    };
}

pub fn translate_to_indonesian(input: &str) -> Option<&'static str> {
    let input_lower = input.to_lowercase();
    
    // First check if the input is a short name/abbreviation
    if let Some(indonesian) = SHORT_TO_INDONESIAN.get(&input_lower) {
        return Some(indonesian);
    }
    
    // Then check if the input is already in Indonesian
    for (id, _) in INDONESIAN_TO_ENGLISH.iter() {
        if id.to_lowercase() == input_lower {
            return Some(id);
        }
    }
    
    // Then try exact match for English to Indonesian
    if let Some(translation) = ENGLISH_TO_INDONESIAN.get(input) {
        return Some(translation);
    }
    
    // Try to find a match by prefix
    for (en, id) in ENGLISH_TO_INDONESIAN.iter() {
        if en.to_lowercase().starts_with(&input_lower) || input_lower.starts_with(&en.to_lowercase()) {
            return Some(id);
        }
    }
    
    // If still no match, try to find a match by substring
    for (en, id) in ENGLISH_TO_INDONESIAN.iter() {
        if en.to_lowercase().contains(&input_lower) || input_lower.contains(&en.to_lowercase()) {
            return Some(id);
        }
    }
    
    None
}

#[allow(dead_code)]
pub fn translate_to_english(indonesian: &str) -> Option<&'static str> {
    INDONESIAN_TO_ENGLISH.get(indonesian).copied()
}

pub fn get_short_name(indonesian: &str) -> Option<&'static str> {
    let indonesian_lower = indonesian.to_lowercase();
    
    // Try case-insensitive lookup
    for (key, value) in INDONESIAN_TO_SHORT.iter() {
        if key.to_lowercase() == indonesian_lower {
            return Some(value);
        }
    }
    
    None
} 