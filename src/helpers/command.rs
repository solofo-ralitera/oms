use std::{process::{Command, Stdio}, io::{BufReader, BufRead}, ffi::OsStr};

pub fn exec<I, S> (command: &str, args: I) -> String
where
I: IntoIterator<Item = S>,
S: AsRef<OsStr>,
{
    let mut cmd = Command::new(command);
    cmd.args(args);

    // println!("{:?}", cmd);

    let cmd= cmd
        .stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
    if let Ok(output) = cmd.output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.trim().to_string();
    }
    return String::new();
}

pub fn exec_stdout<I, S> (command: &str, args: I)
where
I: IntoIterator<Item = S>,
S: AsRef<OsStr>,
{
    let mut cmd = Command::new(command);
    cmd.args(args);

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
