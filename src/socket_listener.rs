use crate::monitors;
use crate::monitors::MonitorEvent;
use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::os::unix::net::UnixStream;

const MONITOR_ADDED_STR: &str = "monitoraddedv2";
const MONITOR_REMOVED_STR: &str = "monitorremoved";

pub fn read_socket(
    socket_addr: String,
    listener: &mut dyn monitors::EventMoniterListener,
) -> std::io::Result<()> {
    let stream = match UnixStream::connect(socket_addr) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Couldn't connect : {e:?}");
            return Err(e);
        }
    };
    let mut reader = BufReader::new(stream);
    loop {
        let mut buff: Vec<u8> = vec![];
        reader.read_until(b'\n', &mut buff).unwrap();
        let data = String::from_utf8_lossy(&buff);
        if let Some(event) = parse_hypr_stream(&data) {
            listener.monitor_event(event);
        }
    }
}

fn parse_hypr_stream(line: &str) -> Option<MonitorEvent> {
    let parts: Vec<&str> = line.trim().split(">>").collect();
    if parts.is_empty() {
        return None;
    }
    match parts[0] {
        MONITOR_ADDED_STR => {
            let info: Vec<&str> = parts[1].split(",").collect();
            Some(MonitorEvent::Connected(info[2].to_string()))
        }
        MONITOR_REMOVED_STR => {
            if parts[1] != "FALLBACK" && parts[1] != "eDP-1" {
                Some(MonitorEvent::Disconnected(parts[1].to_string()))
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn get_hyper_socket() -> Option<String> {
    let key = "HYPRLAND_INSTANCE_SIGNATURE";
    match env::var(key) {
        Ok(hyper_inst) => {
            let default_socket = format!("/tmp/hypr/{}/.socket2.sock", hyper_inst);
            let socket_addr = match env::var("XDG_RUNTIME_DIR") {
                Ok(runtime_dir) => match std::fs::metadata(format!(
                    "{}/hypr/{}/.socket2.sock",
                    runtime_dir, hyper_inst
                )) {
                    Ok(_) => format!("{}/hypr/{}/.socket2.sock", runtime_dir, hyper_inst),
                    Err(..) => default_socket,
                },
                Err(..) => default_socket,
            };
            println!("{ }", socket_addr);
            Some(socket_addr)
        }
        Err(..) => None,
    }
}

#[cfg(test)]
mod test {
    use super::parse_hypr_stream;
    use crate::monitors::MonitorEvent;
    use crate::socket_listener::MONITOR_ADDED_STR;
    use crate::socket_listener::MONITOR_REMOVED_STR;
    #[test]
    fn test_parser_monitor_connect() {
        let monitor_name: &str = "monitor_1";
        let input_str = format!("{}>>1,DP-3,{}", MONITOR_ADDED_STR, monitor_name);
        assert_eq!(parse_hypr_stream(""), None); // empty string
        assert_eq!(
            parse_hypr_stream(&input_str),
            Some(MonitorEvent::Connected(monitor_name.to_string()))
        );
        assert_eq!(
            parse_hypr_stream(&format!("{}>>xxx", MONITOR_REMOVED_STR)),
            Some(MonitorEvent::Disconnected("xxx".to_string()))
        );
    }
}
