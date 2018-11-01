use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use env;
use fetch_ip;

pub fn work(urls: Vec<String>, _args: env::Args) -> fetch_ip::Fetch {
    let nthreads: i32 = urls.len() as i32;
    let (tx, rx): (Sender<fetch_ip::Fetch>, Receiver<fetch_ip::Fetch>) = mpsc::channel();

    let mut children = Vec::new();
    for url in urls {
        // sender endpoint can be copied
        let thread_tx = tx.clone();

        let child = thread::spawn(move || {
            // thread takes ownership over `thread_tx`, each thread queues message in channel
            // sending is non-blocking operation, thread will continue after sending its message
            thread_tx.send({ fetch_ip::Fetch::init(&url) }).unwrap();
        });

        children.push(child);
    }

    let mut results = Vec::with_capacity(nthreads as usize);
    for _ in 0..nthreads {
        // `recv` will block the current thread if there are no messages available
        results.push(rx.recv().unwrap());

        // break at the first valid response
        if results[results.len() - 1].valid == true {
            break;
        }
    }

    // wait for the threads to complete any remaining work, but in this case we
    // do not want to wait for all thread, because we wanna process the first result immediately
    // for child in children {
    //     child.join().expect("oops! the child thread panicked");
    // }

    return results[results.len() - 1].clone();
}
