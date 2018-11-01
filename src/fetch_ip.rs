use util;

#[derive(Debug, Clone)]
pub struct Fetch {
    pub url: String,
    pub body: String,
    pub status: u16,
    pub ip: String,
    pub valid: bool,
}

impl Fetch {
    pub fn init(url: &str) -> Self {
        let response = Self::get(&url);
        let ip = Self::extract_ip(&response.1);
        let valid = Self::validate_ip(&ip, &response.0);
        return Fetch {
            url: url.to_string(),
            body: response.1,
            status: response.0,
            ip: ip,
            valid: valid,
        };
    }

    fn get(url: &str) -> (u16, String) {
        return util::fetch_url(url);
    }

    fn extract_ip(s: &str) -> String {
        return util::rx_find("(?:[0-9]{1,3}\\.){3}[0-9]{1,3}", s);
    }

    fn validate_ip(s: &str, r: &u16) -> bool {
        if util::rx_match(
            "^(25[0-5]|2[0-4][0-9]|[0-1]{1}[0-9]{2}|[1-9]{1}[0-9]{1}|[1-9])\\.(25[0-5]|2[0-4][0-9]|[0-1]{1}[0-9]{2}|[1-9]{1}[0-9]{1}|[1-9]|0)\\.(25[0-5]|2[0-4][0-9]|[0-1]{1}[0-9]{2}|[1-9]{1}[0-9]{1}|[1-9]|0)\\.(25[0-5]|2[0-4][0-9]|[0-1]{1}[0-9]{2}|[1-9]{1}[0-9]{1}|[0-9])$",
            s,
        ) == true && r == &200{
            return true
        }else{
            return false
        }
    }
}
