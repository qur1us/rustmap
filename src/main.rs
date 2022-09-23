use std::env;
use std::net::{IpAddr,Ipv4Addr};
use std::str::FromStr;
use std::process::exit;
use scanner::{Target, Scanner};

mod scanner;


fn print_banner() {
    let banner: &'static str = "
_________________________________

rustmap v0.1 by qurius
_________________________________
";

    println!("{}", banner);
}

fn print_help() -> String {
    // TODO help message

    return String::from("help me!");
}

fn parse_args(args: Vec<String>) -> (String, String) {
    // TODO handle number of cmdline argumets
    
    if args.len() < 3 {
        println!("{}", print_help());
        exit(1);
    }
    
    return (args[1].clone(), args[2].clone());
}

fn main() {

    let (ipv4_addr_str, target_name): (String, String) = parse_args(env::args().collect());

    print_banner();
    
    println!(
        "Current working directory: {}\n",
        env::current_dir()
        .unwrap()
        .display()
    );

    match Ipv4Addr::from_str(&ipv4_addr_str) {
        
        Ok(ipv4_addr) => {

            let rhost: IpAddr = IpAddr::V4(ipv4_addr);
            let target: Target = Target::new(rhost, target_name, Vec::new());
            let mut scanner: Scanner = Scanner::new(target, 3, Vec::new());
            
            let ports: Vec<u16> = scanner.get_ports();

            for port in ports {
                print!("{},", port);
            }
        }

        Err(_) => println!("The ip address is in a wrong format!")
    }
}
