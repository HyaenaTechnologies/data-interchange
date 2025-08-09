[Rust Language]: https://rust-lang.org
[CommonMark]:https://commonmark.org/
[HTTP]: https://developer.mozilla.org/en-US/docs/Web/HTTP
[JSON]: https://www.json.org/json-en.html
[TOML]: https://toml.io/en/
[Web Transport]: https://w3c.github.io/webtransport/
[YAML]: https://yaml.org/

# Data Interchange

Data Interchange Format and Network Protocol Implementations

## Features

- Comma-Separated Values
- [Hypertext Transfer Protocol][HTTP]
- [JavaScript Object Notation][JSON]
- [Markdown][CommonMark]
- [Tom's Obvious Minimal Language][TOML]
- Web Socket Protocol
- [Web Transport Protocol][Web Transport]
- [YAML Ain't Markup Language][YAML]

## Build

- [Rust][Rust Language]

```shell
git clone

cargo check

cargo build --release --target x86_64-unknown-linux-gnu

mv ./target/x86_64-unknown-linux-gnu/release/htdi ./binary

./binary/htdi
```

## Install

```shell
sudo install ./htdi /usr/local/bin/
```
