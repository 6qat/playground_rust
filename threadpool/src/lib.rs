#![allow(dead_code, unused)]

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        Self
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
