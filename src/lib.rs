//! Batched receive from queues
//!
//! To use the [`batch_recv` method](./trait.BatchRecv.html#method.batch_recv), import the [`BatchRecv` trait](./trait.BatchRecv.html):
//!
//! ```
//! use batch_recv::BatchRecv;
//! ```
//!
//! ## Crate Features
//! - `crossbeam`
//!   - Disabled by default
//!   - Provied [`TryIterRecv` trait](./trait.TryIterRecv.html) implementation for crossbeam_channel::Receiver

#[cfg(feature = "crossbeam")]
extern crate crossbeam_channel;

#[cfg(feature = "crossbeam")]
mod crossbeam;

use std::iter::{self, Iterator};
use std::vec;
use std::sync::mpsc;

/// Trait for queues which have batch-able receiving method.
///
/// This is mainly used by [`BatchRecv` trait](./trait.BatchRecv.html).
///
/// At first, [`BatchRecv` trait](./trait.BatchRecv.html) calls `recv`
/// which blocks the thread until the first value comes.
///
/// And then, it calls `try_iter` to retrieve the following values.
pub trait TryIterRecv<'a> {
    type Iter: Iterator + 'a;
    type Error;
    fn try_iter(&'a self) -> Self::Iter;
    fn recv(&self) -> Result<<Self::Iter as Iterator>::Item, Self::Error>;
}

/// Trait which provides batched receiving method.
pub trait BatchRecv<'a>: TryIterRecv<'a> {
    /// Creates an iterator yields its first `n` values.
    /// It blocks until the first value comes if the queue is empty.
    fn batch_recv(&'a self, n: usize) -> Result<BatchIter<Self::Iter>, Self::Error> {
        let first = self.recv()?;
        let rest = self.try_iter().take(n - 1);
        let inner = vec![first].into_iter().chain(rest);
        Ok(inner)
    }
}
impl<'a, T> BatchRecv<'a> for T
where
    T: TryIterRecv<'a>,
{
}

pub type BatchIter<I> = iter::Chain<vec::IntoIter<<I as Iterator>::Item>, iter::Take<I>>;

impl<'a, T: 'a> TryIterRecv<'a> for mpsc::Receiver<T> {
    type Iter = mpsc::TryIter<'a, T>;
    type Error = mpsc::RecvError;
    fn try_iter(&'a self) -> mpsc::TryIter<'a, T> {
        mpsc::Receiver::try_iter(&self)
    }
    fn recv(&self) -> Result<T, Self::Error> {
        mpsc::Receiver::recv(&self)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use super::BatchRecv;
    #[test]
    fn take3of4() {
        let (tx, rx) = mpsc::sync_channel(10);
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        tx.send(3).unwrap();
        tx.send(4).unwrap();
        let first3: Vec<_> = rx.batch_recv(3).unwrap().collect();
        assert_eq!(first3, vec![1, 2, 3]);
    }

    #[test]
    fn take2of2() {
        let (tx, rx) = mpsc::sync_channel(10);
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        let first2: Vec<_> = rx.batch_recv(3).unwrap().collect();
        assert_eq!(first2, vec![1, 2]);
    }

    #[test]
    fn take_first1() {
        let (tx, rx) = mpsc::sync_channel(10);
        thread::spawn(move || {
            thread::yield_now();
            tx.send(1).unwrap();
        });
        let first1: Vec<_> = rx.batch_recv(3).unwrap().collect();
        assert_eq!(first1, vec![1]);
    }
}
