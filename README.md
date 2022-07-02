# Yabai Scratchpad
Scratchpads are essential for my workflow, but Yabai doesn't have scratchpad feature. I wanted to replicate the functionality of the LeftWM Scratchpad that I use on Linux. Result is this simple yet extremely useful CLI app.

## Demo
https://user-images.githubusercontent.com/46302068/177003301-54cf74ad-af72-4297-842e-68ba203c894b.mp4 

## Installation Instruction
* **Step 1:** Clone the repository
* **Step 2:** `cargo build --release`
* **Step 3:** `cp target/release/scratchpad /opt/homebrew/bin/` to copy `scratchpad` to path.
* **Step 4:** Define configuration by creating `$HOME/.config/scratchpad/config.toml`
* **Step 5:** Create SKHD config. eg. `alt - t : scratchpad --toggle telegram`

*Note: Copying binary to `/usr/local/bin` won't work with SKHD*

## Usage
```
scratchpad --toggle {name}
```

## Config Options

| Field            | Description                        | Example                              |Default   |
|------------------|------------------------------------|--------------------------------------|----------|
|`name`            | Name of scratchpad                 | `calculator`                         | N/A      |
|`target_type`     | Type of target                     | {`app`,`title`}                      | `app`    |
|`target`          | Target app name or title           | `Discord`                            | N/A      |
|`position`        | Position where scratchpad shows up | `[290, 175]`                         | N/A      |
|`size`            | Size of scratchpad                 | `[1100, 700]`                        | N/A      |
|`launch_type`     | Type of launch method              | {`app`,`app_with_arg`, `command`}    | `command`|
|`launch_command`  | Command or name of application     | `open -n /Applications/Alacritty.app`| N/A      |

## Configuration Example
```
scratchpad_space = 8
launch_timeout = 5

[[scratchpad]]
name = "telegram"
target_type = "app"
target = "Telegram"
position = [290, 175]
size = [1100, 700]
launch_type = "app"
launch_command = "Telegram.app"

[[scratchpad]]
name = "alacritty"
target_type = "title"
target = "AlacrittyScratchpad"
position = [290, 175]
size = [1100, 700]
launch_type = "app_with_arg"
launch_command = ["Alacritty.app", "--title", "AlacrittyScratchpad"]

[[scratchpad]]
name = "discord"
target_type = "app"
target = "Discord"
position = [290, 175]
size = [1100, 700]
launch_type = "app"
launch_command = "Discord.app"
```
