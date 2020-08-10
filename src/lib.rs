use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};

mod poll_fn;
pub use poll_fn::{PollFn, poll_fn};

mod poll_state;
pub use poll_state::{PollState, poll_state};

// licensed differently
mod stolen_from_lite;

// Non-async api

/// Extends Future with some methods
trait FuturesMicroExt : Future {
    /// Poll a future without faffing with pinning
    fn poll_ref(&mut self, ctx: &mut Context) -> Poll<<Self as Future>::Output>;
}

impl<F: Future + Unpin> FuturesMicroExt for F {
    fn poll_ref(&mut self, ctx: &mut Context) -> Poll<<Self as Future>::Output> {
        <F as Future>::poll(unsafe { Pin::new_unchecked(self) }, ctx)
    }
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

/// Polls two futures with a left bias until one of them succeeds.
pub async fn or<T>(a: impl Future<Output=T>, b: impl Future<Output=T>) -> T {
    pin!(a, b);
    poll_fn(move |ctx|{
        match a.as_mut().poll(ctx) {
            Poll::Ready(val) => Poll::Ready(val),
            Poll::Pending => {
                match b.as_mut().poll(ctx) {
                    Poll::Ready(val) => Poll::Ready(val),
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }).await
}

/// Polls two futures until they are both completed.
pub async fn zip<A, B>(a: impl Future<Output=A>, b: impl Future<Output=B>) -> (A, B) {
    pin!(a, b);
    poll_state((None, None), move |(c, d), ctx|{
        if c.is_none() {
            if let Poll::Ready(val) = a.as_mut().poll(ctx) {
                *c = Some(val);
            }
        }
        if d.is_none() {
            if let Poll::Ready(val) = b.as_mut().poll(ctx) {
                *d = Some(val);
            }
        }
        if c.is_some() && d.is_some() {
            Poll::Ready((c.take().unwrap(), d.take().unwrap()))
        } else {
            Poll::Pending
        }
    }).await
}

