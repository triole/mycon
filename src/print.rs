use fetch_ip;

pub fn print(result: fetch_ip::Fetch) {
    if result.valid == true {
        println!("Reponse from: {}", result.url);
        println!(
            "Duration    : {seconds},{millis:>0width$} sec",
            seconds = result.duration.as_secs(),
            millis = result.duration.subsec_millis(),
            width = 3
        );
        println!("External IP : {}", result.ip);
    } else {
        println!("{}", "Error, none of the requests returned a valid result.");
    }
}
