use std::{
    ffi::OsStr,
    path::Path,
    process::{Child, Command, Stdio},
};

pub mod handler;
pub mod request;

pub fn spawn_from_file(file_name: &str) -> Option<Child> {
    let path = Path::new(file_name);
    let mut cmd;
    match path.extension().and_then(OsStr::to_str) {
        Some("js") | Some("ts") => {
            cmd = Command::new("deno");
            cmd.arg(file_name);
        }
        Some("java") => {
            cmd = Command::new("java");
            cmd.args(["-cp", "userlib/java", file_name]);
        }
        Some(e) => {
            println!("[\x1b[31m{file_name}\x1b[0m] Unknown file extension: {e}");
            return None;
        }
        None => {
            cmd = Command::new(file_name);
        }
    };
    match cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(c) => Some(c),
        Err(e) => {
            println!("[\x1b[31m{file_name}\x1b[0m] Error running file: {e}");
            None
        }
    }
}
