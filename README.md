# futures-micro

<!-- [![License](https://img.shields.io/crates/l/futures-micro.svg)](https://github.com/irrustible/futures-micro/blob/main/LICENSE) -->
<!-- [![Package](https://img.shields.io/crates/v/futures-micro.svg)](https://crates.io/crates/futures-micro) -->
<!-- [![Documentation](https://docs.rs/futures-micro/badge.svg)](https://docs.rs/futures-micro) -->

To futures-lite as futures-lite is to futures: smaller.

Features:
* Fun tools to write everything as async fns.
* Tiny, with no dependencies.
* 100% `no_std` support, no heap allocation required.

* Bootstrap tools:
  * `poll_fn` - wrap a function into a future
  * `poll_state` - wrap a function and some state into a future
  * `pin!()` - pin a value to the stack.
* Futures interface subversion (poll interface from async fns):
  * `async fn waker()` to get the current waker.
  * `async fn sleep()` to wait until you are woken.
  * `async fn next_poll()` - polls a future once, returning it for reuse if pending.
  * `poll_ref()` - poll as a mut ref method instead of a pin associated fn.
* Common stuff:
  * `pending()` - never completes.
  * `ready()` - completes on first poll.
  * `yield_once()` - lets some other futures do some work .
  * `or()` - return the result of the first future to complete.
  * `zip()` - return the result of two futures when they both complete.
  * `unwrap_ready!()` - unwraps a ready value or returns pending.
  * `ors!()` - `or()`, but for any number of futures of the same type.
  * `zips!()` - `zip()`, but for any number of futures.

## Status

Brand new. The API might change as we discover we've messed up.

The tests are currently being written, but hey it's all too obvious to
fail, right? Right...?

Note that this library does most things through async fns, so in
stable rust you may have to box some of these things to be able to
name the type (a need which is getting rarer over
time). `futures-lite` returns named types if you don't want to box
these.

## Copyright and License

Almost everything:

    Copyright (c) 2020 James Laver, futures-micro contributors.
    
    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/.

Exclusion: module `stolen_from_lite`:

    Source: https://github.com/stjepang/futures-lite

    Licensed under either of
    
        Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
        MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
    
    at your option.
