use crate::error::Error;
use crate::translator::{Language, Param, Response, Translator};
use std::fs;
use std::path::Path;
use subparse::{timetypes::TimeSpan, SrtFile, SubtitleFile, SubtitleFileInterface, SubtitleFormat};

#[derive(Clone)]
pub(crate) struct Subtitle(pub SubtitleFile);

impl Subtitle {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let file = fs::read(path).unwrap();

        let sub_format = subparse::get_subtitle_format_err(path.extension(), &file)
            .map_err(|e| Error::SubtitleDecodeError(e.to_string()))?;

        let sub = match sub_format {
            SubtitleFormat::SubRip => {
                // Only support UTF-8 for now
                SubtitleFile::SubRipFile(
                    SrtFile::parse(&String::from_utf8_lossy(&file))
                        .map_err(|e| Error::SubtitleDecodeError(e.to_string()))?,
                )
            }
            SubtitleFormat::SubStationAlpha => todo!(),
            SubtitleFormat::VobSubIdx => todo!(),
            SubtitleFormat::VobSubSub => todo!(),
            SubtitleFormat::MicroDVD => todo!(),
        };

        Ok(Subtitle(sub))
    }

    pub fn translate(
        &self,
        svc: impl Translator,
        source_lang: Option<Language>,
        target_lang: Language,
    ) -> Result<(String, Vec<u8>), Error> {
        let entries = self
            .0
            .get_subtitle_entries()
            .map_err(|e| Error::SubtitleDecodeError(e.to_string()))?;

        let texts: Vec<String> = entries
            .iter()
            .map(|t| t.line.as_ref().map(|s| s.to_string()).unwrap_or_default())
            .collect();

        let mut translated_text = Response {
            source_lang: source_lang.as_ref().unwrap_or(&Language::EN).to_string(),
            texts: Vec::new(),
        };

        // Up to 50 text parameters can be submitted in one request.
        // We choose 30 text per request.
        for text in texts.chunks(30) {
            let param = Param {
                texts: text
                    .iter()
                    .map(|s| {
                        let mut ss = s.clone();
                        ss.retain(|c| c != '\n');
                        ss
                    })
                    .collect(),
                source_lang,
                target_lang,
            };
            let mut response = svc.translate(param)?;

            translated_text.source_lang = response.source_lang.to_string();
            translated_text.texts.append(&mut response.texts);
        }

        let translated_sub: Vec<(TimeSpan, String)> = entries
            .into_iter()
            .zip(translated_text.texts.into_iter())
            .map(|(x, y)| {
                (
                    x.timespan,
                    // return bilingual subtitle, original language first
                    x.line.map_or(y.clone(), |u| format!("{u}\n{y}")),
                )
            })
            .collect();

        let srt = SrtFile::create(translated_sub)
            .map_err(|e| Error::SubtitleDecodeError(format!("error in create srtfile, {e}")))?
            .to_data()
            .map_err(|e| Error::SubtitleDecodeError(e.to_string()))?;

        Ok((translated_text.source_lang, srt))
    }
}
