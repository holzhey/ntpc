use std::time::UNIX_EPOCH;

use rsntp::SntpClient;

fn main() {
    let client = SntpClient::new();
    let servers = vec![
        "time.google.com",
        "time-a-wwv.nist.gov",
        "time-a-b.nist.gov",
        "time.nist.gov",
        "utcnist.colorado.edu",
        "pool.ntp.org",
    ];
    for server in servers {
        let local_time: String = get_local_time_from_ntp_server(&client, server);
        println!("Current time from {}: {}", server, local_time);
    }
}

fn get_local_time_from_ntp_server(client: &SntpClient, server: &str) -> String {
    match client.synchronize(server) {
        Ok(result) => {
            let ldt = result.datetime().into_system_time().unwrap();
            format!("{:?}", ldt.duration_since(UNIX_EPOCH))
        }
        Err(error) => error.to_string(),
    }
}
