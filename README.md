# MonitorMaestro
Tired of fiddling with Hyprland configs?  

Monitor Maestro lets you switch between monitor layouts in a snap!  

This Rust-powered TUI app lets you define layouts in a simple TOML or JSON file and switch between them with a few keystrokes.  

Effortless Workflow:
- Define layouts in TOML or JSON.
- Simple TUI for easy selection.
- Switch layouts instantly.

The TUI runs on a terminal, so it's up to you to add a `bind` in you Hyprland configs  
If you want an example, take a look at my Hyprlad [dotfiles](https://github.com/Degra02/dotfiles/blob/master/hypr/hyprland.conf) and search for `monitor_maestro`

## Functionality
The program has 3 execution options:  
- __tui__ \<MODE\>: start the TUI, either in:  
    - __list__: list workspaces from config (specify with `-p / --path`) file and select one  
    - __interactive__: draw current monitors with rectangles (needs a larger terminal window)  
- __workspace__ \<NAME\>: no tui, only run the commands to start the specified workspace  
- __state__: get the current workspace from the file `/tmp/monitor_maestro_state.txt`, auto-handled  

## Keybinds
Tui in mode:  
__list__:  
    - `j - k`: go down - up the list  
    - `q - ESC`: exit   
    - `Enter`: select monitor layout  

__interactive__:    
    - `h - l`: go left - right the monitor rectangles  
    - `q - ESC`: exit  
    - `Enter`: WIP  


## Installation
Clone the repo, then build with `cargo` 
```bash
cargo build --release
```
and copy the executable `monitor_maestro` wherever you choose.

## Configuration
An example workspaces configuration toml [file](./workspaces_example.toml) is provided  
Also json [file](./workspaces_example.json).  
A monitor can either be:  
- __Enabled__, and requires the fields:  
    - dimensions  
    - refresh rate  
    - position  
    - scaling  

- __Disabled__


## Milestones
- [X] Dynamic TUI showing rectangles as current monitor layout, in a pseudo realistic way  
- [X] Toml configuration
- [ ] TUI for interactive monitor layout creation and json output
