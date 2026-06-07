# IP Replacement

Fetch SIP002 subscriptions, replace server hostnames with a specified IP address, and generate:

* A rewritten `.sub` subscription file
* A FlClash hosts overwriting script

This is useful when DNS resolution is unreliable, blocked, or intentionally bypassed.

## Mihomo

Follow [虚空终端 Docs](https://wiki.metacubex.one/config/dns/hosts/).

## Features

* Download Base64-encoded SIP002 subscriptions over HTTP(S)
* Replace server hostnames with a fixed IP address
* Generate hostname → IP mappings
* Per-subscription User-Agent override
* Multiple subscriptions in a single configuration file

## Build

```bash
cargo build --release
```

The binary will be located at:

```text
target/release/ip-replacement
```

## Usage

Create a `config.toml` file in the current working directory:

```bash
cargo run --release
```

Generated files are written to the `subs/` directory.

## Configuration

### Example

```toml
[[subscription]]
name = "example"
url = "https://example.com/subscription"
ip = "1.2.3.4"

# Optional
user_agent = "MyClient/1.0"
```

### Fields

| Field        | Required | Description                                     |
| ------------ | -------- | ----------------------------------------------- |
| `name`       | Yes      | Output file name prefix                         |
| `url`        | Yes      | Subscription URL                                |
| `ip`         | Yes      | IP address used to replace all server hostnames |
| `user_agent` | No       | Custom User-Agent for fetching the subscription |

### Multiple Subscriptions

```toml
[[subscription]]
name = "provider-a"
url = "https://a.example/sub"
ip = "1.2.3.4"

[[subscription]]
name = "provider-b"
url = "https://b.example/sub"
ip = "8.8.8.8"
user_agent = "CustomAgent/1.0"
```

## Output

### Subscription File

`<name>.sub`

All server hostnames are replaced with the configured IP address.

Example:

```text
ss://...@example.com:443
```

becomes:

```text
ss://...@1.2.3.4:443
```

### Host Mapping File

`<name>.yaml`

Example:

```yaml
hosts:
    example.com: "1.2.3.4"
    cdn.example.com: "1.2.3.4"
```

These mappings can be consumed by tools that support static hostname overrides.

## Notes

* `www.google.com` and `www.g00gle.com` are intentionally excluded from host mapping generation.

## License

MIT
