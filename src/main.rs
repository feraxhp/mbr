mod commands;
mod monitor;
// mod utils;

use crate::commands::list::{list_command, list_mannager};

use clap::{arg, command, crate_version, Arg, Command};
use color_print::cprintln;
use commands::get::{get_command, get_mannager};
use commands::set::{set_command, set_mannager};
use std::io;
use std::io::Write;
use std::process::exit;

fn main() {
    let matches = command!()
            .name("mbr")
            .about("A simple CLI to manage monitores with ddc-ci tecnology")
            .arg(arg!( -v --"number" "Prints the version number to the standard output").exclusive(true))
            .subcommand(list_command())
            .subcommand(get_command())
            .subcommand(set_command())
            .get_matches();
    
    match matches.clone().args_present() {
        true => {
            if *matches.get_one::<bool>("number").unwrap_or(&false) {
                let version = crate_version!();
                let _ = io::stdout().write(version.as_bytes());
                println!();
                exit(0);
            }
        },
        _ => {}
    }
    
    match matches.subcommand() {
        Some(sub) => match sub {
            ("list", list) => list_mannager(list),
            ("get", get) => get_mannager(get),
            ("set", set) => set_mannager(set),
            _ => {}
        },
        _ => {
            cprintln!("<y>* No command was provided try using <g,i>'--help'</>");
        }
    }
    
    // Enumerar monitores
    // for mut display in Display::enumerate() {
    //     // Leer brillo
    //     if let Ok(val) = display.handle.get_vcp_feature(0x10) {
    //         println!("Brillo actual {:#?}:{}", display.info.id, val.value());
    //     }
    //     // Poner brillo al 50%
    //     let _ = display.handle.set_vcp_feature(0x10, 50);
    // }
}