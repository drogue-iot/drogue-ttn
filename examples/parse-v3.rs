use std::{env, fs};

use drogue_ttn::v3 as ttn;
use serde_json as json;

fn main() -> Result<(), String> {
    // Args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(format!("Usage: {} <jsonfile>", args[0]));
    }

    // Read from file
    let buffer = fs::read(&args[1]).unwrap();

    let ttn_msg = match json::from_slice::<ttn::Message>(&buffer) {
        Ok(msg) => msg,
        Err(e) => {
            return Err(format!("Uplink message could not be parsed: {}", e));
        }
    };
    println!("Parsed: {:#?}", ttn_msg);
    Ok(())
}
