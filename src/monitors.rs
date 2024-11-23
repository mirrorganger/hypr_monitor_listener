use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitorConfig {
    pub name: String,
    pub on_connect: String,
    pub on_disconnect: String,
}

#[allow(dead_code)]
pub enum MonitorEvent {
    Connected,
    Disconnected,
}

pub struct MonitorListener {
    pub monitors: Vec<MonitorConfig>,
    pub monitor_count: u8,
}

impl MonitorListener {
    const FILE_PATH: &str = "/home/cesar/.config/hypr/conf/monitor.conf";
    const BASE_CFG_STR: &str = "source = ~/.config/hypr/conf/monitors";
    const DEFAULT_CFG: &str = "default.conf";
    pub fn monitor_event(&mut self, name: &str, event: MonitorEvent) {
        match event {
            MonitorEvent::Connected => {
                for monitor in self.monitors.iter() {
                    if monitor.name == name {
                        println!("monitor {} connected", name);
                        self.update_monitor_config(&monitor.on_connect);
                        self.monitor_count += 1;
                    }
                }
            }
            MonitorEvent::Disconnected => {
                if self.monitor_count >= 1 {
                    self.monitor_count -= 1;
                }
                if self.monitor_count == 0 {
                    self.update_monitor_config(MonitorListener::DEFAULT_CFG);
                }
            }
        }
    }

    fn update_monitor_config(&self, config: &str) {
        let updated_content = format!("{}/{}", MonitorListener::BASE_CFG_STR, config);
        match fs::write(MonitorListener::FILE_PATH, updated_content) {
            Ok(()) => println!("Monitor cfg {} applied", config),
            Err(e) => println!(
                "Error {} writting to file {}",
                e,
                MonitorListener::FILE_PATH
            ),
        }
    }
}
