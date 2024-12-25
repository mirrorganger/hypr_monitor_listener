use std::env;
use std::fs;
use std::path::Path;

mod monitors;
mod socket_listener;

#[cfg(debug_assertions)]
const FILE_PATH: &str = "test.conf";

#[cfg(not(debug_assertions))]
const FILE_PATH: &str = "/home/cesar/.config/hypr/conf/monitor.conf";

fn main() {
    env_logger::init();
    if let Some(home_dir) = env::var_os("HOME") {
        let json_path = Path::new(&home_dir).join(".config/hypr-monitor-listener/monitors.json");
        let json_str = fs::read_to_string(json_path).expect("Couldn't load the file");

        let mut config_writer = monitors::MonitorCfgWriter {
            file_name: FILE_PATH.to_string(),
        };
        let mut listener = monitors::MonitorListener {
            monitors: serde_json::from_str(&json_str).unwrap_or_default(),
            monitor_count: 0,
            writer: &mut config_writer,
        };
        listener.print_config();
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
