

pub struct LangConfig {
    pub source: &'static str,
    pub run: Vec<&'static str>
}


pub fn get_lang_config(lang: &str) -> LangConfig {
    match lang {
        "python" => LangConfig {
            source: "main.py",
            run: vec!["/usr/bin/python3", "main.py"],
        },
        _ => panic!("Unsupported language")
    }
}
