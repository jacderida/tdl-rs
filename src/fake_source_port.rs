use color_eyre::{Report, Result};
use std::collections::HashMap;

/// This binary is intended to be used in integration tests to check that TDL is calling the source
/// ports using the correct arguments. A real source port can't be used because it would launch the
/// GUI and then need manual intervention to be stopped.
/// Instead, the tests can use this fake source port that will simply just print out the
/// arguments it was passed. The integration tests can then assert that the correct arguments were
/// passed to the process it was calling, and that's what I actually care about anyway.
fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let mut game_args: HashMap<String, String> = HashMap::new();
    let flags = vec!["-fullscreen", "-music", "-nofullscreen", "-nomusic"];
    println!("Running fake source port");

    // This is probably the most inelegant implementation ever devised for parsing arguments, but
    // it seems to work and it's just a dummy thing anyway.
    let mut warp_arg = String::new();
    let mut prev = "".to_string();
    for arg in std::env::args().skip(1) {
        if prev == "-warp" {
            println!("in here");
            warp_arg.push_str(&format!("{} ", arg.to_owned()));
            if warp_arg.len() >= 3 {
                game_args.insert(prev.to_owned(), warp_arg.to_owned().trim().to_string());
            }
        } else {
            if arg.starts_with("-") {
                if flags.iter().any(|x| **x == arg) {
                    game_args.insert(arg.to_owned(), "true".to_string());
                } else {
                    prev = arg.to_owned();
                }
            } else {
                game_args.insert(prev.to_owned(), arg.to_owned());
            }
        }
    }
    for (arg, value) in game_args.iter() {
        if value == "true" {
            println!("Game called with {}", arg);
        } else {
            println!("Game called with {}: {}", arg, value);
        }
    }
    Ok(())
}
