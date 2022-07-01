use crate::scratchpad::{LaunchOption, Scratchpad, Target};
use std::fs::File;
use std::io::Read;
use std::process;
use toml::Value;

#[derive(Debug, Clone)]
pub struct Config {
    pub launch_timeout: u8,
    pub scratchpad_space: u8,
    pub scratchpads: Vec<Scratchpad>,
}

impl Config {
    pub fn get_config() -> Config {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("scratchpad").unwrap();
        let config_path = xdg_dirs
            .place_config_file("config.toml")
            .expect("cannot create configuration directory");

        if !config_path.exists() {
            eprintln!("Couldn't find scratchpad config file!");
            process::exit(0x1);
        }

        let mut config_file = File::open(config_path).expect("Coulnd't open the config file!");
        let mut config_string = String::new();

        config_file
            .read_to_string(&mut config_string)
            .expect("Failed to read config file!");

        return Config::from(config_string.as_str());
    }
}

impl From<&str> for Config {
    fn from(config_string: &str) -> Self {
        let config: Value = toml::from_str(config_string).expect("Invalid config!");

        let mut launch_timeout: u8 = 10;
        let mut scratchpad_space: u8 = 8;

        match config.get("launch_timeout") {
            Some(value) => launch_timeout = value.as_integer().unwrap() as u8,
            _ => (),
        }

        match config.get("scratchpad_space") {
            Some(value) => scratchpad_space = value.as_integer().unwrap() as u8,
            _ => (),
        }

        let scratchpad_values = config
            .get("scratchpad")
            .expect("No scratchpad defined in config!");

        let scratchpad = scratchpad_values
            .as_array()
            .unwrap()
            .iter()
            .map(|item| {
                let name = item
                    .get("name")
                    .expect("Name for scratchpad must be given!")
                    .as_str()
                    .unwrap()
                    .to_string();

                let target_str = item
                    .get("target")
                    .expect("Target for scratchpad must be given!")
                    .as_str()
                    .unwrap()
                    .to_string();

                let target = match item.get("target_type") {
                    Some(Value::String(value)) => match value.as_str() {
                        "app" => Target::App(target_str),
                        "title" => Target::Title(target_str),
                        _ => Target::App(target_str),
                    },
                    _ => Target::App(target_str),
                };

                let position_value = item
                    .get("position")
                    .expect("Position of scratchpad is required!")
                    .as_array()
                    .unwrap();

                let position: [u16; 2] = position_value
                    .iter()
                    .map(|item| item.as_integer().unwrap() as u16)
                    .collect::<Vec<u16>>()
                    .try_into()
                    .expect("Invalid position configuration!");

                let size_value = item
                    .get("size")
                    .expect("Size of scratchpad is required!")
                    .as_array()
                    .unwrap();

                let size: [u16; 2] = size_value
                    .iter()
                    .map(|item| item.as_integer().unwrap() as u16)
                    .collect::<Vec<u16>>()
                    .try_into()
                    .expect("Invalid size configuration!");

                let launch_command_value = item
                    .get("launch_command")
                    .expect("Launch command is required!");

                let launch_type_str = match item.get("launch_type") {
                    Some(Value::String(value)) => value.as_str(),
                    _ => "application",
                };

                let launch_command = if launch_type_str == "application_with_arg" {
                    let tokens = launch_command_value
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|item| item.as_str().unwrap().to_string())
                        .collect::<Vec<String>>();

                    LaunchOption::ApplicationWithArgs(tokens[0].clone(), tokens[1..].to_vec())
                } else {
                    let command_string = launch_command_value
                        .as_str()
                        .expect("Invalid launch command!")
                        .to_string();

                    match launch_type_str {
                        "application" => LaunchOption::Application(command_string),
                        "command" => LaunchOption::Command(command_string),
                        _ => LaunchOption::Application(command_string),
                    }
                };

                Scratchpad {
                    name,
                    target,
                    position: position.into(),
                    size: size.into(),
                    launch_command,
                    launch_timeout,
                    scratchpad_space,
                }
            })
            .collect::<Vec<Scratchpad>>();

        Config {
            launch_timeout,
            scratchpad_space,
            scratchpads: scratchpad,
        }
    }
}
