use std::env;
use std::net::{IpAddr,Ipv4Addr};
use std::str::FromStr;
use scanner::{Target, Scanner};

mod scanner;
mod tools;

fn main() {

    let (ipv4_addr_str, target_name): (String, String) = tools::parse_args(env::args().collect());

    tools::print_banner();

    match Ipv4Addr::from_str(&ipv4_addr_str) {
        
        Ok(ipv4_addr) => {

            let rhost: IpAddr = IpAddr::V4(ipv4_addr);
            let target: Target = Target::new(rhost, target_name, String::new());
            let mut scanner: Scanner = Scanner::new(target, 3, Vec::new());
            
            println!("Target:\n{}\n", scanner.target_info());

            scanner.run();
        }

        Err(_) => println!("The ip address is in a wrong format!")
    }
}
