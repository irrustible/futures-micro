use core::fmt;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

/// Future for the [`poll_state()`] function.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct PollState<F, S> {
    fun: F,
    state: S,
}
    
impl<F, S> fmt::Debug for PollState<F, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PollState").finish()
    }
}

/// Like [`poll_fn`][`crate::poll_fn()`], but provides mutable access to a state.
pub fn poll_state<F, S, T>(state: S, fun: F) -> PollState<F, S>
where F: FnMut(&mut S, &mut Context) -> Poll<T> {
    PollState { state, fun }
}

impl<F, S, T> Future for PollState<F, S>
where F: FnMut(&mut S, &mut Context) -> Poll<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<T> {
        let this = unsafe { Pin::get_unchecked_mut(self) };
        (this.fun)(&mut this.state, ctx)
    }
}
