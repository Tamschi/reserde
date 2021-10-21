# reserde Changelog

<!-- markdownlint-disable no-trailing-punctuation -->

## 0.0.3

2021-10-21

- Features:
  - `-h` made alternate option to display help. (contributed by @scribe in #31 and #32)
    > This is implemented by switching to `structopt` for argument parsing.  
    > Additionally, `--in` and `--out` now show valid choices with the `-h`/`--help` command.

- Revisions:
  - Updated dependencies.

## 0.0.2

2021-08-01

- **Breaking**:
  - Increased minimum Rust version to 1.53.
  - Removed features switches.
  - `serde_taml` type inference changed,
    so `true` and `false` are interpreted as booleans even by default now.

- Features:
  - Generally updated dependencies,
    which comes with newly supported input in at least some cases.

## 0.0.1

2020-08-26

Initial unstable release
