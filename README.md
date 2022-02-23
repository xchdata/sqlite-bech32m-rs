# sqlite-bech32m-rs

An extension for SQLite, written in Rust, which provides bech32m utility
functions.

## Installation

Run `cargo build --release` to build a shared library loadable as SQLite
extension.

Optionally, you can manually strip the library to decrease binary size:
`strip --strip-all target/release/libbech32m.so`.

## Example Usage

```
$ sqlite3
sqlite> .load ./target/release/libbech32m.so

sqlite> select bech32m_encode('xch', x'f4f6ca53d56211869b1705ce29726bad7a67d30ebe002a65450b13adbb05a669');
xch17nmv5574vggcdxchqh8zjunt44ax05cwhcqz5e29pvf6mwc95e5s27yfa4

sqlite> select hex(bech32m_decode('xch17nmv5574vggcdxchqh8zjunt44ax05cwhcqz5e29pvf6mwc95e5s27yfa4'));
F4F6CA53D56211869B1705CE29726BAD7A67D30EBE002A65450B13ADBB05A669
```

The extension also provides a utility function `blob_from_hex`, which converts
hexstrings into blobs (bytes):

```
sqlite> select x'cafe' = blob_from_hex('cafe');
1
sqlite> select hex(blob_from_hex('cafe'));
CAFE
```

## Dependencies & References

Bech32m is specified in [BIP-350].

The Bech32m implementations used is [rust-bech32].

Binding to SQLite's [loadable extension interface][loadext] is handled by
[rusqlite] extended with support for creating loadable extensions (see [pull
request #910][pr910] or [corresponding branch][rusqlite-le]).

[BIP-350]: https://github.com/bitcoin/bips/blob/master/bip-0350.mediawiki
[loadext]: https://www.sqlite.org/loadext.html
[pr910]: https://github.com/rusqlite/rusqlite/pull/910
[rusqlite-le]: https://github.com/Genomicsplc/rusqlite/tree/loadable-extensions
[rusqlite]: https://github.com/rusqlite/rusqlite
[rust-bech32]: https://github.com/rust-bitcoin/rust-bech32

## License

SPDX-License-Identifier: MIT

Copyright (C) 2022 xchdata.io

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
