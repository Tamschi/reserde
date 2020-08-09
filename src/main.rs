use {
    argh::FromArgs,
    serde_detach::detach,
    serde_object::Object,
    std::{
        borrow::Cow,
        fs::File,
        io::{stdin, stdout, Read as _, Write as _},
        path::PathBuf,
    },
    strum::EnumString,
    wyz::Pipe as _,
};

#[derive(Debug, FromArgs)]
/// Transcode one self-describing format into another.
///
/// Currently supports Bencode, CBOR, JSON (--pretty), TAML (--in only), XML, x-www-form-urlencoded (as urlencoded) and YAML.
/// All names are lowercase.
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

    #[cfg(feature = "stringify")]
    #[argh(option, short = 's')]
    /// stringify bytes and non-string value keys into strings where possible, possible values are: utf8. (Tries encodings in the order specified.) [try with: --in bencode]
    stringify: Vec<Encoding>,

    #[cfg(feature = "enum-bools")]
    #[argh(switch)]
    /// case-insensitively convert unit variants with name `true` or `false` into booleans. [try with: --in taml]
    enum_bools: bool,
}

#[derive(Debug, EnumString, Clone, Copy)]
enum In {
    #[cfg(feature = "de-bencode")]
    #[strum(serialize = "bencode")]
    Bencode,

    #[cfg(feature = "de-cbor")]
    #[strum(serialize = "cbor")]
    Cbor,

    #[cfg(feature = "de-json")]
    #[strum(serialize = "json")]
    Json,

    #[cfg(feature = "de-taml")]
    #[strum(serialize = "taml")]
    Taml,

    #[cfg(feature = "de-urlencoded")]
    #[strum(serialize = "urlencoded")]
    Urlencoded,

    #[cfg(feature = "de-xml")]
    #[strum(serialize = "xml")]
    Xml,

    #[cfg(feature = "de-yaml")]
    #[strum(serialize = "yaml")]
    Yaml,
}

#[derive(Debug, EnumString, Clone, Copy)]
enum Out {
    #[cfg(feature = "ser-bencode")]
    #[strum(serialize = "bencode")]
    Bencode,

    #[cfg(feature = "ser-cbor")]
    #[strum(serialize = "cbor")]
    Cbor,

    #[cfg(feature = "ser-json")]
    #[strum(serialize = "json")]
    Json,

    #[cfg(feature = "ser-urlencoded")]
    #[strum(serialize = "urlencoded")]
    Urlencoded,

    #[cfg(feature = "ser-xml")]
    #[strum(serialize = "xml")]
    Xml,

    #[cfg(feature = "ser-yaml")]
    #[strum(serialize = "yaml")]
    Yaml,
}

#[cfg(feature = "stringify")]
#[derive(Debug, EnumString, Clone, Copy)]
enum Encoding {
    #[strum(serialize = "utf8")]
    Utf8,
}

