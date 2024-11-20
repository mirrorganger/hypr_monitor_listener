use serde::{Deserialize, Serialize};

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
}

impl MonitorListener {
    pub fn monitor_event(&self, name: &str, event: MonitorEvent) -> Option<String> {
        for monitor in self.monitors.iter() {
            if monitor.name == name {
                return Some(match event {
                    MonitorEvent::Connected => monitor.on_connect.clone(),
                    MonitorEvent::Disconnected => monitor.on_disconnect.clone(),
                });
            }
        }
        None
    }
}
