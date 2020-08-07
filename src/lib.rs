use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};

mod poll_fn;
pub use poll_fn::{PollFn, poll_fn};

mod poll_state;
pub use poll_state::{PollState, poll_state};

// Non-async api

/// Extends Future with some methods
trait FuturesMicroExt : Future {
    /// Poll a future without faffing with pinning
    fn poll_ref(&mut self, ctx: &mut Context) -> Poll<<Self as Future>::Output>;
}

impl<F: Future + Unpin> FuturesMicroExt for F {
    fn poll_ref(&mut self, ctx: &mut Context) -> Poll<<Self as Future>::Output> {
        <F as Future>::poll(Pin::new(self), ctx)
    }
}

#[allow(unused_unsafe)] // lol thanks rust, sorry for guarding against the future
pub unsafe fn poll_ref_unchecked<F, T>(fut: &mut F, ctx: &mut Context) -> Poll<T>
where F: Future<Output = T> {
    let pin = unsafe { Pin::new_unchecked(fut) };
    <F as Future>::poll(pin, ctx)
}

// Async API

/// Never complete.
pub async fn pending() {
    poll_fn(|_| Poll::Pending).await
}

/// Complete on first poll with the provided value
pub async fn ready<T>(val: T) -> T {
    poll_state(Some(val), |val, _| Poll::Ready(val.take().unwrap())).await
}

/// Get the [`Waker`] inside an async fn where you aren't supposed to
/// have it.
pub async fn waker() -> Waker {
    poll_fn(|ctx| Poll::Ready(ctx.waker().clone())).await
}

/// Goes to sleep until woken by its [`Waker`] being called.
pub async fn sleep() {
    poll_state(false, |done, _| {
        if *done { Poll::Ready(()) }
        else {
            *done = true;
            Poll::Pending
        }
    }).await
}    

/// Polls a future once. If it does not succeed, return it to try again
pub async fn next_poll<F: Future>(f: F) -> Result<F::Output, F> {
    poll_state(Some(f), |f, ctx| {
        let mut f = f.take().unwrap();
        let pin = unsafe { Pin::new_unchecked(&mut f) };
        match <F as Future>::poll(pin, ctx) {
            Poll::Ready(val) => Poll::Ready(Ok(val)),
            Poll::Pending => Poll::Ready(Err(f)),
        }
    }).await
}

/// Pushes itself to the back of the executor queue so some other
/// tasks can do some work.
pub async fn yield_once() {
    poll_state(false, |done, ctx| {
        if *done { Poll::Ready(()) }
        else {
            *done = true;
            ctx.waker().wake_by_ref();
            Poll::Pending
        }
    }).await
}

pub async fn or<A, B, T>(a: A, b: B) -> T
where A: Future<Output=T> + Unpin,
      B: Future<Output=T> + Unpin {
    poll_state(Some((a, b)), |state, ctx|{
        let (a, b) = state.as_mut().unwrap();
        match <A as Future>::poll(Pin::new(a), ctx) {
            Poll::Ready(val) => Poll::Ready(val),
            Poll::Pending => {
                match <B as Future>::poll(Pin::new(b), ctx) {
                    Poll::Ready(val) => Poll::Ready(val),
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }).await
}

pub async fn or_unchecked<A, B, T>(a: A, b: B) -> T
where A: Future<Output=T>, B: Future<Output=T> {
    poll_state((a, b), |(a, b), ctx| {
        let af = unsafe { Pin::new_unchecked(a) };
        let bf = unsafe { Pin::new_unchecked(b) };
        match <A as Future>::poll(af, ctx) {
            Poll::Ready(val) => Poll::Ready(val),
            Poll::Pending => {
                match <B as Future>::poll(bf, ctx) {
                    Poll::Ready(val) => Poll::Ready(val),
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }).await
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
