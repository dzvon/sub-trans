pub(crate) mod deepl;

use crate::error::Error;
use clap::ArgEnum;
use std::fmt;

#[derive(ArgEnum, Clone, Debug)]
pub(crate) enum TranslateService {
    DeepL,
}

// For simplicity, This item is extract from DeepL docs.
// https://www.deepl.com/docs-api/translating-text/
#[derive(ArgEnum, Clone, Copy, Debug)]
pub(crate) enum Language {
    BG,   // - Bulgarian
    CS,   // - Czech
    DA,   // - Danish
    DE,   // - German
    EL,   // - Greek
    EnGb, // - English (British)
    EnUs, // - English (American)
    EN, // - English (unspecified variant for backward compatibility; please select EN-GB or EN-US instead)
    ES, // - Spanish
    ET, // - Estonian
    FI, // - Finnish
    FR, // - French
    HU, // - Hungarian
    IT, // - Italian
    JA, // - Japanese
    LT, // - Lithuanian
    LV, // - Latvian
    NL, // - Dutch
    PL, // - Polish
    PtPt, // - Portuguese (all Portuguese varieties excluding Brazilian Portuguese)
    PtBr, // - Portuguese (Brazilian)
    PT, // - Portuguese (unspecified variant for backward compatibility; please select PT-PT or PT-BR instead)
    RO, // - Romanian
    RU, // - Russian
    SK, // - Slovak
    SL, // - Slovenian
    SV, // - Swedish
    ZH, // - Chinese
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

pub(crate) struct Param {
    pub texts: Vec<String>,
    pub source_lang: Option<Language>,
    pub target_lang: Language,
}

#[derive(Default)]
pub(crate) struct Response {
    pub source_lang: String,
    pub texts: Vec<String>,
}

pub(crate) trait Translator {
    fn translate(&self, param: Param) -> Result<Response, Error>;
}
