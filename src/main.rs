#![doc(html_root_url = "https://docs.rs/reserde/0.0.4")]
#![warn(clippy::pedantic)]

use serde_detach::detach;
use serde_object::Object;
use std::{
	borrow::Cow,
	fs::File,
	io::{stdin, stdout, Read as _, Write as _},
	path::PathBuf,
};
use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};
use tap::Pipe as _;

#[derive(Debug, StructOpt)]
#[structopt(name = "reserde")]
/// Transcode a self-describing format into a different format.
///
/// Currently supports Bencode, Bincode (--out only), CBOR, JSON (--pretty), TAML (--in only), XML, x-www-form-urlencoded (as urlencoded) and YAML.
/// All names are lowercase.
struct Args {
	#[structopt(long = "if")]
	/// where to read input from. Defaults to stdin
	in_file: Option<PathBuf>,

	#[structopt(long = "of")]
	/// where to write output to. Defaults to stdout
	out_file: Option<PathBuf>,

	#[structopt(short = "i", long = "in", possible_values = In::VARIANTS)]
	/// what to read
	in_format: In,

	#[structopt(short = "o", long = "out", possible_values = Out::VARIANTS)]
	/// what to write
	out_format: Out,

	#[structopt(short = "p")]
	/// pretty-print (where supported)
	pretty: bool,

	#[structopt(short = "s", possible_values = Encoding::VARIANTS)]
	/// stringify bytes and non-string value keys into strings where possible. (Tries encodings in the order specified.) [try with: --in bencode]
	stringify: Vec<Encoding>,

	#[structopt(long = "enum-bools")]
	/// case-insensitively convert unit variants with name `true` or `false` into booleans.
	enum_bools: bool,
}

#[derive(Debug, EnumString, EnumVariantNames, Clone, Copy)]
enum In {
	#[strum(serialize = "bencode")]
	Bencode,

	#[strum(serialize = "cbor")]
	Cbor,

	#[strum(serialize = "json")]
	Json,

	#[strum(serialize = "taml")]
	Taml,

	#[strum(serialize = "urlencoded")]
	Urlencoded,

	#[strum(serialize = "xml")]
	Xml,

	#[strum(serialize = "yaml")]
	Yaml,
}

#[derive(Debug, EnumString, EnumVariantNames, Clone, Copy)]
enum Out {
	#[strum(serialize = "bencode")]
	Bencode,

	#[strum(serialize = "bincode")]
	Bincode,

	#[strum(serialize = "cbor")]
	Cbor,

	#[strum(serialize = "json")]
	Json,

	#[strum(serialize = "urlencoded")]
	Urlencoded,

	#[strum(serialize = "xml")]
	Xml,

	#[strum(serialize = "yaml")]
	Yaml,
}

#[derive(Debug, EnumString, EnumVariantNames, Clone, Copy)]
enum Encoding {
	#[strum(serialize = "utf8")]
	Utf8,
}

#[allow(clippy::too_many_lines)]
fn main() {
	let args: Args = StructOpt::from_args();

	//TODO: Avoid leaking.

	let mut object: Object = match args.in_format {
		In::Bencode => {
			let mut data = vec![];
			if let Some(path) = args.in_file {
				File::open(path).unwrap().read_to_end(&mut data).unwrap();
			} else {
				stdin().read_to_end(&mut data).unwrap();
			}
			serde_bencode::from_bytes(&data).map(detach).unwrap()
		}

		In::Cbor => args
			.in_file
			.map_or_else(
				|| stdin().pipe(ciborium::de::from_reader),
				|path| File::open(path).unwrap().pipe(ciborium::de::from_reader),
			)
			.unwrap(),

		In::Json => {
			let mut text = String::new();
			if let Some(path) = args.in_file {
				File::open(path).unwrap().read_to_string(&mut text).unwrap();
			} else {
				stdin().read_to_string(&mut text).unwrap();
			}
			serde_json::from_str(&text).map(detach).unwrap()
		}

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
			serde_taml::de::from_taml_str(&text, diagnostics, &[])
				.map(detach)
				.unwrap()
		}

		In::Urlencoded => args
			.in_file
			.map_or_else(
				|| stdin().pipe(serde_urlencoded::from_reader).map(detach),
				|path| {
					File::open(path)
						.unwrap()
						.pipe(serde_urlencoded::from_reader)
						.map(detach)
				},
			)
			.unwrap(),

		In::Xml => {
			let mut text = String::new();
			if let Some(path) = args.in_file {
				File::open(path).unwrap().read_to_string(&mut text).unwrap();
			} else {
				stdin().read_to_string(&mut text).unwrap();
			}
			quick_xml::de::from_str(&text).map(detach).unwrap()
		}

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

	for encoding in args.stringify {
		stringify(&mut object, encoding);
	}

	if args.enum_bools {
		convert_bool_variants(&mut object);
	}

	let pretty = args.pretty;
	match args.out_format {
		Out::Bencode => {
			let data = serde_bencode::to_bytes(&object).unwrap();

			if let Some(path) = args.out_file {
				File::create(path).unwrap().write_all(&data).unwrap();
			} else {
				stdout().write_all(&data).unwrap();
			}
		}

		Out::Bincode => {
			if let Some(path) = args.out_file {
				bincode::serialize_into(File::create(path).unwrap(), &object).unwrap();
			} else {
				bincode::serialize_into(stdout(), &object).unwrap();
			}
		}

		Out::Cbor => args.out_file.map_or_else(
			|| ciborium::ser::into_writer(&object, stdout()).unwrap(),
			|path| ciborium::ser::into_writer(&object, File::create(path).unwrap()).unwrap(),
		),

		Out::Json => {
			if let Some(path) = args.out_file {
				let file = File::create(path).unwrap();
				if pretty {
					serde_json::to_writer_pretty(file, &object).unwrap();
				} else {
					serde_json::to_writer(file, &object).unwrap();
				}
			} else if pretty {
				serde_json::to_writer_pretty(stdout(), &object).unwrap();
			} else {
				serde_json::to_writer(stdout(), &object).unwrap();
			}
		}

		Out::Urlencoded => {
			let text = serde_urlencoded::to_string(&object).unwrap();

			if let Some(path) = args.out_file {
				write!(File::create(path).unwrap(), "{}", text).unwrap();
			} else {
				print!("{}", text);
			}
		}

		Out::Xml => args.out_file.map_or_else(
			|| quick_xml::se::to_writer(stdout(), &object).unwrap(),
			|path| {
				let file = File::create(path).unwrap();
				quick_xml::se::to_writer(file, &object).unwrap();
			},
		),

		Out::Yaml => args.out_file.map_or_else(
			|| serde_yaml::to_writer(stdout(), &object).unwrap(),
			|path| {
				let file = File::create(path).unwrap();
				serde_yaml::to_writer(file, &object).unwrap();
			},
		),
	};

	stdout().flush().unwrap();
}

