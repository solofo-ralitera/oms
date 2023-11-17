use oms::app::App;
use std::process;


fn main() {
    App::from_env()
        .unwrap_or_else(|err| {
            eprintln!("\nInvalid command: {err}");
            process::exit(1);
        })
        .run()
        .unwrap_or_else(|err| {
            eprintln!("\n{err}\n");
            process::exit(1);
        });
}
