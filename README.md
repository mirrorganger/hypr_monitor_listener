### Hypr-monitor-listener

Simple binary that changes the monitor configuration automatically in a system running [Hyperland Wayland Compositor](https://github.com/hyprwm/Hyprland).

The hyprland configuration should point to a monitor file

```bash
source = ~/.config/hypr/monitors/monitor.conf
```

In the end, this file will point to the desired monitor configuration file. The alternative configurations must be place under

```bash
~/.config/hypr/monitors
```

The binary listens to the hyprland socket and waits for monitor connection/disconnection events, adjusting the monitor configuration. The desired on_connect / on_disconnect behavior is specified with a config json file. 

#### Configuration file example

The configuration file, **monitors.json**, should be be placed under

```bash
~/.config/hypr-monitor-listener
```

The configuration file has the the following structure:

```json
[
  {"name" : "Samsung Electric Company SAMSUNG", "on_connect" : "home-one.conf", "on_disconnect" : "default.conf"},
  {"name" : "LG Electronics LG TV SSCR2 0x01010101", "on_connect" : "home-tv.conf", "on_disconnect" : "default.conf"}
]
```

To get the name of your monitor run

```bash
hyprctl monitor all
```

### Install and autostart

Once cloned, build and intall the binary by running

```bash
cargo install --path .
```

Once installed, you can autostart the binary by adding

```bash
exec-once hypr-monitor-listener &
```

to your hyprland config 

Note: personal project to learn how to use Rust for a simple task


[![Build and Test](https://github.com/mirrorganger/hypr-monitor-listener/actions/workflows/rust.yml/badge.svg)](https://github.com/mirrorganger/hypr-monitor-listener/actions/workflows/rust.yml)
