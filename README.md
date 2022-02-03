![Crates.io](https://img.shields.io/crates/v/mactime2)
![Crates.io (latest)](https://img.shields.io/crates/dv/mactime2)
![Codecov](https://img.shields.io/codecov/c/github/janstarke/mactime2)
# mactime2
Replacement for `mactime`

## Changes to original `mactime`

 - no implicit conversion of timestamp to local date/time
 - possibility of explicit timezone corrrection
 - other datetime format (RFC3339) which always includes the timezone offset
 - faster

# Installation

```shell
cargo install mactime2
```

# Usage

```
mactime2 0.1.2
Jan Starke <Jan.Starke@t-systems.com>
Replacement for `mactime`

USAGE:
    mactime2 [FLAGS] [OPTIONS]

FLAGS:
    -d               output as CSV instead of TXT
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b <BODYFILE>        path to bodyfile of '-' for stdin (files ending with .gz will be treated as being gzipped)
    -t <DST_ZONE>        name of offset of destination timezone (or 'list' to display all possible values
    -f <SRC_ZONE>        name of offset of source timezone (or 'list' to display all possible values
```

# Changes

|Version|Change|
|-|-|
|0.1.2|Support for gzip compressed input as optional feature. This was a user request to allow for smaller bodyfile footprint|