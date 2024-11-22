use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::os::unix::net::UnixStream;

use crate::monitors;

pub fn read_socket(
    socket_addr: String,
    listener: &mut monitors::MonitorListener,
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
        let parts: Vec<&str> = data.trim().split(">>").collect();
        if parts[0] == "monitoraddedv2" {
            let info: Vec<&str> = parts[1].split(",").collect();
            listener.monitor_event(info[2], monitors::MonitorEvent::Connected);
        } else if parts[0] == "monitorremoved" {
            listener.monitor_event(parts[1], monitors::MonitorEvent::Disconnected);
        }
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
