extern crate regex;
extern crate ureq;

use self::regex::Regex;

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

pub fn fetch_url(url: &str) -> (u16, String) {
    // sync post request of some json.
    let mut req = ureq::get(url)
        .set("X-My-Header", "Secret")
        .set("Accept", "text/plain")
        .timeout_connect(2000)
        .to_owned();
    let response = req.call();
    let status = response.status();
    let body = response.into_string().unwrap();
    return (status, body);
}
