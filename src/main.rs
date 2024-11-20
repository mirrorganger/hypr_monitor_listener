use std::fs;

mod monitors;
mod parser;
mod socket_listener;

fn main() {
    let json_path = "data/monitors.json".to_owned();
    let json_str = fs::read_to_string(json_path).expect("Couldn't load the file");

    let listener = monitors::MonitorListener {
        monitors: parser::parse_type::<monitors::MonitorConfig>(&json_str),
    };
    let result = listener.monitor_event("monitor_1", monitors::MonitorEvent::Connected);

    match result {
        Some(r) => println!("on connected {}", r),
        None => println!("no result"),
    }

    //}
}
