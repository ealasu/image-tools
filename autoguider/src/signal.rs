use std::sync::{Mutex, Condvar};
use std::mem;

pub struct Signal<T> {
    lock: Mutex<Option<T>>,
    cvar: Condvar,
}

impl<T> Signal<T> {
    pub fn new() -> Self {
        Signal {
            lock: Mutex::new(None),
            cvar: Condvar::new(),
        }
    }

    pub fn set_notify(&self, v: T) {
        let mut guard = self.lock.lock().unwrap();
        *guard = Some(v);
        self.cvar.notify_one();
    }

    pub fn get(&self) -> Option<T> {
        let mut guard = self.lock.lock().unwrap();
        let mut res = None;
        mem::swap(&mut res, &mut guard);
        res
    }

    pub fn get_wait(&self) -> T {
        let mut guard = self.lock.lock().unwrap();
        loop {
            guard = self.cvar.wait(guard).unwrap();
            let mut res = None;
            mem::swap(&mut res, &mut guard);
            if let Some(res) = res {
                break res;
            }
        }
    }

    pub fn get_notify(&self) -> Option<T> {
        let res = self.get();
        self.cvar.notify_one();
        res
    }

    pub fn set_wait(&self, v: T) {
        let mut guard = self.lock.lock().unwrap();
        *guard = Some(v);
        loop {
            guard = self.cvar.wait(guard).unwrap();
            if guard.is_none() {
                break;
            }
        }
    }
}

pub trait IterableSignal<T> {
    fn iter<'a>(&'a self) -> SignalIter<'a, T>;
}

impl<T> IterableSignal<T> for Signal<Option<T>> {
    fn iter<'a>(&'a self) -> SignalIter<'a, T> {
        SignalIter(self)
    }
}

pub struct SignalIter<'a, T: 'a>(&'a Signal<Option<T>>);

impl<'a, T> Iterator for SignalIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.get_wait()
    }
}
