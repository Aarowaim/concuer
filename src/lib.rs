mod tests;

use std::thread::*;
extern crate rand;

// Concuer is a lightweight threading library. A play on "Concurrent" and "Divide and Conquer".
// It provides a few experimental threading traits.

pub trait Worker<R>
where
    Self: Sync + 'static,
    R: Send + 'static,
{
    fn run(&self) -> JoinHandle<R> {
        spawn(move || {
            Self::body()
        })
    }
    
    fn body() -> R;
}

pub trait Concurrent<R>
where
    Self: Sync + Send + Sized + 'static,
    R: Send + 'static,
{
    fn run(self) -> JoinHandle<R> {
        spawn(move || {
            self.body()
        })
    }
    
    fn body(self) -> R;
}

pub trait Task<A, R>
where
    Self: Sync + 'static,
    A: Send + 'static,
    R: Send + 'static,
{
    fn run(&self, a: A) -> JoinHandle<R> {
        spawn(move || {
            Self::body(a)
        })
    }
    
    fn body(a: A) -> R;
}