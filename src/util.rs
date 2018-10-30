extern crate regex;
extern crate ureq;

use self::regex::Regex;

pub fn get_ip(s: &str) -> String {
    return rx_find("(?:\\\\d{1,3}\\\\.){3}\\\\d{1,3}", s);
}

fn rx_find(rx_str: &str, searched_str: &str) -> String {
    let re = Regex::new(rx_str).unwrap();
    let caps = re.captures(searched_str);
    match caps {
        Some(caps) => caps[0].to_string(),
        None => String::new(),
    }
}

pub fn fetch_url(url: &str) -> String {
    // sync post request of some json.
    let mut req = ureq::get(url)
        .set("X-My-Header", "Secret")
        .set("Accept", "text/plain")
        .timeout_connect(2000)
        .to_owned();
    let response = req.call();
    let r = response.into_string().unwrap();
    return r;
}
