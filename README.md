# MonitorMaestro
Tired of fiddling with Hyprland configs?  

Monitor Maestro lets you switch between monitor layouts in a snap!  

This Rust-powered TUI app lets you define layouts in a simple JSON file and switch between them with a few keystrokes.  

Effortless Workflow:
- Define layouts in JSON.
- Simple TUI for easy selection.
- Switch layouts instantly.

The TUI runs on a terminal, so it's up to you to add a `bind` in you Hyprland configs 

## Installation
Clone the repo, build with `cargo` and copy the executable wherever you choose.

## Configuration
An example workspaces configuration json [file](./workspaces_example.json) is provided
A monitor can either be Enabled, and requires the fields:
- dimensions
- refresh rate
- position
- scaling

Or Disabled

