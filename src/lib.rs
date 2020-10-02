//! A very small, no-std compatible toolbox of async utilities.
#![no_std]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

pub mod prelude;

#[doc(no_inline)]
pub use core::future::Future;
#[doc(no_inline)]
pub use core::pin::Pin;
#[doc(no_inline)]
pub use core::task::{Context, Poll, Waker};

use core::fmt;
use core::marker::PhantomData;

// ---------- futures using the poll api -----------


/// Creates a future from a function returning [`Poll`].
///
/// # Examples
///
/// ```
/// use futures_lite::future::block_on;
/// use futures_micro::poll_fn;
/// use std::task::{Context, Poll};
///
/// # block_on(async {
/// fn f(_ctx: &mut Context<'_>) -> Poll<i32> {
///     Poll::Ready(7)
/// }
///
/// assert_eq!(poll_fn(f).await, 7);
/// # })
/// ```
pub fn poll_fn<F, T>(inner: F) -> PollFn<F>
where F: FnMut(&mut Context<'_>) -> Poll<T> {
    PollFn { inner }
}

/// Creates a future from a function returning [`Poll`] that has
/// access to a provided state value.
///
/// # Examples
///
/// ```
/// use futures_lite::future::block_on;
/// use futures_micro::poll_state;
/// use std::task::{Context, Poll};
///
/// # block_on(async {
/// fn f(state: &mut i32, _ctx: &mut Context<'_>) -> Poll<i32> {
///     Poll::Ready(*state + 1)
/// }
///
/// assert_eq!(poll_state(7, f).await, 8);
/// # })
/// ```
pub fn poll_state<F, S, T>(state: S, fun: F) -> PollState<F, S>
where F: FnMut(&mut S, &mut Context<'_>) -> Poll<T> {
    PollState { state, fun }
}


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

impl<F, T> Future for PollFn<F>
where F: FnMut(&mut Context<'_>) -> Poll<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_> ) -> Poll<T> {
        let this = unsafe { self.get_unchecked_mut() };
        (this.inner)(ctx)
    }
}

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

impl<F, S, T> Future for PollState<F, S>
where F: FnMut(&mut S, &mut Context<'_>) -> Poll<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<T> {
        let this = unsafe { Pin::get_unchecked_mut(self) };
        (this.fun)(&mut this.state, ctx)
    }
}

/// Returns the result of `left` or `right` future, preferring `left` if both are ready.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Or<F1, F2> {
    future1: F1,
    future2: F2,
}

impl<F1, F2> Or<F1, F2>
where
    F1: Future,
    F2: Future,
{
    /// Returns the result of `left` or `right` future, preferring `left` if both are ready.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures_micro::prelude::{Or, pending, ready};
    ///
    /// # futures_lite::future::block_on(async {
    /// assert_eq!(Or::new(ready(1), pending::<i32>()).await, 1);
    /// assert_eq!(Or::new(pending::<i32>(), ready(2)).await, 2);
    ///
    /// // The first future wins.
    /// assert_eq!(Or::new(ready(1), ready(2)).await, 1);
    /// # })
    /// ```
    pub fn new(future1: F1, future2: F2) -> Self {
        Or { future1, future2 }
    }
}

impl<T, F1, F2> Future for Or<F1, F2>
where
    F1: Future<Output = T>,
    F2: Future<Output = T>,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        if let Poll::Ready(t) = unsafe { Pin::new_unchecked(&mut this.future1) }.poll(cx) {
            return Poll::Ready(t);
        }
        if let Poll::Ready(t) = unsafe { Pin::new_unchecked(&mut this.future2) }.poll(cx) {
            return Poll::Ready(t);
        }
        Poll::Pending
    }
}

/// Waits for two [`Future`]s to complete, returning both results.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Zip<F1, F2>
where
    F1: Future,
    F2: Future,
{
    future1: F1,
    output1: Option<F1::Output>,
    future2: F2,
    output2: Option<F2::Output>,
}

impl<F1, F2> Zip<F1, F2>
where
    F1: Future,
    F2: Future,
{
    /// Zips two futures, waiting for both to complete.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures_micro::Zip;
    ///
    /// # futures_lite::future::block_on(async {
    /// let a = async { 1 };
    /// let b = async { 2 };
    ///
    /// assert_eq!(Zip::new(a, b).await, (1, 2));
    /// # })
    /// ```
    pub fn new(future1: F1, future2: F2) -> Self {
        Zip {
            future1, future2,
            output1: None,
            output2: None,
        }
    }
}

