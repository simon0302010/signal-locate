use wifiscanner::{self, scan};
use chrono::{self, Utc};

fn main() {
    let time_format = "%Y-%m-%d %H:%M:%S";
    let mut current_time;
    let mut scanner_result;
    loop {
        scanner_result = scan();
        current_time = Utc::now().format(time_format);
        println!("Time: {}", current_time);
        match scanner_result {
            Ok(wifis) => {
                if wifis.get(0) != None {
                    println!("\nDetected {} Networks at {}: ", wifis.len(), current_time);
                    for single_wifi in &wifis {
                        println!("{}", single_wifi.ssid)
                    }
                }
            }
            Err(e) => {
                println!("Scan failed: {:?}", e)
            }
        }
    }
}
