use std::env;
use std::net::{IpAddr,Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use async_std::net::{TcpStream};
use std::process::{Command, Output};
use std::io::Result;

fn print_banner() {
    let banner: &'static str = "
_________________________________

rustmap v0.1 by qurius
_________________________________
";

    println!("{}", banner);
}

fn get_ports() -> Vec<u16> {
    return (1..=65535).collect();
}

async fn check_port(rhost: IpAddr, rport: u16, timeout: u64) {
    let delay: Duration = Duration::from_secs(timeout);
    let socket_address: SocketAddr = SocketAddr::new(rhost.clone(), rport);
   
    match tokio::time::timeout(delay, TcpStream::connect(&socket_address)).await {
        Ok(Ok(_)) => {
            println!("Port {} open.", rport);
        }
        _ => {}
    }
}

async fn tcp_connect_scan(rhost: IpAddr) {
    for port in get_ports().into_iter() {
        check_port(rhost, port, 10).await;
    }
}

fn run_nmap(rhost: IpAddr, machine_name: String) {
    println!("Running script scan.");

    Command::new("mkdir")
                .arg("nmap")
                .spawn()
                .expect("Failed");
    
    match Command::new("nmap")
                    .arg("-sC")
                    .arg("-sV")
                    .arg("-oA")
                    .arg(format!("nmap/{}", machine_name)) 
                    .arg(rhost.to_string())
                    .arg("-p")
                    .arg("22,25,631,3306")
                    .spawn() {
        Ok(child) => {
            let output: Result<Output> = child.wait_with_output();
            println!("Nmap script scan finished.");
        }
        Err(_) => {
            println!("Nmap script scan failed.");
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let machine_name: String = args[2].clone();

    print_banner();

    match Ipv4Addr::from_str(&args[1]) {
        Ok(ipv4_addr) => {
            let rhost: IpAddr = IpAddr::V4(ipv4_addr);

            tcp_connect_scan(rhost).await;
            run_nmap(rhost, machine_name);
        }
        Err(_) => {
            println!("The ip address is in a wrong format!");
        }
    }
}