impl<F1, F2> Future for Zip<F1, F2>
where
    F1: Future,
    F2: Future,
{
    type Output = (F1::Output, F2::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = unsafe { self.get_unchecked_mut() };

        if this.output1.is_none() {
            if let Poll::Ready(out) = unsafe { Pin::new_unchecked(&mut this.future1) }.poll(cx) {
                this.output1 = Some(out);
            }
        }

        if this.output2.is_none() {
            if let Poll::Ready(out) = unsafe { Pin::new_unchecked(&mut this.future2) }.poll(cx) {
                this.output2 = Some(out);
            }
        }

        if this.output1.is_some() && this.output2.is_some() {
            Poll::Ready((this.output1.take().unwrap(), this.output2.take().unwrap()))
        } else {
            Poll::Pending
        }
    }
}

/// Creates a future that is always pending.
///
/// # Examples
///
/// ```no_run
/// use futures_micro::pending;
///
/// # futures_lite::future::block_on(async {
/// pending::<()>().await;
/// unreachable!();
/// # })
/// ```
pub fn pending<T>() -> Pending<T> {
    Pending {
        _marker: PhantomData,
    }
}

/// Future for the [`pending()`] function.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Pending<T> {
    _marker: PhantomData<T>,
}

impl<T> Unpin for Pending<T> {}

impl<T> fmt::Debug for Pending<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pending").finish()
    }
}

impl<T> Future for Pending<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<T> {
        Poll::Pending
    }
}

/// A future that resolves to the provided value.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Ready<T>(Option<T>);

impl<T> Ready<T> {

    /// Creates a future that resolves to the provided value.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures_micro::Ready;
    ///
    /// # futures_lite::future::block_on(async {
    /// assert_eq!(Ready::new(7).await, 7);
    /// # })
    /// ```
    pub fn new(value: T) -> Self {
        Ready(Some(value))
    }
}

impl<T> Unpin for Ready<T> {}

impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Ready(self.0.take().expect("`Ready` polled after completion"))
    }
}

/// Get the [`Waker`] inside an async fn where you aren't supposed to
/// have it.
///
/// This is a low level primitive for implementing more complex
/// patterns while avoiding the [`Poll`] API.
///
/// # Examples
///
/// ```
/// use futures_micro::{sleep, waker};
///
/// # futures_lite::future::block_on(async {
/// let waker = waker().await;
/// assert_eq!(async { waker.wake(); sleep().await; 1 }.await, 1)
/// # })
/// ```
pub fn waker() -> impl Future<Output = Waker> {
    poll_fn(|ctx| Poll::Ready(ctx.waker().clone()))
}

/// Goes to sleep until woken by its [`Waker`] being called.
///
/// This is a low level primitive for implementing more complex
/// patterns while avoiding the [`Poll`] API.
///
/// # Examples
///
/// ```
/// use futures_micro::{sleep, waker};
///
/// # futures_lite::future::block_on(async {
/// let waker = waker().await;
/// assert_eq!(async { waker.wake(); sleep().await; 1 }.await, 1)
/// # })
/// ```
pub fn sleep() -> impl Future<Output = ()> {
    poll_state(false, |done, _| {
        if *done { Poll::Ready(()) }
        else {
            *done = true;
            Poll::Pending
        }
    })
}    

/// Polls a future once. If it does not succeed, return it to try again
///
/// # Examples
///
/// ```
/// use futures_micro::*;
///
/// # futures_lite::future::block_on(async {
/// let f = poll_state(false, |done: &mut bool, ctx: &mut Context<'_>| {
///   if *done { Poll::Ready(1) } else { *done = true; Poll::Pending }
/// });
/// let f = next_poll(f).await.unwrap_err();
/// assert_eq!(f.await, 1);
/// # })
/// ```
pub async fn next_poll<F: Future>(mut f: F) -> Result<F::Output, F> {
    {
        let mut pin = unsafe { Pin::new_unchecked(&mut f) };
        poll_fn(|ctx| {
            match pin.as_mut().poll(ctx) {
                Poll::Ready(val) => Poll::Ready(Ok(val)),
                Poll::Pending => Poll::Ready(Err(())),
            }
        }).await
    }.map_err(|_| f)
}

