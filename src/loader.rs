use std::collections::HashMap;
use crate::models::LangConfig;


use once_cell::sync::Lazy;
use std::fs;

static LANG_CONFIG: Lazy<HashMap<String, LangConfig>> = Lazy::new(|| {
    let text = fs::read_to_string("lang_config.json")
        .expect("lang_config.json missing");

    serde_json::from_str(&text.as_str())
        .expect("invalid lang_config.json")
});

pub fn get_lang_config(lang: &str) -> LangConfig {
    LANG_CONFIG.get(lang).cloned().unwrap_or_else(|| panic!("Unsupported language"))
}
