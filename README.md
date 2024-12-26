### hypr-monitor-listener

This is personal project to learn how to use Rust for a simple task.

The idea of the project is to change the monitor configuration based on configuration file.

The system must use hyperland as window manager, and the dotfiles from

[My Linux For Work - dotfiles](https://github.com/mylinuxforwork/dotfiles)

The binary listens to the hyprland socket and waits for monitor connection/disconnection events, adjusting the monitor configuration.

The desired behavior is specified with a json file. 

#### Configuration file example

The configuration file is a json file with the following structure:

```json
[
  {"name" : "Samsung Electric Company SAMSUNG", "on_connect" : "home-one.conf", "on_disconnect" : "on_disconnect_A"},
  {"name" : "LG Electronics LG TV SSCR2 0x01010101", "on_connect" : "home-tv.conf", "on_disconnect" : "on_disconnect_B"}
]
```
The configuration file **monitors.json** should be be placed under **$/.config/hypr-monitor-listener/**

#### On connect configurations

The on-connect configurations files must exists under **$/.config/hypr/conf/monitors/**
