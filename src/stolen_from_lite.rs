/// Unwraps `Poll<T>` or returns [`Pending`][`std::task::Poll::Pending`].
///
/// # Examples
///
/// ```
/// use futures_lite::*;
/// use std::pin::Pin;
/// use std::task::{Context, Poll};
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
macro_rules! unwrap_ready {
    ($e:expr $(,)?) => {
        match $e {
            std::task::Poll::Ready(t) => t,
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}

/// Pins a variable of type `T` on the stack and rebinds it as `Pin<&mut T>`.
#[macro_export]
macro_rules! pin {
    ($($x:ident),* $(,)?) => {
        $(
            let mut $x = $x;
            #[allow(unused_mut)]
            let mut $x = unsafe {
                std::pin::Pin::new_unchecked(&mut $x)
            };
        )*
    }
}

// Helper for `or!`
#[doc(hidden)]
#[macro_export]
macro_rules! __internal_fold_with {
    ($func:path, $e:expr) => { $e };
    ($func:path, $e:expr, $($es:expr),+) => {
        $func($e, $crate::__internal_fold_with!($func, $($es),+))
    };
}

/// Like `or()`, but accepts an arbitrary number of futures rather
/// than just two. Returns the result of the first future to complete;
/// if multiple futures complete at the same time, returns the first
/// one to complete. All of the futures must have the same return
/// type.
#[macro_export]
macro_rules! ors {
    ($($es:expr),+$(,)?) => { $crate::__internal_fold_with!($crate::or, $($es),+) };
}

/// Like [`ors`], but combines with [`zip`] instead of [`or`], only
/// returning when all are completed.
///
/// Note: this is going to return tuples nesting towards the right
/// when n > 2. I don't like it, but my macro-fu is too weak to
/// unravel it.
#[macro_export]
macro_rules! zips {
    ($($es:expr),+$(,)?) => { $crate::__internal_fold_with!($crate::zip, $($es),+) };
}
