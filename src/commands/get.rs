use std::io::{self, Write};

use clap::{arg, command, Arg, ArgMatches, Command};
use clap::builder::Str;
use color_print::cprintln;

use crate::monitor::get::get_brightness_id;
use crate::monitor::list::{list_monitors, Config};
use crate::monitor::monitor::Monitor;


pub fn get_command() -> Command {
    let vr = arg!( -q --quiet "Don't show describing text");
    let id = arg!( <id> "The id of the monitor to get the brigthness");
    
    command!("get")
        .aliases(vec!["g"])
        .about("List all displays")
        .arg(id)
        .arg(vr)
}

pub fn get_mannager(args: &ArgMatches) {
    let qt = *args.get_one::<bool>("quiet").unwrap();
    let id = args.get_one::<String>("id").unwrap();
    
    match get_brightness_id(id.clone()) {
        Some(br) => match br {
            u16::MAX => {
                if !qt { cprintln!("<r>* Error:</> The monitor with {} does not support DDC/CI", id)}
            },
            br => {
                if qt {
                    let _ = io::stdout().write(br.to_string().as_bytes());
                } else {
                    let _ = io::stdout().write(
                        format!("Brightness: {}\n", br).as_bytes()
                    );
                }
                
                let _ = io::stdout().flush();
            }
        },
        None => {
            if !qt { cprintln!("<y>* No monitor found with id:</> <m,i>'{}'</>", id)}
        },
    }
    
}