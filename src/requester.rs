#![allow(unused_imports)]
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use std::iter;
use self::futures::{stream, Future, Stream};
use self::hyper::Client;
// use tokio_core::reactor::Core;


pub fn get_url(url: &str) -> impl Future<Item=(), Error=()> {
    let uri: self::hyper::Uri = url.parse().unwrap();
    let client = Client::new();

    client
        // Fetch the url...
        .get(uri)
        // And then, if we get a response back...
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: {:#?}", res.headers());

            // The body is a stream, and for_each returns a new Future
            // when the stream is finished, and calls the closure on
            // each chunk of the body...
            res.into_body().for_each(|chunk| {
                io::stdout().write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error={}", e))
            })
        })
        // If all good, just tell the user...
        .map(|_| {
            println!("\n\nDone.");
        })
        // If there was an error, let the user know...
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
}



// fn get_url() {
//     // still inside rt::run...
// let client = Client::new();
//
// let uri = "http://httpbin.org/ip".parse().unwrap();
//
// client
//     .get(uri)
//     .and_then(|res| {
//         println!("Response: {}", res.status());
//         res
//             .into_body()
//             // Body is a stream, so as each chunk arrives...
//             .for_each(|chunk| {
//                 io::stdout()
//                     .write_all(&chunk)
//                     .map_err(|e| {
//                         panic!("example expects stdout is open, error={}", e)
//                     })
//             })
//     })
//     .map_err(|err| {
//         println!("Error: {}", err);
//     })
//
// }
//
// // fn main() {
// //     get_url();
// // }
