/**************************************************************************************************
 *                                                                                                *
 * This Source Code Form is subject to the terms of the Mozilla Public                            *
 * License, v. 2.0. If a copy of the MPL was not distributed with this                            *
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.                                       *
 *                                                                                                *
 **************************************************************************************************/

// =========================================== Imports ========================================== \\

pub use either::Either;

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

// ============================================ Types =========================================== \\

/// The [`Future`s] returned by this crate's functions.
///
/// [`Future`s]: core::future::Future
pub mod futs {
    /// The [`Future`] returned by [`either()`].
    ///
    /// [`Future`]: core::future::Future
    /// [`either()`]: crate::either()
    pub struct Either<L, R> {
        pub(super) left: L,
        pub(super) right: R,
    }

    #[cfg(feature = "fair")]
    #[cfg_attr(docsrs, doc(cfg(feature = "fair")))]
    /// The [`Future`] returned by [`either_fair()`].
    ///
    /// [`Future`]: core::future::Future
    /// [`either_fair()`]: crate::either_fair()
    pub struct EitherFair<L, R> {
        pub(super) left: L,
        pub(super) right: R,
    }

    /// The [`Future`] returned by [`try_either()`].
    ///
    /// [`Future`]: core::future::Future
    /// [`try_either()`]: crate::try_either()
    pub struct TryEither<L, R> {
        pub(super) fut: Either<L, R>,
    }

    #[cfg(feature = "fair")]
    #[cfg_attr(docsrs, doc(cfg(feature = "fair")))]
    /// The [`Future`] returned by [`try_either_fair()`].
    ///
    /// [`Future`]: core::future::Future
    /// [`try_either_fair()`]: crate::try_either_fair()
    pub struct TryEitherFair<L, R> {
        pub(super) fut: EitherFair<L, R>,
    }
}

// ========================================== either() ========================================== \\

/// ## Example
///
/// ```rust
/// use futures_lite::future;
/// use futures_either::{either, Either};
///
/// # future::block_on(async {
/// #
/// let out = either(
///     async { 42 },
///     async { false },
/// ).await;
/// assert_eq!(out, Either::Left(42));
///
/// let out = either(
///     future::pending::<bool>(),
///     async { 42 },
/// ).await;
/// assert_eq!(out, Either::Right(42));
/// #
/// # });
/// ```
pub fn either<L, R>(left: L, right: R) -> futs::Either<L, R>
where
    L: Future,
    R: Future,
{
    futs::Either { left, right }
}

// ======================================== either_fair() ======================================= \\

#[cfg(feature = "fair")]
#[cfg_attr(docsrs, doc(cfg(feature = "fair")))]
/// ## Example
///
/// ```rust
/// use futures_lite::future;
/// use futures_either::{either_fair, Either};
///
/// # future::block_on(async {
/// #
/// let out = either_fair(
///     async { 42 },
///     async { false },
/// ).await;
/// assert!(out == Either::Left(42) || out == Either::Right(false));
///
/// let out = either_fair(
///     future::pending::<bool>(),
///     async { 42 },
/// ).await;
/// assert_eq!(out, Either::Right(42));
/// #
/// # });
/// ```
pub fn either_fair<L, R>(left: L, right: R) -> futs::EitherFair<L, R>
where
    L: Future,
    R: Future,
{
    futs::EitherFair { left, right }
}

// ======================================== try_either() ======================================== \\

/// ## Example
///
/// ```rust
/// use futures_lite::future;
/// use futures_either::{try_either, Either};
///
/// # future::block_on(async {
/// #
/// let out = try_either(
///     async { Ok(42) },
///     async { Result::<bool, bool>::Err(false) },
/// ).await;
/// assert_eq!(out, Ok(Either::Left(42)));
///
/// let out = try_either(
///     future::pending::<Result<bool, i32>>(),
///     async { Result::<i32, i32>::Err(42) },
/// ).await;
/// assert_eq!(out, Err(42));
/// #
/// # });
/// ```
pub fn try_either<OL, OR, E, L, R>(left: L, right: R) -> futs::TryEither<L, R>
where
    L: Future<Output = Result<OL, E>>,
    R: Future<Output = Result<OR, E>>,
{
    futs::TryEither { fut: either(left, right), }
}

