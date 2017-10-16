use std::thread::*;
extern crate rand;

// Concuer is a lightweight threading library. A play on "Concurrent" and "Divide and Conquer".
// It provides two types of threading traits.

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

#[cfg(test)]
mod tests {
	use Worker;
	use rand;
	use std::{thread, time};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    struct Message;

	impl Worker<()> for Message {
  		fn body() {
        	println!("{}", rand::random::<u32>());
    	}
	}

    #[test]
	fn worker_test() {

    	let mut handles = Vec::new();
    
    	for _ in 0..20100 { // pagefile overflow?
        	handles.push(Message.run());
    	}

    	for h in handles {
        	h.join().unwrap();
    	}

    	//thread::sleep(time::Duration::from_millis(10000));
	}
}