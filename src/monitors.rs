use log;
use serde::{Deserialize, Serialize};
use std::fs;

const BASE_CFG_STR: &str = "source = ~/.config/hypr/conf/monitors";
const DEFAULT_CFG: &str = "default.conf";

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitorConfig {
    pub name: String,
    pub on_connect: String,
    pub on_disconnect: String,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum MonitorEvent {
    Connected(String),
    Disconnected(String),
}

pub struct MonitorCfgWriter {
    pub file_name: String,
}

pub trait ConfigWriter {
    fn write(&self, config: &str);
}

impl ConfigWriter for MonitorCfgWriter {
    fn write(&self, config: &str) {
        let updated_content = format!("{}/{}", BASE_CFG_STR, config);
        match fs::write(&self.file_name, updated_content){
            Ok(()) => println!("Monitor cfg {} applied", config),
            Err(e) => println!(
                "Error {} writting to file {}",
                e,
                self.file_name
            ),
        }
    }
}

pub struct MonitorListener<W: ConfigWriter> {
    pub monitors: Vec<MonitorConfig>,
    pub monitor_count: u8,
    pub writer: W,
}

pub trait EventMoniterListener {
    fn monitor_event(&mut self, event: MonitorEvent);
}

impl<W: ConfigWriter> EventMoniterListener for MonitorListener<W> {
    fn monitor_event(&mut self, event: MonitorEvent) {
        match event {
            MonitorEvent::Connected(name) => {
                for monitor in self.monitors.iter() {
                    if monitor.name == name {
                        log::info!("monitor {} connected", name);
                        self.writer.write(&monitor.on_connect);
                        self.monitor_count += 1;
                    }
                }
            }
            MonitorEvent::Disconnected(name) => {
                log::info!("monitor {} disconnected", name);
                if self.monitor_count >= 1 {
                    self.monitor_count -= 1;
                }
                if self.monitor_count == 0 {
                    self.writer.write(DEFAULT_CFG);
                }
            }
        }
    }
}
