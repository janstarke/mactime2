![Crates.io](https://img.shields.io/crates/v/mactime2)
![Crates.io (latest)](https://img.shields.io/crates/dv/mactime2)
![Codecov](https://img.shields.io/codecov/c/github/janstarke/mactime2)
# mactime2
Replacement for `mactime`

## Changes to original `mactime`

 - no implicit conversion of timestamp to local date/time
 - possibility of explicit timezone correction
 - other datetime format (RFC3339) which always includes the timezone offset
 - faster

# Installation

```shell
cargo install mactime2
```

# Usage

```
Usage: mactime2 [OPTIONS]

Options:
  -v, --verbose...                More output per occurrence
  -q, --quiet...                  Less output per occurrence
  -b <INPUT_FILE>                 path to input file or '-' for stdin (files ending with .gz will be treated as being gzipped) [default: -]
  -f, --from-timezone <SRC_ZONE>  name of offset of source timezone (or 'list' to display all possible values
  -t, --to-timezone <DST_ZONE>    name of offset of destination timezone (or 'list' to display all possible values
      --strict                    strict mode: do not only warn, but abort if an error occurs
  -F, --format <OUTPUT_FORMAT>    output format, if not specified, default value is 'txt' [possible values: csv, txt, json, elastic]
  -d                              output as CSV instead of TXT. This is a conveniance option, which is identical to `--format=csv` and will be removed in a future release.
                                  If you specified `--format` and `-d`, the latter will be ignored
  -j                              output as JSON instead of TXT. This is a conveniance option, which is identical to `--format=json` and will be removed in a future release.
                                  If you specified `--format` and `-j`, the latter will be ignored
  -h, --help                      Print help information
  -V, --version                   Print version information

```

# Examples

## Convert Windows Event Log timelines to better parsable json

```shell
mactime2 -j -b sample.bodyfile  | jq '{"ts": .ts, "event": .name | fromjson | {"event_id": .event_id, "channel": .channel_name, "data": .custom_data} }'
```

results in an output like the following:
```json
{
  "ts": "2022-05-16T03:19:19+00:00",
  "event": {
    "event_id": 4624,
    "channel": "Security",
    "data": {
      "EventData": {
        "AuthenticationPackageName": "-",
        "IpAddress": "-",
        "IpPort": "-",
        "KeyLength": 0,
        "LmPackageName": "-",
        "LogonGuid": "00000000-0000-0000-0000-000000000000",
        "LogonProcessName": "-",
        "LogonType": 0,
        "ProcessId": "0x4",
        "ProcessName": "",
        "SubjectDomainName": "-",
        "SubjectLogonId": "0x0",
        "SubjectUserName": "-",
        "SubjectUserSid": "S-1-0-0",
        "TargetDomainName": "NT-AUTORITÄT",
        "TargetLogonId": "0x3e7",
        "TargetUserName": "SYSTEM",
        "TargetUserSid": "S-1-5-18",
        "TransmittedServices": "-",
        "WorkstationName": "-"
      }
    }
  }
}
```

# Changes

|Version|Change|
|-|-|
|0.1.2|Support for gzip compressed input as optional feature. This was a user request to allow for smaller bodyfile footprint|
|0.2.2|don't ignore lines with invalid characters anymore|
|1.0.1|Also display lines with all timestamps set to `-1`|
|1.0.5|better handling of ambiguous file names| 
|1.1.0|Support for JSON output, parsable by ` jq`|