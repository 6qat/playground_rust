#![allow(dead_code, unused)]
#![allow(clippy::empty_loop)]

use std::sync::Mutex;
use std::sync::{mpsc, Arc};

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
    _sender: mpsc::Sender<Box<dyn Fn() + Send>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = mpsc::channel::<Box<dyn Fn() + Send>>();
        let receiver = Mutex::new(receiver);

        // let handles = (0..num_threads)
        //     .map(|_| {
        //         std::thread::spawn(move || loop {
        //             let work = receiver.lock().unwrap().recv().unwrap();
        //             work();
        //         })
        //     })
        //     .collect();

        // Study: std::borrow::Cow https://doc.rust-lang.org/std/borrow/enum.Cow.html

        let receiver = Arc::new(receiver);
        let mut handles = Vec::new();
        for _ in 0..num_threads {
            let clone = receiver.clone();
            let handle = std::thread::spawn(move || loop {
                let work = clone.lock().unwrap().recv().unwrap();
                println!("Thread {:?} got work", std::thread::current().id());
                work();
                println!("Thread {:?} finished work", std::thread::current().id());
            });
            handles.push(handle);
        }

        Self {
            _handles: handles,
            _sender: sender,
        }
    }

    pub fn execute<T: Fn() + Send + 'static>(&self, work: T) {
        self._sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new(4);
        let foo = || std::thread::sleep(std::time::Duration::from_secs(1));
        pool.execute(foo); // foo implements Copy trait, so no need to clone() it here
        pool.execute(foo);
        pool.execute(foo);
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
