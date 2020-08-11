#![feature(test)]

use futures_lite::future::{block_on, FutureExt};
use futures_micro::*;

#[test]
fn pending_test() {
    assert_eq!(false, block_on(async { pending().await; true }.or(ready(false))));
}

#[test]
fn ready_test() {
    assert_eq!((), block_on(ready(())));
}

#[test]
fn waker_sleep_test() {
    assert_eq!(true, block_on(async {
        let waker = waker().await;
        waker.wake();
        sleep().await;
        true
    }));
}

#[test]
fn next_poll_test() {
    // we can't use unwrap because these functions are not debug, le sigh.
    if let Ok(ret) = block_on(next_poll(ready(1))) {
        assert_eq!(ret, 1);
    } else { panic!() }
    assert!(block_on(next_poll(pending())).is_err());
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
        block_on(
            or(async { yield_once().await; true }, ready(false))
        )
    );
}

#[test]
fn or_test() {
    assert_eq!(
        false,
        block_on(or(async { pending().await; true }, ready(false)))
    );
}

#[test]
fn ors_test() {
    assert_eq!(
        1,
        block_on(
            ors!(
                ready(1),
                ready(2),
                ready(3)
            )
        )
    );
    assert_eq!(
        2,
        block_on(
            ors!(
                async { pending().await; 1 },
                ready(2),
                ready(3)
            )
        )
    );
    assert_eq!(
        3,
        block_on(
            ors!(
                async { pending().await; 1 },
                async { pending().await; 2 },
                ready(3)
            )
        )
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