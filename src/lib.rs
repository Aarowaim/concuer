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

#[cfg(test)]
mod tests {
	use {Worker, Concurrent, Task};
	use rand;
	use std::{thread, time};

    struct WorkThread;

	impl Worker<()> for WorkThread {
  		fn body() {
        	println!("{}", rand::random::<u32>());
    	}
	}

    #[test]
	fn worker_test() {

    	let mut handles = Vec::new();
    
    	for _ in 0..200 {
        	handles.push(WorkThread.run());
    	}

    	for h in handles {
        	h.join().unwrap();
    	}
	}

    type Message = u32;

    impl Concurrent<()> for Message {
        fn body(self) {
            println!("{:?}", &self);
        }
    }

    #[test]
    fn concurrent_test() {

        let mut handles = Vec::new();
        let mut m: Message = 0; // use mutable binding in top scope to give 'static lifetime.
    
        for _ in 0..200 {
            m = rand::random::<u32>();
            handles.push(m.run());
        }

        for h in handles {
            h.join().unwrap();
        }
    }

    struct TaskThread;

    impl Task<u32, ()> for TaskThread {
        fn body(a: u32) {
            println!("{:?}", a);
        }
    }

    #[test]
    fn task_test() {

        let mut handles = Vec::new();
    
        for _ in 0..200 {
            handles.push(TaskThread.run(rand::random::<u32>()));
        }

        for h in handles {
            h.join().unwrap();
        }
    }
}