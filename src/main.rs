use oms::app::App;
use std::process;

/// RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release
/// upx --best --lzma target/x86_64-unknown-linux-gnu/release/oms
fn main() {
    App::from_env()
        .unwrap_or_else(|err| {
            eprintln!("\n{err}");
            process::exit(1);
        })
        .run()
        .unwrap_or_else(|err| {
            eprintln!("\n{err}\n");
            process::exit(1);
        });
}
