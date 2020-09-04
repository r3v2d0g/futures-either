# A way to await on the output of either of two futures

[![img](https://img.shields.io/crates/l/futures-either.svg)](https://github.com/r3v2d0g/futures-either/blob/main/LICENSE.txt) [![img](https://img.shields.io/crates/v/futures-either.svg)](https://crates.io/crates/futures-either) [![img](https://docs.rs/futures-either/badge.svg)](https://docs.rs/futures-either)


## Example

```rust
use futures_lite::future;
use futures_either::{either, Either};

let out = either(
    async { 42 },
    async { false },
).await;
assert_eq!(out, Either::Left(42));

let out = either(
    future::pending::<bool>(),
    async { 42 },
).await;
assert_eq!(out, Either::Right(42));
```


## License

> This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
