//! This module has extras that clash with names in [`futures-lite`],
//! which depends on us.
pub use crate::*;

/// Creates a future that resolves to the provided value.
///
/// # Examples
///
/// ```
/// use futures_micro::prelude::ready;
///
/// # futures_lite::future::block_on(async {
/// assert_eq!(ready(7).await, 7);
/// # })
/// ```
pub fn ready<T>(val: T) -> Ready<T> {
    Ready::new(val)
}

/// Zips two futures, waiting for both to complete.
///
/// # Examples
///
/// ```
/// use futures_micro::prelude::zip;
///
/// # futures_lite::future::block_on(async {
/// let a = async { 1 };
/// let b = async { 2 };
///
/// assert_eq!(zip(a, b).await, (1, 2));
/// # })
/// ```
pub fn zip<F1, F2>(f1: F1, f2: F2) -> Zip<F1, F2>
where
    F1: Future,
    F2: Future,
{
    Zip::new(f1, f2)
}

/// Returns the result of `left` or `right` future, preferring `left` if both are ready.
///
/// # Examples
///
/// ```
/// use futures_micro::prelude::{or, pending, ready};
///
/// # futures_lite::future::block_on(async {
/// assert_eq!(or(ready(1), pending::<i32>()).await, 1);
/// assert_eq!(or(pending::<i32>(), ready(2)).await, 2);
///
/// // The first future wins.
/// assert_eq!(or(ready(1), ready(2)).await, 1);
/// # })
/// ```
pub fn or<F1, F2>(future1: F1, future2: F2) -> Or<F1, F2>
where
    F1: Future,
    F2: Future,
{
    Or { future1, future2 }
}
