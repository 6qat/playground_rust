#![allow(dead_code, unused)]
#![allow(clippy::empty_loop)]

use std::sync::Mutex;
use std::sync::{mpsc, Arc};

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
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

        let receiver = Arc::new(receiver);
        let mut handles = Vec::new();
        for _ in 0..num_threads {
            let clone = receiver.clone();
            let handle = std::thread::spawn(move || loop {
                let work = clone.lock().unwrap().recv().unwrap();
                work();
            });
            handles.push(handle);
        }

        Self { _handles: handles }
    }

    pub fn execute<T: Fn()>(&self, work: T) {
        work();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new(4);
        pool.execute(|| {
            println!("Hello from thread pool");
            println!("Hello from thread pool");
        });
    }
}
