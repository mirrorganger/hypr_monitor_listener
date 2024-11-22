use std::fs;

mod monitors;
mod parser;
mod socket_listener;

fn main() {
    let json_path = "data/monitors.json".to_owned();
    let json_str = fs::read_to_string(json_path).expect("Couldn't load the file");

    let mut listener = monitors::MonitorListener {
        monitors: parser::parse_type::<monitors::MonitorConfig>(&json_str),
        monitor_count: 0,
    };

    match socket_listener::read_socket(
        socket_listener::get_hyper_socket().expect("Could not get socket"),
        &mut listener,
    ) {
        Ok(()) => {}
        Err(..) => {}
    };
}
