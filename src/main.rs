extern crate create2crunch;

use std::env;
use std::process;

use create2crunch::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if config.gpu_device == 255 {
        if let Err(e) = create2crunch::cpu(config) {
            eprintln!("CPU application error: {}", e);
            process::exit(1);
        }
    } else {
        // if let Err(e) = create2crunch::gpu(config) {
        //     eprintln!("GPU application error: {}", e);
        //     process::exit(1);
        // }
    }
}
