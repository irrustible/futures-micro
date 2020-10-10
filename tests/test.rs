#![feature(test)]
#![allow(deprecated)]

use futures_lite::future::{block_on, FutureExt};
use futures_micro::prelude::*;

#[test]
fn pending_test() {
    assert_eq!(false, block_on(pending::<bool>().or(ready(false))));
}

#[test]
fn ready_test() {
    assert_eq!((), block_on(ready(())));
}

#[test]
fn waker_sleep_test() {
    assert!(block_on(async {
        let waker = waker().await;
        waker.wake();
        sleep().await;
        true
    }));
}

#[test]
fn next_poll_test() {
    assert_eq!(block_on(next_poll(ready(1))), Ok(1));
    assert!(block_on(next_poll(pending::<bool>())).is_err());
}

#[test]
fn yield_once_test() {
    assert_eq!(
        true,
        block_on(async {
            yield_once().await;
            true
        })
    );
    assert_eq!(
        false,
        block_on(or!(
            async {
                yield_once().await;
                true
            },
            ready(false)
        ))
    );
}

#[test]
fn or_test() {
    assert_eq!(false, block_on(or(pending::<bool>(), ready(false))));
    assert_eq!(1, block_on(or!(ready(1), ready(2), ready(3))));
    assert_eq!(2, block_on(or!(pending(), ready(2), ready(3))));
    assert_eq!(3, block_on(or!(pending(), pending(), ready(3))));
}

#[test]
fn zip_test() {
    assert_eq!((true, false), block_on(zip(ready(true), ready(false))));
    assert_eq!((1, 2, 3), block_on(zip!(ready(1), ready(2), ready(3))));

    assert_eq!(
        (1, false, 3),
        block_on(zip!(ready(1), ready(false), ready(3)))
    );
}
