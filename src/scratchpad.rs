use crate::socket::query_socket;
use crate::yabai_schema::{Space, Window};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use shlex::split;
use std::process::Command as SysCommand;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum Target {
    Title(String),
    App(String),
}

#[derive(Debug, Clone)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}

impl From<[u16; 2]> for Coordinate {
    fn from(nums: [u16; 2]) -> Self {
        return Self {
            x: nums[0],
            y: nums[1],
        };
    }
}

impl ToString for Coordinate {
    fn to_string(&self) -> String {
        return format!("{}:{}:{}", "abs", self.x, self.y);
    }
}

#[derive(Debug, Clone)]
pub struct Scratchpad {
    pub name: String,
    pub target: Target,
    pub position: Coordinate,
    pub size: Coordinate,
    pub launch_command: LaunchOption,
    pub launch_timeout: u8,
    pub scratchpad_space: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LaunchOption {
    Application(String),
    ApplicationWithArgs(String, Vec<String>),
    Command(String),
}

impl Scratchpad {
    pub fn toggle(&self) -> Result<()> {
        let mut target_window_opt = self.get_target_window()?;

        if target_window_opt.is_none() {
            let timer = Instant::now();
            self.launch()?;

            while target_window_opt.is_none() {
                target_window_opt = self.get_target_window()?;

                if timer.elapsed() > Duration::from_secs(self.launch_timeout as u64) {
                    panic!("Application didn't launch within timeout period!");
                }

                thread::sleep(Duration::from_millis(100));
            }
        }

        let target_window = target_window_opt.unwrap();
        let window_id = target_window.id;

        // If window already has focus, send it to scratchpad workspace
        if target_window.has_focus {
            query_socket(&[
                "window",
                &window_id.to_string(),
                "--space",
                &self.scratchpad_space.to_string(),
            ])?;
            return Ok(());
        }

        let focused_space_id = self.get_focused_space()?.unwrap().index;

        // Set window to floating if it isn't
        if !target_window.is_floating {
            query_socket(&["window", &window_id.to_string(), "--toggle", "float"])?;
        }

        // Move target window to focused space
        query_socket(&[
            "window",
            &window_id.to_string(),
            "--space",
            &focused_space_id.to_string(),
        ])?;

        // Move target window to target position
        query_socket(&[
            "window",
            &window_id.to_string(),
            "--move",
            &self.position.to_string(),
        ])?;

        // Resize target window to target size
        query_socket(&[
            "window",
            &window_id.to_string(),
            "--resize",
            &self.size.to_string(),
        ])?;

        // Focus the target window
        query_socket(&["window", "--focus", &window_id.to_string()])?;

        return Ok(());
    }

    pub fn launch(&self) -> Result<()> {
        match &self.launch_command {
            LaunchOption::Application(application) => SysCommand::new("open")
                .arg("-n")
                .arg(format!("/Applications/{}", application))
                .spawn()?,

            LaunchOption::ApplicationWithArgs(application, args) => SysCommand::new("open")
                .arg("-n")
                .arg(format!("/Applications/{}", application))
                .arg("--args")
                .args(args)
                .spawn()?,

            LaunchOption::Command(command) => {
                let splitted_cmd = split(command).expect("Invalid command!");

                SysCommand::new(&splitted_cmd[0])
                    .args(&splitted_cmd[1..])
                    .spawn()?
            }
        };

        Ok(())
    }

    pub fn get_target_window(&self) -> Result<Option<Window>> {
        let windows = Self::get_windows()?;

        let window = match &self.target {
            Target::Title(title) => windows.iter().find(|item| item.title == title.as_str()),
            Target::App(app) => windows.iter().find(|item| item.app == app.as_str()),
        };

        match window {
            Some(target_window) => {
                return Ok(Some(target_window.clone()));
            }
            None => return Ok(None),
        }
    }

    pub fn get_focused_space(&self) -> Result<Option<Space>> {
        let spaces = Self::get_spaces()?;

        let space = spaces.iter().find(|item| item.has_focus);

        match space {
            Some(focused_space) => {
                return Ok(Some(focused_space.clone()));
            }
            None => return Ok(None),
        }
    }

    pub fn get_windows() -> Result<Vec<Window>> {
        let string_data = query_socket(&["query", "--windows"])?;

        let deser_data = serde_json::from_str::<Vec<Window>>(&string_data)?;
        return Ok(deser_data);
    }

    pub fn get_spaces() -> Result<Vec<Space>> {
        let string_data = query_socket(&["query", "--spaces"])?;

        let deser_data = serde_json::from_str::<Vec<Space>>(&string_data)?;
        return Ok(deser_data);
    }
}
