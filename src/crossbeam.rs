use crossbeam_channel as chan;

use super::TryIterRecv;

impl<'a, T: 'a> TryIterRecv<'a> for chan::Receiver<T> {
    type Iter = chan::TryIter<'a, T>;
    type Error = chan::RecvError;
    fn try_iter(&'a self) -> chan::TryIter<'a, T> {
        chan::Receiver::try_iter(&self)
    }
    fn recv(&self) -> Result<T, Self::Error> {
        chan::Receiver::recv(&self)
    }
}

#[cfg(test)]
mod tests {
    use crossbeam_channel as chan;
    use std::thread;
    use super::super::BatchRecv;
    #[test]
    fn take3of4() {
        let (tx, rx) = chan::bounded(10);
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        tx.send(3).unwrap();
        tx.send(4).unwrap();
        let first3: Vec<_> = rx.batch_recv(3).unwrap().collect();
        assert_eq!(first3, vec![1, 2, 3]);
    }

    #[test]
    fn take2of2() {
        let (tx, rx) = chan::bounded(10);
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        let first2: Vec<_> = rx.batch_recv(3).unwrap().collect();
        assert_eq!(first2, vec![1, 2]);
    }

    #[test]
    fn take_first1() {
        let (tx, rx) = chan::bounded(10);
        thread::spawn(move || {
            thread::yield_now();
            tx.send(1).unwrap();
        });
        let first1: Vec<_> = rx.batch_recv(3).unwrap().collect();
        assert_eq!(first1, vec![1]);
    }
}
