extern crate regex;
extern crate ureq;
use self::regex::Regex;
use self::ureq::Error;
use std::time::Duration;

pub fn rx_find(rx_str: &str, searched_str: &str) -> String {
    let re = Regex::new(rx_str).unwrap();
    let caps = re.captures(searched_str);
    match caps {
        Some(caps) => caps[0].to_string(),
        None => String::new(),
    }
}

pub fn rx_match(rx_str: &str, s: &str) -> bool {
    let re = Regex::new(rx_str).unwrap();
    return re.is_match(s);
}

pub fn fetch_url(url: &str) -> (String, u16) {
    // sync post request of some json.
    let two_seconds = Duration::new(2, 0);
    let response = ureq::get(url)
        .set("X-My-Header", "Secret")
        .set("Accept", "text/plain")
        .timeout(two_seconds)
        .to_owned()
        .call();

    match response {
        Ok(x) => {
            return (x.into_string().unwrap(), 200);
        }
        Err(Error::Status(code, _)) => {
            return ("".to_string(), code);
        }
        Err(_) => return ("".to_string(), 9999),
    };
}

pub fn tor_stats(torcheck_url: &str) -> (bool, String) {
    let (body, _) = fetch_url(&torcheck_url);
    let enabled = rx_match("Congratulations.*to use Tor", &body);
    let ip = rx_find("(?:[0-9]{1,3}\\.){3}[0-9]{1,3}", &body);
    let mut msg: String = String::new();
    if enabled == true {
        msg.push_str("Tor is up and running.");
    } else {
        msg.push_str("You are not using Tor.");
    }
    msg.push_str("\nYour IP appears to be ");
    msg.push_str(&ip);
    return (enabled, msg);
}