fn main() {
    let args: Args = argh::from_env();

    //TODO: Avoid leaking.

    let mut object: Object = match args.in_format {
        #[cfg(feature = "de-bencode")]
        In::Bencode => {
            let mut data = vec![];
            if let Some(path) = args.in_file {
                File::open(path).unwrap().read_to_end(&mut data).unwrap();
            } else {
                stdin().read_to_end(&mut data).unwrap();
            }
            serde_bencode::from_bytes(&data).map(detach).unwrap()
        }

        #[cfg(feature = "de-cbor")]
        In::Cbor => if let Some(path) = args.in_file {
            File::open(path)
                .unwrap()
                .pipe(serde_cbor::from_reader)
                .map(detach)
        } else {
            stdin().pipe(serde_cbor::from_reader).map(detach)
        }
        .unwrap(),

        #[cfg(feature = "de-json")]
        In::Json => {
            let mut text = String::new();
            if let Some(path) = args.in_file {
                File::open(path).unwrap().read_to_string(&mut text).unwrap();
            } else {
                stdin().read_to_string(&mut text).unwrap();
            }
            serde_json::from_str(&text).map(detach).unwrap()
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
            taml::deserializer::from_str(&text, diagnostics)
                .map(detach)
                .unwrap()
        }

        #[cfg(feature = "de-urlencoded")]
        In::Urlencoded => if let Some(path) = args.in_file {
            File::open(path)
                .unwrap()
                .pipe(serde_urlencoded::from_reader)
                .map(detach)
        } else {
            stdin().pipe(serde_urlencoded::from_reader).map(detach)
        }
        .unwrap(),

        #[cfg(feature = "de-xml")]
        In::Xml => {
            let mut text = String::new();
            if let Some(path) = args.in_file {
                File::open(path).unwrap().read_to_string(&mut text).unwrap();
            } else {
                stdin().read_to_string(&mut text).unwrap();
            }
            quick_xml::de::from_str(&text).map(detach).unwrap()
        }

        #[cfg(feature = "de-yaml")]
        In::Yaml => {
            let mut text = String::new();
            if let Some(path) = args.in_file {
                File::open(path).unwrap().read_to_string(&mut text).unwrap();
            } else {
                stdin().read_to_string(&mut text).unwrap();
            }
            serde_yaml::from_str(&text).map(detach).unwrap()
        }
    };

    #[cfg(feature = "stringify")]
    for encoding in args.stringify {
        stringify(&mut object, encoding)
    }

    #[cfg(feature = "enum-bools")]
    if args.enum_bools {
        convert_bool_variants(&mut object)
    }

    let pretty = args.pretty;
    match args.out_format {
        #[cfg(feature = "ser-bencode")]
        Out::Bencode => {
            let data = serde_bencode::to_bytes(&object).unwrap();

            if let Some(path) = args.out_file {
                File::create(path).unwrap().write_all(&data).unwrap();
            } else {
                stdout().write_all(&data).unwrap();
            }
        }

        #[cfg(feature = "ser-cbor")]
        Out::Cbor => {
            if let Some(path) = args.out_file {
                serde_cbor::to_writer(File::create(path).unwrap(), &object).unwrap()
            } else {
                serde_cbor::to_writer(stdout(), &object).unwrap()
            }
        }

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

        #[cfg(feature = "ser-urlencoded")]
        Out::Urlencoded => {
            let text = serde_urlencoded::to_string(&object).unwrap();

            if let Some(path) = args.out_file {
                write!(File::create(path).unwrap(), "{}", text).unwrap();
            } else {
                print!("{}", text)
            }
        }

        #[cfg(feature = "ser-xml")]
        Out::Xml => {
            if let Some(path) = args.out_file {
                let file = File::create(path).unwrap();
                quick_xml::se::to_writer(file, &object).unwrap()
            } else {
                quick_xml::se::to_writer(stdout(), &object).unwrap()
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

    stdout().flush().unwrap()
}

// TODO: Simplify all this code by extracting a `recurse` function.

fn convert_bool_variants<'a>(object: &mut Object<'a>) {
    match object {
        Object::Bool(_)
        | Object::I8(_)
        | Object::I16(_)
        | Object::I32(_)
        | Object::I64(_)
        | Object::I128(_)
        | Object::U8(_)
        | Object::U16(_)
        | Object::U32(_)
        | Object::U64(_)
        | Object::U128(_)
        | Object::F32(_)
        | Object::F64(_)
        | Object::Char(_)
        | Object::String(_)
        | Object::ByteArray(_)
        | Object::Option(_)
        | Object::Unit
        | Object::UnitStruct { .. }
        | Object::DualVariantKey { .. } => {} // Do nothing.
        Object::UnitVariant { name: _, variant } => {
            convert_bool_variants(variant);
            match variant.as_ref() {
                Object::String(cow) if cow.to_ascii_lowercase() == "true" => {
                    *object = Object::Bool(true)
                }
                Object::String(cow) if cow.to_ascii_lowercase() == "false" => {
                    *object = Object::Bool(false)
                }
                Object::ByteArray(cow) if cow.to_ascii_lowercase() == b"true" => {
                    *object = Object::Bool(true)
                }
                Object::ByteArray(cow) if cow.to_ascii_lowercase() == b"false" => {
                    *object = Object::Bool(false)
                }
                _ => {} // Do nothing.
            }
        }
        Object::NewtypeStruct { name: _, value } => convert_bool_variants(value),
        Object::NewtypeVariant {
            name: _,
            variant,
            value,
        } => {
            convert_bool_variants(variant);
            convert_bool_variants(value)
        }
        Object::Seq(elements)
        | Object::Tuple(elements)
        | Object::TupleStruct {
            name: _,
            fields: elements,
        } => convert_bool_variants_iter(elements.iter_mut()),
        Object::TupleVariant {
            name: _,
            variant,
            fields,
        } => {
            convert_bool_variants(variant);
            convert_bool_variants(fields)
        }
        Object::Map(map) => {
            for (k, v) in map.iter_mut() {
                convert_bool_variants(k);
                convert_bool_variants(v)
            }
        }
        Object::Struct { name: _, fields } => {
            convert_bool_variants_iter(fields.iter_mut().filter_map(|(_, v)| v.as_mut()))
        }
        Object::StructVariant {
            name: _,
            variant,
            fields,
        } => {
            convert_bool_variants(variant);
            convert_bool_variants(fields)
        }
        Object::FieldMap(map) => {
            for (k, v) in map.iter_mut() {
                convert_bool_variants(k);
                if let Some(v) = v.as_mut() {
                    convert_bool_variants(v)
                }
            }
        }
    }
}

fn convert_bool_variants_iter<'a, 'b: 'a>(iter: impl IntoIterator<Item = &'a mut Object<'b>>) {
    for item in iter.into_iter() {
        convert_bool_variants(item)
    }
}

fn stringify<'a>(object: &mut Object<'a>, encoding: Encoding) {
    match object {
        Object::Bool(_)
        | Object::I8(_)
        | Object::I16(_)
        | Object::I32(_)
        | Object::I64(_)
        | Object::I128(_)
        | Object::U8(_)
        | Object::U16(_)
        | Object::U32(_)
        | Object::U64(_)
        | Object::U128(_)
        | Object::F32(_)
        | Object::F64(_)
        | Object::Char(_)
        | Object::String(_) => {} // Do nothing.

        Object::DualVariantKey { .. } => {} // Do nothing. A well-behaved serializer will get the appropriate version.

        Object::ByteArray(_) => stringify_value(object, encoding),
        Object::Option(Some(b)) => stringify(b, encoding),
        Object::Option(None) | Object::Unit | Object::UnitStruct { name: _ } => {} // Do nothing.
        Object::UnitVariant { name: _, variant } => stringify_value(variant, encoding),
        Object::NewtypeStruct { name: _, value } => stringify(value, encoding),
        Object::NewtypeVariant {
            name: _,
            variant,
            value,
        } => {
            stringify_value(variant, encoding);
            stringify(value, encoding)
        }
        Object::Seq(list) => stringify_keys_iter(list.iter_mut(), encoding),
        Object::Tuple(fields) => stringify_keys_iter(fields.iter_mut(), encoding),
        Object::TupleStruct { name: _, fields } => stringify_keys_iter(fields.iter_mut(), encoding),
        Object::TupleVariant {
            name: _,
            variant,
            fields,
        } => {
            stringify_value(variant, encoding);
            stringify(fields.as_mut(), encoding)
        }
        Object::Map(map) => {
            for (k, v) in map.iter_mut() {
                stringify_value(k, encoding);
                stringify(v, encoding)
            }
        }
        Object::Struct { name: _, fields } => {
            stringify_keys_iter(fields.iter_mut().filter_map(|(_, v)| v.as_mut()), encoding)
        }
        Object::StructVariant {
            name: _,
            variant,
            fields,
        } => {
            stringify_value(variant, encoding);
            stringify(fields, encoding)
        }
        Object::FieldMap(map) => {
            for (k, v) in map.iter_mut() {
                stringify_value(k, encoding);
                if let Some(v) = v.as_mut() {
                    stringify(v, encoding)
                }
            }
        }
    }
}

