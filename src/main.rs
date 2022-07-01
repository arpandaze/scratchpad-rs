use anyhow::Result;
use config::Config;
use std::env;
use std::process;

mod config;
mod scratchpad;
mod socket;
mod yabai_schema;

fn main() -> Result<()> {
    match (env::args().nth(1), env::args().nth(2)) {
        (Some(command), Some(name)) => {
            if command == "--toggle" {
                let config = Config::get_config();

                let scratchpad = config
                    .scratchpads
                    .into_iter()
                    .find(|item| item.name == name);

                if scratchpad.is_none() {
                    eprintln!("Didn't find scratchpad named `{}`", name);
                    process::exit(0x1);
                }

                scratchpad.unwrap().toggle()?;
            }
        }
        _ => {
            eprintln!("Invalid arguments! Try `scratchpad --toggle example` to toggle scratchpad named `example`!");
            process::exit(0x1);
        }
    };
    Ok(())
}
