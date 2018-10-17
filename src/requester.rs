#![allow(unused_imports)]
#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

extern crate regex;
use self::regex::Regex;

use std::io::{self, Write};
use std::iter;
// use self::futures::{stream, Future, Stream};
use self::hyper::rt::{self, Future, Stream};
use self::hyper::Client;

use self::hyper::client::HttpConnector;
use self::hyper_tls::HttpsConnector;

pub fn get_url(url: &str) {
    let full_url = "https://".to_owned() + url;
    rt::run(fetch_url(&full_url));
}

fn fetch_url(url: &str) -> impl Future<Item = (), Error = ()> {
    let uri: self::hyper::Uri = url.parse().unwrap();

    // connector init
    let http = HttpsConnector::new(4).expect("TLS initialization failed");

    // client init
    let client = Client::builder().build::<_, hyper::Body>(http);
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

// --- utils
fn is_https(url: &str) -> bool {
    let re = Regex::new("^https.*").unwrap();
    return re.is_match(url);
}

#[test]
fn test_is_https() {
    assert_eq!(is_https("http://blabla.org"), false);
    assert_eq!(is_https("https://blabla.org"), true);
}