fn stringify_keys_iter<'a, 'b: 'a>(
    iter: impl IntoIterator<Item = &'a mut Object<'b>>,
    encoding: Encoding,
) {
    for item in iter.into_iter() {
        stringify(item, encoding)
    }
}

fn stringify_value<'a>(object: &mut Object<'a>, encoding: Encoding) {
    *object = Object::String(
        match object {
            Object::Bool(value) => value.to_string(),
            Object::I8(value) => value.to_string(),
            Object::I16(value) => value.to_string(),
            Object::I32(value) => value.to_string(),
            Object::I64(value) => value.to_string(),
            Object::I128(value) => value.to_string(),
            Object::U8(value) => value.to_string(),
            Object::U16(value) => value.to_string(),
            Object::U32(value) => value.to_string(),
            Object::U64(value) => value.to_string(),
            Object::U128(value) => value.to_string(),
            Object::F32(value) => value.to_string(),
            Object::F64(value) => value.to_string(),
            Object::Char(value) => value.to_string(),
            Object::String(_) => return,
            Object::ByteArray(bytes) => match std::str::from_utf8(bytes.as_ref()) {
                Ok(str) => str.to_string(),
                Err(_) => {
                    return;
                }
            },
            Object::Option(option) => {
                if let Some(obj) = option.as_deref_mut() {
                    stringify_value(obj, encoding)
                }
                return;
            }
            Object::Unit
            | Object::UnitStruct { .. }
            | Object::UnitVariant { .. }
            | Object::NewtypeStruct { .. }
            | Object::NewtypeVariant { .. }
            | Object::Seq(_)
            | Object::Tuple(_)
            | Object::TupleStruct { .. }
            | Object::TupleVariant { .. }
            | Object::Map(_)
            | Object::Struct { .. }
            | Object::StructVariant { .. } => {
                return;
            }
            Object::DualVariantKey { index: _, name } => name.to_string(),
            Object::FieldMap(_) => {
                return;
            }
        }
        .pipe(Cow::Owned),
    )
}