// TODO: Simplify all this code by extracting a `recurse` function.

fn convert_bool_variants(object: &mut Object) {
	#[allow(clippy::match_same_arms)]
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
		| Object::DualVariantKey { .. } => (), // Do nothing.
		Object::UnitVariant { name: _, variant } => {
			convert_bool_variants(variant);
			match variant.as_ref() {
				Object::String(cow) if cow.to_ascii_lowercase() == "true" => {
					*object = Object::Bool(true);
				}
				Object::String(cow) if cow.to_ascii_lowercase() == "false" => {
					*object = Object::Bool(false);
				}
				Object::ByteArray(cow) if cow.to_ascii_lowercase() == b"true" => {
					*object = Object::Bool(true);
				}
				Object::ByteArray(cow) if cow.to_ascii_lowercase() == b"false" => {
					*object = Object::Bool(false);
				}
				_ => (), // Do nothing.
			}
		}
		Object::NewtypeStruct { name: _, value } => convert_bool_variants(value),
		Object::NewtypeVariant {
			name: _,
			variant,
			value,
		} => {
			convert_bool_variants(variant);
			convert_bool_variants(value);
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
			convert_bool_variants(fields);
		}
		Object::Map(map) => {
			for (k, v) in map.iter_mut() {
				convert_bool_variants(k);
				convert_bool_variants(v);
			}
		}
		Object::Struct { name: _, fields } => {
			convert_bool_variants_iter(fields.iter_mut().filter_map(|(_, v)| v.as_mut()));
		}
		Object::StructVariant {
			name: _,
			variant,
			fields,
		} => {
			convert_bool_variants(variant);
			convert_bool_variants(fields);
		}
		Object::FieldMap(map) => {
			for (k, v) in map.iter_mut() {
				convert_bool_variants(k);
				if let Some(v) = v.as_mut() {
					convert_bool_variants(v);
				}
			}
		}
	}
}

fn convert_bool_variants_iter<'a, 'b: 'a>(iter: impl IntoIterator<Item = &'a mut Object<'b>>) {
	for item in iter {
		convert_bool_variants(item);
	}
}

fn stringify(object: &mut Object, encoding: Encoding) {
	#[allow(clippy::match_same_arms)]
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
		| Object::String(_) => (), // Do nothing.

		Object::DualVariantKey { .. } => (), // Do nothing. A well-behaved serializer will get the appropriate version.

		Object::ByteArray(_) => stringify_value(object, encoding),
		Object::Option(Some(b)) => stringify(b, encoding),
		Object::Option(None) | Object::Unit | Object::UnitStruct { name: _ } => (), // Do nothing.
		Object::UnitVariant { name: _, variant } => stringify_value(variant, encoding),
		Object::NewtypeStruct { name: _, value } => stringify(value, encoding),
		Object::NewtypeVariant {
			name: _,
			variant,
			value,
		} => {
			stringify_value(variant, encoding);
			stringify(value, encoding);
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
			stringify(fields.as_mut(), encoding);
		}
		Object::Map(map) => {
			for (k, v) in map.iter_mut() {
				stringify_value(k, encoding);
				stringify(v, encoding);
			}
		}
		Object::Struct { name: _, fields } => {
			stringify_keys_iter(fields.iter_mut().filter_map(|(_, v)| v.as_mut()), encoding);
		}
		Object::StructVariant {
			name: _,
			variant,
			fields,
		} => {
			stringify_value(variant, encoding);
			stringify(fields, encoding);
		}
		Object::FieldMap(map) => {
			for (k, v) in map.iter_mut() {
				stringify_value(k, encoding);
				if let Some(v) = v.as_mut() {
					stringify(v, encoding);
				}
			}
		}
	}
}

fn stringify_keys_iter<'a, 'b: 'a>(
	iter: impl IntoIterator<Item = &'a mut Object<'b>>,
	encoding: Encoding,
) {
	for item in iter {
		stringify(item, encoding);
	}
}

fn stringify_value(object: &mut Object, encoding: Encoding) {
	*object = Object::String(
		#[allow(clippy::match_same_arms)]
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
					stringify_value(obj, encoding);
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
			Object::DualVariantKey { index: _, name } => (*name).to_string(),
			Object::FieldMap(_) => {
				return;
			}
		}
		.pipe(Cow::Owned),
	);
}
