use std::env;
use std::process::exit;

fn main() {
    if let Some(url) = env::args().nth(1) {
        println!("health_check url={}", url);

        let response = minreq::get(url).send();
        match response {
            Ok(res) => {
                if res.status_code != 200 {
                    println!("health_check status={}", res.status_code);
                    exit(1)
                }
                exit(0)
            }
            Err(err) => {
                println!("Error:{}", err);
                exit(1);
            }
        }
    } else {
        println!("health_check url empty!");
        exit(1);
    }
}
