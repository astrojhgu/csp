use crossbeam::{channel::bounded, channel::Receiver, channel::Sender};
use std::{collections::VecDeque};
pub struct SyncQueue<T> {
    qin: Vec<Receiver<(usize, T)>>,
    buffer: Vec<VecDeque<(usize, T)>>,
}

impl<T> SyncQueue<T>
where
    T: Clone + Default,
{
    pub fn new(n: usize) -> (Self, Vec<Sender<(usize, T)>>) {
        let (senders, qin): (Vec<_>, Vec<_>) = (0..n).map(|_| unbounded).unzip();

        (
            Self {
                qin,
                buffer: vec![VecDeque::default(); n],
            },
            senders,
        )
    }

    pub fn fetch(&mut self) -> (usize, Vec<T>) {
        loop {
            for (r, b) in self.qin.iter().zip(self.buffer.iter_mut()) {
                b.push_back(r.recv().unwrap());
            }
            let m = self
                .buffer
                .iter()
                .map(|x| x.front().unwrap().0)
                .max()
                .unwrap();
            self.buffer.iter_mut().for_each(|d| {
                while let Some(x) = d.front() {
                    if x.0 < m {
                        d.pop_front();
                    }
                }
            });
            if !self.buffer.iter().any(|x| x.is_empty()) {
                break (
                    m,
                    self.buffer
                        .iter_mut()
                        .map(|d| d.pop_front().unwrap().1)
                        .collect(),
                );
            }
        }
    }
}
