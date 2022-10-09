use std::{env};
use std::net::{IpAddr,Ipv4Addr};
use std::str::FromStr;
use scanner::{Target, Scanner};

mod scanner;
mod tools;

fn main() {

    let mut nmap_flags: Vec<String> = tools::parse_args(env::args().collect());

    println!("{}", tools::print_banner());

    let ipv4_addr_str = nmap_flags[0].clone();
    let target_name = nmap_flags[1].clone();
    
    nmap_flags.drain(0..2);

    match Ipv4Addr::from_str(&ipv4_addr_str) {
        
        Ok(ipv4_addr) => {

            let rhost: IpAddr = IpAddr::V4(ipv4_addr);
            let target: Target = Target::new(rhost, target_name, String::new());
            let mut scanner: Scanner = Scanner::new(target, nmap_flags);
            
            println!("Target:\n{}\n", scanner.target_info());

            scanner.run();
        }

        Err(_) => println!("The ip address is in a wrong format!")
    }
}
