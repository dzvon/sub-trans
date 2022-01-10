# sub-trans

A tool for translate subtitle to other language.

Currently under active development, there are many features that have not been implemented.

```
sub-trans 0.1.0

USAGE:
    sub-trans [OPTIONS] <SUBTITLE_PATH>

ARGS:
    <SUBTITLE_PATH>
            Path of the subtitle file which will to be translated

OPTIONS:
    -h, --help
            Print help information

    -s, --source-lang <SOURCE_LANG>
            Language of the subtitle to be translated

            [possible values: bg, cs, da, de, el, en-gb, en-us, en, es, et, fi, fr, hu, it, ja, lt,
            lv, nl, pl, pt-pt, pt-br, pt, ro, ru, sk, sl, sv, zh]

    -t, --target-lang <TARGET_LANG>
            Target language you want to translate to

            [default: en]
            [possible values: bg, cs, da, de, el, en-gb, en-us, en, es, et, fi, fr, hu, it, ja, lt,
            lv, nl, pl, pt-pt, pt-br, pt, ro, ru, sk, sl, sv, zh]

        --translate-service <TRANSLATE_SERVICE>
            Which translate service you wish to use, only support DeepL at the moment. Please set
            proper environment variable to make the service avaliable. --help for more detail

            DeepL: DEEPL_AUTH_KEY

            [default: deep-l]
            [possible values: deep-l]

    -V, --version
            Print version information
```
