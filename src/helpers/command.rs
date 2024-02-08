use std::{ffi::OsStr, io::{self, BufRead, BufReader}, process::{Command, Stdio}};

type Result<T> = std::result::Result<T, std::io::Error>;

pub fn exec_result<I, S> (command: &str, args: I) -> Result<String>
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

    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            if stdout.is_empty() && !stderr.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted, 
                    stderr
                ));
            } else {
                return Ok(stdout.trim().to_string());
            }
        },
        Err(err) => {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted, 
                format!("{}", err.to_string())
            ));
        }
    }
}

pub fn exec<I, S> (command: &str, args: I) -> String
where
I: IntoIterator<Item = S>,
S: AsRef<OsStr>,
{
    return match exec_result(command, args) {
        Err(_) => String::new(),
        Ok(o) => o,
    };
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
