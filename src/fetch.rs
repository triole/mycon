extern crate ureq;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use fetch;

pub fn channelize(urls: Vec<String>) {
    let threads: i32 = urls.len() as i32;
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let mut children = Vec::new();
    for url in urls {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send({ fetch::fetch_url(&url) }).unwrap();

            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            println!("thread {} finished", url);
        });
        children.push(child);
    }

    let mut ids = Vec::with_capacity(threads as usize);
    for _ in 0..threads {
        // `recv` will block the current thread if there are no messages available
        ids.push(rx.recv());
        if 1 == 1 {
            break;
        }
    }

    // wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);
}

pub fn fetch_url(url: &str) -> String {
    // sync post request of some json.
    let mut req = ureq::get(url)
        .set("X-My-Header", "Secret")
        .set("Accept", "text/plain")
        .timeout_connect(2000)
        .to_owned();
    let response = req.call();

    // println!("\n{:?}", id);
    // println!("{:#?}", response.into_string());
    let r = response.into_string().unwrap();
    return r;
}
