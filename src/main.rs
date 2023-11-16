use oms::app;
use std::process;

/**
* cargo run -- read /home/solofo/Videos/text.txt
*/
fn main() {
    app::parse_from_env()
        .unwrap_or_else(|err| {
            println!("{err}");
            process::exit(1);
        })
        .run()
        .unwrap_or_else(|err| {
            println!("{err}");
            process::exit(1);
        });
}
