use chrono::{DateTime, Local};
use rsntp::SntpClient;

fn main() {
    let client = SntpClient::new();
    let server = "time.google.com";
    let result = client.synchronize(server).unwrap();
    let local_time: DateTime<Local> =
        DateTime::from(result.datetime().into_chrono_datetime().unwrap());
    print!("Current time: {}", local_time);
}
