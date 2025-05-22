use clap::{arg, command, ArgMatches, Command};

use crate::monitor::list::{list_monitors, Config};
use crate::monitor::monitor::Monitor;


pub fn list_command() -> Command {
    
    let br = arg!( -b --brightness "Show the brightness of the display");
    let vr = arg!( -v --verbose "Show like a table");
    
    command!("list")
        .aliases(vec!["ls"])
        .about("List all displays")
        .arg(br)
        .arg(vr)
}

pub fn list_mannager(args: &ArgMatches) {
    let mut options: Vec<Config> = Vec::new();
    
    let br = *args.get_one::<bool>("brightness").unwrap();
    let qt = *args.get_one::<bool>("verbose").unwrap();
    
    if br { options.push(Config::BRIGHTNESS); }
    
    let monitors = list_monitors(options);
    
    Monitor::show(monitors, qt);
}