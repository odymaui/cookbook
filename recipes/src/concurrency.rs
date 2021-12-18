use std::thread;
use std::time::Duration;

use std::sync::mpsc;
//use std::rc::Rc;
use std::sync::{Arc, Mutex};
//use std::sync::atomic;

//based on https://doc.rust-lang.org/book/ch16-00-concurrency.html

pub fn run_threads_without_with_join(join: bool) {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    /* putting here prevents any overlap as the threads need to finish
    if join {
        handle.join().unwrap();
    }

    pub fn join(self) -> Result<T>

Waits for the associated thread to finish.
This function will return immediately if the associated thread has already finished.
In terms of atomic memory orderings, the completion of the associated thread synchronizes with this function returning. 
In other words, all operations performed by that thread happen before all operations that happen after join returns.
    */

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    if join {
        handle.join().unwrap();
    }

}

pub fn run_thread_move_data() {
    let v = vec![1, 2, 3];

    //without move keyword, error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

pub fn thread_channel_example() {
    //channel is just a way to a transmitter on one thread
    //to send/data data to a reciever in another thread
    let (tx, rx) = mpsc::channel();
    //mpsc - multiple producer, single consumer

    let sender = thread::spawn(move || {
        let val = String::from("This is a message between threads.");
        tx.send(val).unwrap();
    });


    let receiver = thread::spawn(move || {
        let recieved = rx.recv().unwrap();
        println!("Received: {}", recieved);
    });

    //required to allow it to finish or can't run...
    sender.join().expect("Sender panicked!");
    receiver.join().expect("Receiver panicked!");
}


pub fn thread_channel_example_multiple_producers() {
    //channel is just a way to a transmitter on one thread
    //to send/data data to a reciever in another thread
    let (tx, rx) = mpsc::channel();
    //mpsc - multiple producer, single consumer

    let tx2 = tx.clone();

    let sender = thread::spawn(move || {
        let val = String::from("This is a message between threads.");
        tx.send(val).unwrap();
    });

    let sender2 = thread::spawn(move || {
        let msg = "Another message between threads.".to_string();

        tx2.send(msg).unwrap();
        //just an open pipe so no storage issue?
        //this works without issue other than time...
        //for idx in 1..=100000 {
        for idx in 1..=15 {
            tx2.send(format!("And another message-{} between threads", idx)).unwrap();
        }
    });

    let receiver = thread::spawn(move || {
        
        for recieved in rx {
            println!("Received: {}", recieved);
        }
    });

    //required to allow it to finish or can't run...
    sender.join().expect("Sender panicked!");
    sender2.join().expect("Sender panicked!");
    receiver.join().expect("Receiver panicked!");
}


pub fn mutex_demo() {
    //let m = Rc::new(Mutex::new(5));
    let m = Arc::new(Mutex::new(0));
    //you can loop and maintain the handle collection as well.
    let mut handles = vec!();
    for _ in 1..11 {
    let mc = Arc::clone(&m);
    let handle = thread::spawn(move || {
            //`Rc<Mutex<i32>>` cannot be sent between threads safely
            //let mc = Rc::clone(&m);
            //let mut num = mc.lock().unwrap();

            //need to be careful with this as you can cause mem leaks / deadlocks
            //it isn't without risks...
            //todo review example rust deadlocks with mutex.

            let mut num = mc.lock().unwrap();
            *num += 1;
            println!("incrementing {}...", num);
        });
        handles.push(handle);
    }

    //have to join before not after or not change is shown...
    //by joining here you ensure it is done so if multiple concurrent, this ensures they are done...
    for handle in handles {
        handle.join().unwrap();
    }

    println!("m = {:?}", m.lock().unwrap());
}


