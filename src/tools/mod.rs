use std::{process::exit};

pub fn print_banner() -> &'static str {

    let banner: &'static str = "
_________________________________

rustmap v1.2 by qurius
_________________________________
";

    return banner;
}

pub fn print_help() -> &'static str {

    let help_msg: &'static str = "

-h                      This help message.
--speed <number 1-5>    Speed of nmap port scan.
--warp                  Packet minimal rate = 10000 pkt/s.
--verbose               Show ports as nmap finds them in command line.
--no-ping               Tells nmap not to use ping to discover host.
";
    return help_msg;
}

pub fn parse_args(args: Vec<String>) -> Vec<String> {
    // TODO handle number of cmdline argumets
    let min_args: usize = 3;
    let mut nmap_args: Vec<String> = Vec::new();

    if args.len() < min_args {
        println!("{}", print_banner());
        println!("{}", print_help());
        exit(1);
    }

    nmap_args.push(args[1].clone());
    nmap_args.push(args[2].clone());
    
    for i in 2..args.len() {

        if args[i] == "-h" {
            println!("{}", print_help());
            exit(1);
        }

        if args[i] == "--speed" {
            nmap_args.push(format!("-T{}", args[i + 1]));
        }

        if args[i] == "--warp" {
            nmap_args.push(format!("--min-rate"));
        }

        if args[i] == "--verbose" {
            nmap_args.push(format!("-v"));
        }

        if args[i] == "--no-ping" {
            nmap_args.push(format!("-Pn"));
        }
    }

    return nmap_args;
}
