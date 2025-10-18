use wifiscanner::{self, scan, Wifi};
use chrono::{self, Utc};

pub fn get_networks() -> Option<Vec<Wifi>> {
    let time_format: &'static str = "%Y-%m-%d %H:%M:%S";
    let scanner_result = scan();
    let current_time = Utc::now().format(time_format);
    match scanner_result {
        Ok(wifis) => {
            if wifis.get(0) != None {
                println!("Detected {} Networks at {}.", wifis.len(), current_time);
                return Some(wifis);
            } else {
                println!("No Networks detected.");
                println!("Please check your WiFi Adapter.");
                return None;
            }
        }
        Err(e) => {
            println!("Scan failed: {:?}", e);
            return None;
        }
    }
}

pub fn strength_by_ssid(ssid: String) -> f64 {
    let min_rssi = -100.0;
    let max_rssi = -35.0;
    let wifis = get_networks();
    for wifi_network in wifis.as_ref().unwrap() {
        if wifi_network.ssid == ssid {
            if let Ok(rssi) = wifi_network.signal_level.parse::<f64>() {
                let normalized = ((rssi - min_rssi) / (max_rssi - min_rssi)).clamp(0.0, 1.0);
                return normalized;
            }
        }
    }
    return 0.0;
}