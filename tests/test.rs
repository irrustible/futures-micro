#![feature(test)]

use futures_lite::future::{block_on, FutureExt};
use futures_micro::*;

#[test]
fn poll_fn_test() {
}

#[test]
fn poll_state_test() {
}

#[test]
fn poll_ref_test() {
}

#[test]
fn pending_test() {
    assert_eq!(false, block_on(async { pending().await; true}.or(ready(false))));
}

#[test]
fn ready_test() {
    assert_eq!((), block_on(ready(())));
}

#[test]
fn waker_test() {
}

#[test]
fn sleep_test() {
}

#[test]
fn next_poll_test() {
    
}

#[test]
fn yield_once_test() {
}

#[test]
fn or_test() {
    assert_eq!(
        false,
        block_on(or(async { pending().await; true }, ready(false)))
    );
}
#[test]
fn zip_test() {
    assert_eq!(
        (true, false),
        block_on(zip(ready(true), ready(false)))
    );
}

#[test]
fn zips_test() {
    assert_eq!(
        (1, (2, 3)),
        block_on(
            zips!(
                ready(1),
                ready(2),
                ready(3)
            )
        )
    );
}