/// Pushes itself to the back of the executor queue so some other
/// tasks can do some work.
pub fn yield_once() -> impl Future<Output = ()> {
    poll_state(false, |done, ctx| {
        if *done { Poll::Ready(()) }
        else {
            *done = true;
            ctx.waker().wake_by_ref();
            Poll::Pending
        }
    })
}

// --------- MACROS ---------

// Helper for `or!`
#[doc(hidden)]
#[macro_export]
macro_rules! __internal_fold_with {
    ($func:path, $e:expr) => { $e };
    ($func:path, $e:expr, $($es:expr),+) => {
        $func($e, $crate::__internal_fold_with!($func, $($es),+))
    };
}

/// Polls arbitrarily many futures, returning the first ready value.
///
/// All futures must have the same output type. Left biased when more
/// than one Future is ready at the same time.
/// # Examples
///
/// ```
/// use futures_micro::prelude::*;
///
/// # futures_lite::future::block_on(async {
/// assert_eq!(or!(ready(1), pending::<i32>()).await, 1);
/// assert_eq!(or!(pending::<i32>(), ready(2)).await, 2);
///
/// // The first ready future wins.
/// assert_eq!(or!(ready(1), ready(2), ready(3)).await, 1);
/// # })
#[macro_export]
macro_rules! or {
    ($($es:expr),+$(,)?) => { $crate::__internal_fold_with!($crate::Or::new, $($es),+) };
}

/// Pins a variable of type `T` on the stack and rebinds it as `Pin<&mut T>`.
///
/// ```
/// use futures_micro::*;
/// use std::fmt::Debug;
/// use std::time::Instant;
///
/// // Inspects each invocation of `Future::poll()`.
/// async fn inspect<T: Debug>(f: impl Future<Output = T>) -> T {
///     pin!(f);
///     poll_fn(|cx| dbg!(f.as_mut().poll(cx))).await
/// }
///
/// # futures_lite::future::block_on(async {
/// let f = async { 1 + 2 };
/// inspect(f).await;
/// # })
/// ```
#[macro_export]
macro_rules! pin {
    ($($x:ident),* $(,)?) => {
        $(
            let mut $x = $x;
            #[allow(unused_mut)]
            let mut $x = unsafe {
                core::pin::Pin::new_unchecked(&mut $x)
            };
        )*
    }
}

/// Unwraps `Poll<T>` or returns [`Pending`][`Poll::Pending`].
///
/// # Examples
///
/// ```
/// use futures_micro::*;
///
/// // Polls two futures and sums their results.
/// fn poll_sum(
///     cx: &mut Context<'_>,
///     mut a: impl Future<Output = i32> + Unpin,
///     mut b: impl Future<Output = i32> + Unpin,
/// ) -> Poll<i32> {
///     let x = ready!(Pin::new(&mut a).poll(cx));
///     let y = ready!(Pin::new(&mut b).poll(cx));
///     Poll::Ready(x + y)
/// }
/// ```
#[macro_export]
macro_rules! ready {
    ($e:expr $(,)?) => {
        match $e {
            core::task::Poll::Ready(t) => t,
            core::task::Poll::Pending => return core::task::Poll::Pending,
        }
    };
}

/// Zips arbitrarily many futures, waiting for all to complete.
///
/// # Examples
///
/// ```
/// use futures_micro::zip;
///
/// # futures_lite::future::block_on(async {
/// let a = async { 1 };
/// let b = async { 2 };
///
/// assert_eq!(zip!(a, b).await, (1, 2));
/// # })
/// ```
#[macro_export]
macro_rules! zip {
    ($($es:expr),+ $(,)?) => {
        $crate::poll_state(
            $crate::__internal_fold_with!($crate::Zip::new, $($es),+),
            |zips, ctx| {
                use ::core::future::Future;
                use ::core::pin::Pin;
                use ::core::task::Poll;

                let zips = unsafe { Pin::new_unchecked(zips) };
                if let Poll::Ready(val) = zips.poll(ctx) {
                    Poll::Ready($crate::zip!(@flatten; ; val; $($es),+))
                } else {
                    Poll::Pending
                }
            },
        )
    };

    (@flatten; $($prev:expr,)*; $tuple:expr; $e:expr) => {
        ($($prev,)* $tuple)
    };

    (@flatten; $($prev:expr,)*; $tuple:expr; $e:expr, $($es:expr),+) => {
        $crate::zip!(@flatten; $($prev,)* $tuple.0,; $tuple.1; $($es),+)
    };
}
