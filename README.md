# reserde

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/reserde)
[![Crates.io](https://img.shields.io/crates/v/reserde)](https://crates.io/crates/reserde)
[![Docs.rs](https://docs.rs/reserde/badge.svg)](https://docs.rs/reserde)

![Rust 1.53](https://img.shields.io/static/v1?logo=Rust&label=&message=1.53&color=grey)
[![CI](https://github.com/Tamschi/reserde/workflows/CI/badge.svg?branch=develop)](https://github.com/Tamschi/reserde/actions?query=workflow%3ACI+branch%3Adevelop)
![Crates.io - License](https://img.shields.io/crates/l/reserde/0.0.2)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/reserde)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/reserde)](https://github.com/Tamschi/reserde/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/reserde)](https://github.com/Tamschi/reserde/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/reserde.svg)](https://web.crev.dev/rust-reviews/crate/reserde/)

A Serde-based document converter.

## Installation

You can install `reserde` using Cargo:

```cmd
cargo install reserde
```

## Usage

```sh
reserde --help
```

```txt
Usage: reserde.exe [--if <if>] [--of <of>] -i <in> -o <out> [-p] [-s <stringify...>] [--enum-bools]

Transcode a self-describing format into a different format. Currently supports Bencode, Bincode (--out only), CBOR, JSON (--pretty), TAML (--in only), XML, x-www-form-urlencoded (as urlencoded) and YAML. All names are lowercase.

Options:
  --if              where to read input from. Defaults to stdin
  --of              where to write output to. Defaults to stdout
  -i, --in          what to read
  -o, --out         what to write
  -p, --pretty      pretty-print (where supported)
  -s, --stringify   stringify bytes and non-string value keys into strings where
                    possible, possible values are: utf8. (Tries encodings in the
                    order specified.) [try with: --in bencode]
  --enum-bools      case-insensitively convert unit variants with name `true` or
                    `false` into booleans.
  --help            display usage information
```

## Examples

### Converting a `.torrent` file

```sh
reserde -i bencode -o json --stringify utf8 --if manjaro-xfce-21.0.7-210614-linux510.iso.torrent
```

Output (excerpt):

```json
{
  "announce": "udp://tracker.opentrackr.org:1337",
  "created by": "mktorrent 1.1",
  "creation date": 1623684209,
  "info": {
    "length": 2600828928,
    "name": "manjaro-xfce-21.0.7-210614-linux510.iso",
    "piece length": 2097152,
    "pieces": [
      128,
      236,
      36,
      37,
      10,
      …
    ]
  },
  "url-list": "https://download.manjaro.org/xfce/21.0.7/mfce/21.0.7/manjaro-xfce-21.0.7-210614-linux51anjaro-xfce-21.0.7-210614-linux510.iso"
}
```

### TAML to YAML

```sh
reserde -i taml -o yaml
```

```taml
# [soundscapes]
// Sewer

## [[loops].{sound, volume}]
"$sewer/amb_drips", 0.8
"$sewer/amb_flies", 0.1
"$sewer/amb_hum", 0.05 // postfix comment

## [moments]
sound: "$sewer/moments/*"
layers: 1
first-interval-no-min: true
interval-range: (10, 60)
volume-range: (0.1, 0.15)

##
`spaced \` identifier`: "asdhasd kjhdajkh"
```

```yaml
---
soundscapes:
  - loops:
      - sound: $sewer/amb_drips
        volume: 0.8
      - sound: $sewer/amb_flies
        volume: 0.1
      - sound: $sewer/amb_hum
        volume: 0.05
    moments:
      - sound: $sewer/moments/*
        layers: 1
        first-interval-no-min: true
        interval-range:
          - 10
          - 60
        volume-range:
          - 0.1
          - 0.15
    "spaced ` identifier": asdhasd kjhdajkh
```

(Comments are generally stripped, as they can't be sent across Serde's interface.)

### TAML to XML

```sh
reserde -i taml -o xml
```

```taml
# [enums]: Structured
field_1: true
field_2: false

# [[enums]]
Tuple(1, 2, 3)
Unit
EmptyTuple()
```

```xml
<enums><Structured field_1="true" field_2="false"/><Tuple>1</Tuple><Tuple>2</Tuple><Tuple>3</Tuple><Unit/></enums>
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`reserde` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
