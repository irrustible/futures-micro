#[feature(test)]
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
fn poll_ref_unchecked_test() {
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
    // assert_eq!(
    //     false,
    //     block_on(
    //         or(
    //             Box::new(async { pending().await; true }),
    //             Box::new(ready(false))
    //         )
    //     )
    // );
}

#[test]
fn or_unchecked_test() {
    // assert_eq!(
    //     false,
    //     block_on(
    //         or_unchecked(
    //             Box::new(async { pending().await; true }),
    //             Box::new(ready(false))
    //         )
    //     )
    // );
}

