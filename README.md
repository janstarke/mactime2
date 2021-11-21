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
mactime2 0.1.0
Jan Starke <Jan.Starke@t-systems.com>
Replacement for `mactime`

USAGE:
    mactime2 [FLAGS] [OPTIONS]

FLAGS:
    -d               output as CSV instead of TXT
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b <BODYFILE>        path to bodyfile of '-' for stdin
    -t <DST_ZONE>        name of offset of destination timezone (or 'list' to display all possible values
    -f <SRC_ZONE>        name of offset of source timezone (or 'list' to display all possible values
```