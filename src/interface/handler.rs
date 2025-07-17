use std::{
    io::{BufRead, BufReader},
    process::{Child, ChildStdout},
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
};

use super::request::Request;

pub fn spawn_handler(
    name: String,
    color_code: String,
    process: &mut Child,
    sender: Sender<Request>,
) -> JoinHandle<()> {
    let mut stdout_reader = BufReader::new(process.stdout.take().expect("Failed to create stdout"));
    thread::spawn(move || {
        loop {
            let Some(string) = try_read(&name, &color_code, &mut stdout_reader) else {
                println!("[\x1b[31m{name}\x1b[0m] Crashed.");
                break;
            };
            let mut words = string[1..].trim().split(" ");

            let Ok(request) = Request::try_from(&mut words) else {
                println!("[\x1b[31m{name}\x1b[0m] Invalid request: {string}");
                continue;
            };
            sender.send(request).unwrap();
        }
    })
}

fn try_read(
    name: &str,
    color_code: &str,
    stdout_reader: &mut BufReader<ChildStdout>,
) -> Option<String> {
    let mut string = String::new();
    stdout_reader.read_line(&mut string).ok()?;
    while !string.starts_with("\0") {
        if string.is_empty() {
            return None;
        }

        println!("[\x1b[{color_code}m{name}\x1b[0m] {}", string.trim());
        string.clear();
        stdout_reader.read_line(&mut string).ok()?;
    }
    Some(string)
}
