use create2crunch::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed parsing arguments: {err}");
        process::exit(1);
    });


    if let Err(e) = create2crunch::gpu(config) {
        eprintln!("GPU application error: {e}");
        process::exit(1);
    }
}
