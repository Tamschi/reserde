use std::io::Read;
use {
    argh::FromArgs,
    serde_any::Object,
    std::{
        fs::File,
        io::{stdin, stdout},
        path::PathBuf,
    },
    strum::EnumString,
};

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

    #[cfg(feature = "de-yaml")]
    #[strum(serialize = "yaml")]
    Yaml,
}

#[derive(Debug, EnumString)]
enum Out {
    #[cfg(feature = "ser-json")]
    #[strum(serialize = "json")]
    Json,

    #[cfg(feature = "ser-yaml")]
    #[strum(serialize = "yaml")]
    Yaml,
}

fn main() {
    let args: Args = argh::from_env();

    //TODO: Avoid leaking.

    let object: Object = match args.in_format {
        #[cfg(feature = "de-json")]
        In::Json => {
            let mut text = String::new();
            if let Some(path) = args.in_file {
                File::open(path).unwrap().read_to_string(&mut text).unwrap();
            } else {
                stdin().read_to_string(&mut text).unwrap();
            }
            let object: Object = serde_json::from_str(&text).unwrap();
            object.into_owned()
        }

        #[cfg(feature = "de-taml")]
        In::Taml => {
            let diagnostics = vec![];
            let diagnostics = Box::new(diagnostics);
            let diagnostics = Box::leak(diagnostics);
            let mut text = String::new();
            if let Some(path) = args.in_file {
                File::open(path).unwrap().read_to_string(&mut text).unwrap();
            } else {
                stdin().read_to_string(&mut text).unwrap();
            }
            let object: Object = taml::deserializer::from_str(&text, diagnostics).unwrap();
            object.into_owned()
        }

        #[cfg(feature = "de-yaml")]
        In::Yaml => {
            let mut text = String::new();
            if let Some(path) = args.in_file {
                File::open(path).unwrap().read_to_string(&mut text).unwrap();
            } else {
                stdin().read_to_string(&mut text).unwrap();
            }
            todo!("YAML doesn't support borrowing deserialization. Find a workaround.");
            // let object: Object = serde_yaml::from_str(&text).unwrap();
            // object.into_owned()
        }
    };

    let pretty = args.pretty;
    match args.out_format {
        #[cfg(feature = "ser-json")]
        Out::Json => {
            if let Some(path) = args.out_file {
                let file = File::create(path).unwrap();
                if pretty {
                    serde_json::to_writer_pretty(file, &object).unwrap()
                } else {
                    serde_json::to_writer(file, &object).unwrap()
                }
            } else if pretty {
                serde_json::to_writer_pretty(stdout(), &object).unwrap()
            } else {
                serde_json::to_writer(stdout(), &object).unwrap()
            }
        }

        #[cfg(feature = "ser-yaml")]
        Out::Yaml => {
            if let Some(path) = args.out_file {
                let file = File::create(path).unwrap();
                serde_yaml::to_writer(file, &object).unwrap()
            } else {
                serde_yaml::to_writer(stdout(), &object).unwrap()
            }
        }
    };

    // Flush stdout.
    println!()
}
