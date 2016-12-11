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
        let mut guard = self.lock.lock().unwrap();
        let mut res = None;
        mem::swap(&mut res, &mut guard);
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
