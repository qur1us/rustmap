use std::env;
use std::net::{IpAddr,Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use async_std::net::{TcpStream};

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

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    print_banner();

    match Ipv4Addr::from_str(&args[1]) {
        Ok(ipv4_addr) => {
            let rhost: IpAddr = IpAddr::V4(ipv4_addr);

            tcp_connect_scan(rhost).await;
        }
        Err(_) => {
            println!("The ip address is in a wrong format!");
        }
    }
}
