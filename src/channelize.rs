use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use env;
use fetch_ip;

pub fn work(urls: Vec<String>, args: env::Args) {
    let threads: i32 = urls.len() as i32;
    let (tx, rx): (Sender<fetch_ip::Fetch>, Receiver<fetch_ip::Fetch>) = mpsc::channel();

    let mut children = Vec::new();
    for url in urls {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send({ fetch_ip::Fetch::init(&url) }).unwrap();

            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            if args.verbose == true || args.verbose == false {
                println!("Request finished: {}", url);
            }
        });
        children.push(child);
    }

    let mut results = Vec::with_capacity(threads as usize);
    for _ in 0..threads {
        // `recv` will block the current thread if there are no messages available
        results.push(rx.recv());

        // break because we do only deal with the first response/result
        break;
    }

    // wait for the threads to complete any remaining work, but in this case we
    // do not want to wait for all thread, because we wanna process the first result immediately
    // for child in children {
    //     child.join().expect("oops! the child thread panicked");
    // }

    // Show the order in which the messages were sent
    let r = results[0].clone();
    println!("External IP is: {:#?}", r);
}
