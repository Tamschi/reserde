# reserde

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/reserde)
[![Crates.io](https://img.shields.io/crates/v/reserde)](https://crates.io/crates/reserde)
[![Docs.rs](https://docs.rs/reserde/badge.svg)](https://docs.rs/reserde)

![Rust 1.53](https://img.shields.io/static/v1?logo=Rust&label=&message=1.53&color=grey)
[![CI](https://github.com/Tamschi/reserde/workflows/CI/badge.svg?branch=unstable)](https://github.com/Tamschi/reserde/actions?query=workflow%3ACI+branch%3Aunstable)
![Crates.io - License](https://img.shields.io/crates/l/reserde/0.0.4)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/reserde)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/reserde)](https://github.com/Tamschi/reserde/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/reserde)](https://github.com/Tamschi/reserde/pulls)
[![good first issues](https://img.shields.io/github/issues-raw/Tamschi/reserde/good%20first%20issue?label=good+first+issues)](https://github.com/Tamschi/reserde/contribute)

[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/reserde.svg)](https://web.crev.dev/rust-reviews/crate/reserde/)
[![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252Freserde)](https://iteration-square.schichler.dev/#narrow/stream/project.2Freserde)

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
reserde 0.0.4
Transcode a self-describing format into a different format.

Currently supports Bencode, Bincode (--out only), CBOR, JSON (--pretty), TAML (--in only), XML, x-www-form-urlencoded
(as urlencoded) and YAML. All names are lowercase.

USAGE:
    reserde.exe [FLAGS] [OPTIONS] --in <in-format> --out <out-format>

FLAGS:
        --enum-bools
            case-insensitively convert unit variants with name `true` or `false` into booleans

    -h, --help
            Prints help information

    -p
            pretty-print (where supported)

    -V, --version
            Prints version information


OPTIONS:
        --if <in-file>
            where to read input from. Defaults to stdin

    -i, --in <in-format>
            what to read [possible values: bencode, cbor, json, taml, urlencoded, xml, yaml]

        --of <out-file>
            where to write output to. Defaults to stdout

    -o, --out <out-format>
            what to write [possible values: bencode, bincode, cbor, json, urlencoded, xml, yaml]

    -s <stringify>...
            stringify bytes and non-string value keys into strings where possible. (Tries encodings in the order
            specified.) [try with: --in bencode] [possible values: utf8]
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

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING](CONTRIBUTING.md) for more information.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`reserde` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

- The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
- The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).

Note that dependencies of this crate may have a more lenient MSRV policy!
Please use `cargo +nightly update -Z minimal-versions` in your automation if you don't generate Cargo.lock manually (or as necessary) and require support for a compiler older than current stable.
