use serde::{Deserialize, Serialize};
use std::fs;

const BASE_CFG_STR: &str = "source = ~/.config/hypr/monitors";
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
    fn write(&mut self, config: &str);
}

impl ConfigWriter for MonitorCfgWriter {
    fn write(&mut self, config: &str) {
        let updated_content = format!("{}/{}", BASE_CFG_STR, config);
        match fs::write(&self.file_name, updated_content) {
            Ok(()) => log::debug!("Monitor cfg {} applied", config),
            Err(e) => log::error!("Error {} writting to file {}", e, self.file_name),
        }
    }
}

pub struct MonitorListener<'a, W>
where
    W: ConfigWriter + 'a,
{
    pub monitors: Vec<MonitorConfig>,
    pub monitor_count: u8,
    pub writer: &'a mut W,
}

pub fn parse_hypr_monitor_output(output: &str) -> Vec<String> {
    output
        .lines()
        .filter(|line| line.trim().contains("description"))
        .map(|line| line.split_once(":").unwrap().1.trim().to_string())
        .collect()
}

pub trait EventMoniterListener {
    fn monitor_event(&mut self, event: MonitorEvent);
    fn init(&mut self, monitors: &Vec<String>);
}

impl<'a, W> MonitorListener<'a, W>
where
    W: ConfigWriter + 'a,
{
    pub fn print_config(&self) {
        for monitor in self.monitors.iter() {
            log::debug!("Monitor: {}", monitor.name);
            log::debug!("On connect: {}", monitor.on_connect);
            log::debug!("On disconnect: {}", monitor.on_disconnect);
        }
    }
}

impl<'a, W> EventMoniterListener for MonitorListener<'a, W>
where
    W: ConfigWriter + 'a,
{
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
                log::debug!("monitor {} disconnected", name);
                if self.monitor_count >= 1 {
                    self.monitor_count -= 1;
                }
                if self.monitor_count == 0 {
                    self.writer.write(DEFAULT_CFG);
                }
            }
        }
    }

    fn init(&mut self, monitors: &Vec<String>) {
        println!("Writing default config");
        self.writer.write(DEFAULT_CFG);
        for monitor in monitors {
            for m in self.monitors.iter() {
                if m.name == *monitor {
                    println!("Writing monitor config {}", m.on_connect);    
                    self.writer.write(&m.on_connect);
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Mock {
        configs: Vec<String>,
    }
    impl ConfigWriter for Mock {
        fn write(&mut self, config: &str) {
            self.configs.push(config.to_string());
        }
    }

    #[test]
    fn test_monitor_cfg_sent() {
        let mut mock_writer = Mock {
            configs: Vec::new(),
        };

        let mut monitor_event_listener = MonitorListener {
            monitors: vec![
                MonitorConfig {
                    name: "monitor1".to_string(),
                    on_connect: "monitor1.conf".to_string(),
                    on_disconnect: "default.conf".to_string(),
                },
                MonitorConfig {
                    name: "monitor2".to_string(),
                    on_connect: "monitor2.conf".to_string(),
                    on_disconnect: "default.conf".to_string(),
                },
            ],
            monitor_count: 0,
            writer: &mut mock_writer,
        };
        monitor_event_listener.monitor_event(MonitorEvent::Connected("monitor1".to_string()));
        monitor_event_listener.monitor_event(MonitorEvent::Disconnected("".to_string()));

        assert_eq!(mock_writer.configs.len(), 2);
        assert_eq!(mock_writer.configs[0], "monitor1.conf");
        assert_eq!(mock_writer.configs[1], DEFAULT_CFG);
    }
}
