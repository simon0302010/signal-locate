use wifiscanner;

fn main() {
    let scanner_result = wifiscanner::scan();
    match scanner_result {
        Ok(wifis) => {
            if wifis.get(0) != None {
                println!("Detected Networks: ");
                for single_wifi in wifis {
                    println!("{}", single_wifi.ssid)
                }
            }
        }
        Err(e) => {
            println!("Scan failed: {:?}", e)
        }
    }
}
