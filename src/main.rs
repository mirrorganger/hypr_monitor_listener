use std::env;
use std::fs;
use std::path::Path;
mod monitors;
mod socket_listener;

fn main() {
    if let Some(home_dir) = env::var_os("HOME") {
        let json_path = Path::new(&home_dir).join(".config/hypr-monitor-listener/monitors.json");
        let json_str = fs::read_to_string(json_path).expect("Couldn't load the file");

        let mut listener = monitors::MonitorListener {
            monitors: serde_json::from_str(&json_str).unwrap_or_default(),
            monitor_count: 0,
        };

        match socket_listener::read_socket(
            socket_listener::get_hyper_socket().expect("Could not get socket"),
            &mut listener,
        ) {
            Ok(()) => {}
            Err(e) => println!("Error reading the socket {e:?}"),
        };
    } else {
        println!("Could not get home directory");
    }
}
