# httparser

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![CI](https://github.com/testingrequired/httparser/workflows/CI/badge.svg)](https://github.com/testingrequired/httparser/actions?query=workflow%3ACI)

A push parser for the HTTP 1.x protocol. Avoids allocations. No copy. **Fast.**

Works with `no_std`, simply disable the `std` Cargo feature.

## Usage

```rust
let mut headers = [httparser::EMPTY_HEADER; 64];
let mut req = httparser::Request::new(&mut headers);

let buf = b"GET /index.html HTTP/1.1\r\nHost";
assert!(req.parse(buf)?.is_partial());

// a partial request, so we try again once we have more data

let buf = b"GET /index.html HTTP/1.1\r\nHost: example.domain\r\n\r\n";
assert!(req.parse(buf)?.is_complete());
```

## License

Licensed under either of

- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
