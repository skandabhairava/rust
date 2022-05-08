use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    let ctr = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let ctr = Arc::clone(&ctr);
        let handle = thread::spawn(move || {
            let mut num = ctr.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {:?}", ctr);
}
