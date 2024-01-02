use std::{process::{Command, Stdio}, io::{BufReader, BufRead}};

pub fn exec(cmd: &mut Command) -> String {
    let cmd= cmd
        .stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
    if let Ok(output) = cmd.output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.trim().to_string();
    }
    return String::new();
}

pub fn exec_stdout(cmd: &mut Command) {
    let mut cmd = cmd
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();
        for line in stdout_lines {
            println!("Read: {:?}", line);
        }
    }
    cmd.wait().unwrap();
}
