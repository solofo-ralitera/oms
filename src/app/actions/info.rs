use std::io;

pub fn help_command() -> &'static str {
    "\nSee 'oms info'\n"
}

pub fn run() -> Result<(), io::Error> {
    println!("");
    println!("Usage:  oms [OPTIONS] COMMAND");
    println!("");
    println!("Available commands:");
    println!("  info                    Display these informations");
    println!("  read [file_path]        Display the content of the file");
    Ok(())
}
