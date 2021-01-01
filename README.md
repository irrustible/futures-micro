# futures-micro

[![License](https://img.shields.io/crates/l/futures-micro.svg)](https://github.com/irrustible/futures-micro/blob/main/LICENSE)
[![Package](https://img.shields.io/crates/v/futures-micro.svg)](https://crates.io/crates/futures-micro)
[![Documentation](https://docs.rs/futures-micro/badge.svg)](https://docs.rs/futures-micro)

To futures-lite as futures-lite is to futures: smaller.

Features:
* Fun tools to write everything as async fns.
* Tiny, no dependencies.
* 100% `no_std` support, no heap allocation required!
* Complete stable compiler support - Uses no nightly features!

* Bootstrap tools:
  * `poll_fn` - wrap a function into a future.
  * dropped: `poll_state` - wrap a function and some state into a future.
    ```rust
    /// outdated
    poll_state(INITIAL, |state, ctx| {...}).await

    /// replacement (NOTE: `*state` should be replaced by `state`)
    let mut state = INITIAL;
    poll_fn(move |ctx| { ... }).await
    ```
  * `pin!()` - pin a value to the stack.
* Futures interface subversion (poll interface from async fns):
  * `waker()` to get the current waker.
  * `sleep()` to wait until you are woken.
  * `next_poll()` - polls a future once, returning it for reuse if pending.
* Common stuff:
  * `yield_once()` - lets some other futures do some work .
  * `or()` - return the result of the first future to complete.
  * `or!()` - `or()`, but varargs.
  * `zip()` - return the result of both futures when they both complete.
  * `zip!()` - `zip()`, but varargs.
  * `ready!()` - unwraps a ready value or returns pending.

## Missing/Removed APIs

There are many APIs you will *not* find in this crate. Some are absent to keep the crate small and quick to compile, others used to be here but have since been removed. These are:

* `pending()` - never completes, now in libcore as `core::future::pending()`
* `ready()` - completes on first poll, now in libcore as `core::future::ready()`

## Status

Beta? The API we have here seems pretty reasonable now.

If there's something you're missing, you may be looking for
[futures-lite](https://github.com/stjepang/futures-lite).

## Soundness

This crate uses `unsafe` for pin projection. We believe the code to be correct,
but we'd welcome more eyes on it.

Yes, we could get rid of them with `pin-project-lite`, but it's just
hiding the mess and then we couldn't claim `zero dependencies`.

## Copyright and License

Copyright (c) 2020 James Laver, Matthieu le Brazidec, Stjepan Glavina, Erik Zscheile,
futures-micro contributors, futures-lite contributors

[Licensed](LICENSE) under Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0),
with LLVM Exceptions (https://spdx.org/licenses/LLVM-exception.html).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
