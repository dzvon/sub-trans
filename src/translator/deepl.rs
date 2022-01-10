use crate::{
    error::Error,
    translator::{self, Response},
};
use reqwest::blocking::Client;
use serde::Deserialize;

pub(crate) struct DeepL {
    auth_key: String,
}

impl DeepL {
    pub fn new(auth_key: String) -> Self {
        Self { auth_key }
    }
}

#[derive(Deserialize)]
struct DeeplResponse {
    translations: Vec<TranslatedText>,
}

#[derive(Deserialize)]
struct TranslatedText {
    detected_source_language: String,
    text: String,
}

impl translator::Translator for DeepL {
    fn translate(&self, param: translator::Param) -> Result<translator::Response, Error> {
        let url = if self.auth_key.ends_with(":fx") {
            "https://api-free.deepl.com/v2/translate"
        } else {
            "https://api.deepl.com/v2/translate"
        };

        let client = Client::new();

        let mut query = Vec::new();

        query.push(("auth_key", self.auth_key.clone()));

        if let Some(lang) = param.source_lang {
            query.push(("source_lang", language_to_deepl_str(lang)));
        }

        query.push(("target_lang", language_to_deepl_str(param.target_lang)));
        for text in param.texts {
            query.push(("text", text));
        }
        // in order to prevent deepl from splitting the sentence unintentionally.
        query.push(("split_sentences", "0".to_string()));

        let response: Response = client
            .post(url)
            .query(&query)
            .send()?
            .json::<DeeplResponse>()?
            .into();

        Ok(response)
    }
}

impl From<DeeplResponse> for Response {
    fn from(r: DeeplResponse) -> Self {
        let mut response = Response::default();

        let mut detected_source_lang_count = std::collections::HashMap::new();

        for t in r.translations {
            *detected_source_lang_count
                .entry(t.detected_source_language)
                .or_insert(0) += 1;
            response.texts.push(t.text);
        }

        let srouce_lang = detected_source_lang_count
            .into_iter()
            .max_by(|x, y| x.1.cmp(&y.1))
            .map(|(k, _v)| k);

        response.source_lang = srouce_lang.unwrap_or_default();

        response
    }
}

fn language_to_deepl_str(lang: translator::Language) -> String {
    match lang {
        translator::Language::BG => String::from("BG"),
        translator::Language::CS => String::from("CS"),
        translator::Language::DA => String::from("DA"),
        translator::Language::DE => String::from("DE"),
        translator::Language::EL => String::from("EL"),
        translator::Language::EnGb => String::from("EN-GB"),
        translator::Language::EnUs => String::from("EN-US"),
        translator::Language::EN => String::from("EN"),
        translator::Language::ES => String::from("ES"),
        translator::Language::ET => String::from("ET"),
        translator::Language::FI => String::from("FI"),
        translator::Language::FR => String::from("FR"),
        translator::Language::HU => String::from("HU"),
        translator::Language::IT => String::from("IT"),
        translator::Language::JA => String::from("JA"),
        translator::Language::LT => String::from("LT"),
        translator::Language::LV => String::from("LV"),
        translator::Language::NL => String::from("NL"),
        translator::Language::PL => String::from("PL"),
        translator::Language::PtPt => String::from("PT-PT"),
        translator::Language::PtBr => String::from("PT-BR"),
        translator::Language::PT => String::from("PT"),
        translator::Language::RO => String::from("RO"),
        translator::Language::RU => String::from("RU"),
        translator::Language::SK => String::from("SK"),
        translator::Language::SL => String::from("SL"),
        translator::Language::SV => String::from("SV"),
        translator::Language::ZH => String::from("ZH"),
    }
}