// ====================================== try_either_fair() ===================================== \\

#[cfg(feature = "fair")]
#[cfg_attr(docsrs, doc(cfg(feature = "fair")))]
/// ## Example
///
/// ```rust
/// use futures_lite::future;
/// use futures_either::{try_either_fair, Either};
///
/// # future::block_on(async {
/// #
/// let out = try_either_fair(
///     async { Ok(42) },
///     async { Result::<bool, bool>::Err(false) },
/// ).await;
/// assert!(out == Ok(Either::Left(42)) || out == Err(false));
///
/// let out = try_either_fair(
///     future::pending::<Result<bool, i32>>(),
///     async { Result::<i32, i32>::Err(42) },
/// ).await;
/// assert_eq!(out, Err(42));
/// #
/// # });
/// ```
pub fn try_either_fair<OL, OR, E, L, R>(left: L, right: R) -> futs::TryEitherFair<L, R>
where
    L: Future<Output = Result<OL, E>>,
    R: Future<Output = Result<OR, E>>,
{
    futs::TryEitherFair { fut: either_fair(left, right), }
}

// ========================================= impl Future ======================================== \\

impl<L, R> Future for futs::Either<L, R>
where
    L: Future,
    R: Future,
{
    type Output = Either<L::Output, R::Output>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        if let Poll::Ready(out) = unsafe { Pin::new_unchecked(&mut this.left) }.poll(ctx) {
            return Poll::Ready(Either::Left(out));
        }

        if let Poll::Ready(out) = unsafe { Pin::new_unchecked(&mut this.right) }.poll(ctx) {
            return Poll::Ready(Either::Right(out));
        }

        Poll::Pending
    }
}

#[cfg(feature = "fair")]
impl<L, R> Future for futs::EitherFair<L, R>
where
    L: Future,
    R: Future,
{
    type Output = Either<L::Output, R::Output>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        if fastrand::bool() {
            if let Poll::Ready(out) = unsafe { Pin::new_unchecked(&mut this.left) }.poll(ctx) {
                return Poll::Ready(Either::Left(out));
            }

            if let Poll::Ready(out) = unsafe { Pin::new_unchecked(&mut this.right) }.poll(ctx) {
                return Poll::Ready(Either::Right(out));
            }
        } else {
            if let Poll::Ready(out) = unsafe { Pin::new_unchecked(&mut this.right) }.poll(ctx) {
                return Poll::Ready(Either::Right(out));
            }
           
            if let Poll::Ready(out) = unsafe { Pin::new_unchecked(&mut this.left) }.poll(ctx) {
                return Poll::Ready(Either::Left(out));
            }
        }

        Poll::Pending
    }
}

impl<OL, OR, E, L, R> Future for futs::TryEither<L, R>
where
    L: Future<Output = Result<OL, E>>,
    R: Future<Output = Result<OR, E>>,
{
    type Output = Result<Either<OL, OR>, E>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        if let Poll::Ready(out) = unsafe { Pin::new_unchecked(&mut this.fut) }.poll(ctx) {
            match out {
                Either::Left(Ok(left)) => Ok(Either::Left(left)),
                Either::Right(Ok(right)) => Ok(Either::Right(right)),
                Either::Left(Err(err)) | Either::Right(Err(err)) => Err(err),
            }.into()
        } else {
            Poll::Pending
        }
    }
}

#[cfg(feature = "fair")]
impl<OL, OR, E, L, R> Future for futs::TryEitherFair<L, R>
where
    L: Future<Output = Result<OL, E>>,
    R: Future<Output = Result<OR, E>>,
{
    type Output = Result<Either<OL, OR>, E>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        if let Poll::Ready(out) = unsafe { Pin::new_unchecked(&mut this.fut) }.poll(ctx) {
            match out {
                Either::Left(Ok(left)) => Ok(Either::Left(left)),
                Either::Right(Ok(right)) => Ok(Either::Right(right)),
                Either::Left(Err(err)) | Either::Right(Err(err)) => Err(err),
            }.into()
        } else {
            Poll::Pending
        }
    }
}
