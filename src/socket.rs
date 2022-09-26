use anyhow::Result;
use std::env;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::str;
use std::time::Duration;

fn format_message(message: &[&str]) -> Vec<u8> {
    let mut command = Vec::from([0x0, 0x0, 0x0, 0x0]);

    for token in message {
        command.append(&mut token.as_bytes().to_vec());
        command.push(0x0);
    }
    command.push(0x0);

    // First byte must denote number of bytes in message part minus the padding
    // fixme: First bytes denotes size but only first byte is being used here
    // Reference: https://github.com/koekeishiya/yabai/issues/1372#issuecomment-1226701120
    command[0] = (command.len() - 4) as u8;

    return command;
}

pub fn get_socket_stream() -> Result<UnixStream> {
    let socket_path = format!("/tmp/yabai_{}.socket", env::var("USER")?);

    // Check if Yabai socket exists
    if !Path::new(&socket_path).exists() {
        panic!("Yabai socket doesn't exists! Is Yabai installed and running?");
    }

    // Connect to the Yabai socket
    let stream = UnixStream::connect(socket_path)?;

    // Set read write timeout for socket
    stream.set_read_timeout(Some(Duration::new(2, 0)))?;
    stream.set_write_timeout(Some(Duration::new(2, 0)))?;

    return Ok(stream);
}

pub fn query_socket(message: &[&str]) -> Result<String> {
    let mut socket_stream = get_socket_stream()?;

    let formatted_msg = format_message(message);

    socket_stream.write_all(&formatted_msg)?;

    let mut response = Vec::new();
    socket_stream.read_to_end(&mut response)?;

    let string_data = String::from_utf8(response)?;

    return Ok(string_data);
}
