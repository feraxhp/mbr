use std::io::{self, Write};
use std::process::exit;

use clap::{arg, command, Arg, ArgMatches, Command};
use clap::builder::Str;
use color_print::cprintln;

use crate::monitor::get::get_brightness_id;
use crate::monitor::list::{list_monitors, Config};
use crate::monitor::monitor::Monitor;
use crate::monitor::set::set_brigthness;


pub fn set_command() -> Command {
    let vr = arg!( -q --quiet "Don't show describing text");
    let id = arg!( [id] "The id of the monitor to get the brigthness");
    let lv = arg!( <level> "The brigthness level to be set");
    
    command!("set")
        .aliases(vec!["s"])
        .about("Set brigthness to the given display [if not present applies to all]]")
        .arg(lv)
        .arg(id)
        .arg(vr)
}

pub fn set_mannager(args: &ArgMatches) {
    let qt = *args.get_one::<bool>("quiet").unwrap();
    let lv: &String = args.get_one::<String>("level").unwrap();
    let lv: u16 = match lv.parse::<u16>() {
        Ok(lv) => lv,
        Err(err) => {
            if !qt { 
                match err {
                    ref ParseIntError => {
                        cprintln!("<r>* Error:</> <m,i>'{}'</> is not a number", lv)
                    }
                    err => cprintln!("<r>* Error:</> {:?}", err) 
                }
                
            }
            exit(1)
        },
    };
    let id = args.get_one::<String>("id").map(|id| id.to_string());
    
    match set_brigthness(id, lv) {
        Ok(level) => {
            if !qt {
                cprintln!("<g>* Brightness set to</> <m,i>'{}'</>", level)
            } else { println!("{}", level) }
        },
        Err(e) => { match e {
            Ok(vec) => {
                if !qt {
                    cprintln!("<y>* Some displays may not support DDC/CI</>");
                    vec.iter().for_each(|e| {
                        cprintln!("  <r>* Error:</> {}", e)
                    });
                }
                
                exit(2)
            },
            Err(e) => {
                if !qt { cprintln!("<r>* Error:</> {}", e) }
                exit(1)
            },
        }},
    }
}