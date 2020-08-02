use std::io::Read;
use {
    argh::FromArgs,
    erased_serde::{Deserializer, Serialize as _, Serializer},
    serde_transcode::Transcoder,
    std::{
        fs::File,
        io::{stdin, stdout},
        path::PathBuf,
    },
    strum::EnumString,
    wyz::Pipe as _,
};

#[cfg(feature = "de-taml")]
use logos::Logos;
use taml::parser::{Taml, TamlValue};

#[derive(Debug, FromArgs)]
/// Transcode one self-describing format into another.
struct Args {
    #[argh(option, long = "if")]
    /// where to read input from. Defaults to stdin
    in_file: Option<PathBuf>,

    #[argh(option, long = "of")]
    /// where to write output to. Defaults to stdout
    out_file: Option<PathBuf>,

    //TODO: List In variant.
    #[argh(option, short = 'i', long = "in")]
    /// what to read
    in_format: In,

    //TODO: List Out variant.
    #[argh(option, short = 'o', long = "out")]
    /// what to write
    out_format: Out,

    #[argh(switch, short = 'p')]
    /// pretty-print (where supported)
    pretty: bool,
}

#[derive(Debug, EnumString)]
enum In {
    #[cfg(feature = "de-json")]
    #[strum(serialize = "json")]
    Json,

    #[cfg(feature = "de-taml")]
    #[strum(serialize = "taml")]
    Taml,
}

#[derive(Debug, EnumString)]
enum Out {
    #[cfg(feature = "ser-json")]
    #[strum(serialize = "json")]
    Json,
}

fn main() {
    let args: Args = argh::from_env();

    //TODO: Avoid leaking.

    let deserializer: Box<dyn Deserializer> = match args.in_format {
        #[cfg(feature = "de-json")]
        In::Json => args
            .in_file
            .map(|path| {
                File::open(path)
                    .unwrap()
                    .pipe(serde_json::Deserializer::from_reader)
                    .pipe(Box::new)
                    .pipe(Box::leak)
                    .pipe(Deserializer::erase)
                    .pipe(Box::new)
                    .pipe(|d| d as Box<dyn Deserializer>)
            })
            .unwrap_or_else(|| {
                stdin()
                    .pipe(serde_json::Deserializer::from_reader)
                    .pipe(Box::new)
                    .pipe(Box::leak)
                    .pipe(Deserializer::erase)
                    .pipe(Box::new)
                    .pipe(|d| d as Box<dyn Deserializer>)
            }),

        #[cfg(feature = "de-taml")]
        In::Taml => args
            .in_file
            .map(|path| {
                let diagnostics = vec![];
                let diagnostics = Box::new(diagnostics);
                let diagnostics = Box::leak(diagnostics);

                File::open(path)
                    .unwrap()
                    .pipe(|mut f| {
                        let mut text = String::new();
                        f.read_to_string(&mut text).unwrap();
                        text
                    })
                    .pipe(Box::new)
                    .pipe(Box::leak)
                    .pipe(|text| taml::token::Token::lexer(&*text))
                    .pipe(|tokens| taml::parser::parse(tokens, diagnostics).unwrap())
                    .pipe(|root_map| Taml {
                        span: ()..(),
                        value: TamlValue::Map(root_map),
                    })
                    .pipe(Box::new)
                    .pipe(Box::leak)
                    .pipe(move |root| taml::deserializer::Deserializer(&*root, diagnostics))
                    .pipe(Box::new)
                    .pipe(Box::leak)
                    .pipe(Deserializer::erase)
                    .pipe(Box::new)
                    .pipe(|d| d as Box<dyn Deserializer>)
            })
            .unwrap_or_else(|| {
                let diagnostics = vec![];
                let diagnostics = Box::new(diagnostics);
                let diagnostics = Box::leak(diagnostics);

                let mut text = String::new();
                stdin().read_to_string(&mut text).unwrap();
                let text = Box::new(text);
                let text = Box::leak(text);
                let lexer = taml::token::Token::lexer(text).spanned();
                let root_map = taml::parser::parse(lexer, diagnostics).unwrap();
                #[allow(clippy::reversed_empty_ranges)]
                let root = Taml {
                    span: 0..0,
                    value: TamlValue::Map(root_map),
                };
                let root = Box::new(root);
                let root = Box::leak(root);
                let deserializer = taml::deserializer::Deserializer(root, diagnostics);
                let deserializer = Box::new(deserializer);
                let deserializer = Box::leak(deserializer);
                let erased = Deserializer::erase(deserializer);
                Box::new(erased)
            }),
    };

    let pretty = args.pretty;
    let mut serializer: Box<dyn Serializer> = match args.out_format {
        #[cfg(feature = "ser-json")]
        Out::Json => args
            .out_file
            .map(|path| {
                File::create(path).unwrap().pipe(|file| {
                    if pretty {
                        file.pipe(serde_json::Serializer::pretty)
                            .pipe(Box::new)
                            .pipe(Box::leak)
                            .pipe(Serializer::erase)
                            .pipe(Box::new)
                            .pipe(|s| s as Box<dyn Serializer>)
                    } else {
                        file.pipe(serde_json::Serializer::new)
                            .pipe(Box::new)
                            .pipe(Box::leak)
                            .pipe(Serializer::erase)
                            .pipe(Box::new)
                            .pipe(|s| s as Box<dyn Serializer>)
                    }
                })
            })
            .unwrap_or_else(|| {
                if pretty {
                    stdout()
                        .pipe(serde_json::Serializer::pretty)
                        .pipe(Box::new)
                        .pipe(Box::leak)
                        .pipe(Serializer::erase)
                        .pipe(Box::new)
                        .pipe(|s| s as Box<dyn Serializer>)
                } else {
                    stdout()
                        .pipe(serde_json::Serializer::new)
                        .pipe(Box::new)
                        .pipe(Box::leak)
                        .pipe(Serializer::erase)
                        .pipe(Box::new)
                        .pipe(|s| s as Box<dyn Serializer>)
                }
            }),
    };

    Transcoder::new(deserializer)
        .erased_serialize(&mut serializer)
        .unwrap();

    println!()
}
