use core::fmt;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

/// Future for the [`poll_fn()`] function.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct PollFn<F> {
    inner: F
}
    
impl<F> fmt::Debug for PollFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PollFn").finish()
    }
}

/// Creates 
pub fn poll_fn<F, T>(inner: F) -> PollFn<F>
where F: FnMut(&mut Context) -> Poll<T> {
    PollFn { inner }
}

impl<F, T> Future for PollFn<F>
where F: FnMut(&mut Context) -> Poll<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<T> {
        let this = unsafe { self.get_unchecked_mut() };
        (this.inner)(ctx)
    }
}
