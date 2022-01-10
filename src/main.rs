mod error;
mod subtitle;
mod translator;

use clap::Parser;
use std::io::Write;
use std::path::PathBuf;
use translator::{deepl::DeepL, Language, TranslateService};

#[derive(Debug, Parser)]
#[clap(about, version, author)]
struct Args {
    /// Path of the subtitle file which will to be translated
    #[clap(parse(from_os_str))]
    subtitle_path: PathBuf,

    /// Language of the subtitle to be translated
    #[clap(short, long, arg_enum)]
    source_lang: Option<Language>,

    /// Target language you want to translate to
    #[clap(arg_enum, short, long, default_value = "en")]
    target_lang: Language,

    /// Which translate service you wish to use, only support DeepL at the moment.
    /// Please set proper environment variable to make the service avaliable. --help
    /// for more detail
    ///
    /// DeepL: DEEPL_AUTH_KEY
    #[clap(arg_enum, long, default_value = "deep-l")]
    translate_service: TranslateService,
}

fn main() {
    let args = Args::parse();

    let translator = match args.translate_service {
        TranslateService::DeepL => match std::env::var("DEEPL_AUTH_KEY") {
            Ok(auth_key) => DeepL::new(auth_key),
            Err(_) => {
                eprintln!("DeepL auth key should be set, via DEEPL_AUTH_KEY environment variable.");
                std::process::exit(64);
            }
        },
    };

    match subtitle::Subtitle::new(&args.subtitle_path) {
        Ok(sub) => {
            let (source_lang, srt) = sub
                .translate(translator, args.source_lang, args.target_lang)
                .expect("something is wrong when translating.");

            let mut translated_file_path = args.subtitle_path.clone();
            translated_file_path.set_extension(format!(
                "{}&{}.srt",
                source_lang.to_lowercase(),
                args.target_lang
            ));

            let mut translated_file = std::fs::File::create(&translated_file_path).unwrap();
            translated_file.write_all(&srt).unwrap();

            println!("translated file path: {}", translated_file_path.display());
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
