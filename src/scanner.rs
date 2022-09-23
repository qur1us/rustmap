use std::net::IpAddr;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::process::{Command, Stdio};
use std::fs::File;


pub enum Options {

}

pub struct Target {
    ip_addr: IpAddr,
    name: String,
    open_ports: Vec<u16>
}

pub struct Scanner {
    target: Target,
    speed: u8,
    options: Vec<Options>
}

impl Target {
    pub fn new(ip_addr: IpAddr, name: String, open_ports: Vec<u16>) -> Target {
        Target {
            ip_addr,
            name,
            open_ports
        }
    }

    pub fn ip(&self) -> &IpAddr {
        &self.ip_addr
    }
}

impl Scanner {
    pub fn new(target: Target, speed: u8, options: Vec<Options>) -> Scanner {
        Scanner {
            target,
            speed,
            options
        }
    }

    pub fn get_ports(&self) -> Vec<u16> {
    
        let outfile: String = format!("./nmap/{}{}", self.target.name, "-all-ports");
        let path_str: String = format!("{}{}", outfile, ".nmap");
        let mut open_ports: Vec<u16> = Vec::new();
        
        // Run nmap SYN scan on all ports
        match Command::new("sudo")
                        .stdout(Stdio::null()) 
                        .arg("nmap")
                        .arg("-sS")
                        .arg("-p-")
                        .arg("-oA")
                        .arg(&outfile) 
                        .arg(self.target.ip_addr.to_string())
                        .spawn() {
            Ok(child) => {
                
                // Wait untill nmap finishes
                _ = child.wait_with_output();

                // Read the nmap output
                // Try to get handle on the file
                match File::open(Path::new(&path_str)) {
                   
                    Ok(file) => {
                        
                        let lines = BufReader::new(file).lines();

                        for line in lines {
                            
                            // Try to read the line
                            match line {

                                Ok(ln) => {
                                    
                                    // Look for open ports
                                    if ln.contains("open") {
                                        
                                        let splitted: Vec<&str> = ln.split("/").collect();

                                        match splitted[0].parse() {

                                            Ok(port) => open_ports.push(port),
                                            Err(_) => panic!("Could not parse &str to u16.")
                                        }
                                    }
                                }
                                Err(_) => panic!("Could not read line.")
                            }                     
                        }
                    }
                    Err(_) => panic!("Failed to open the outfile.")
                }
            }
            Err(_) => println!("Nmap port scan failed.")
        }

        open_ports
    }

    fn run_script_scan(rhost: IpAddr, target_name: String) {

        println!("Running script scan.");
    
        Command::new("mkdir")
                    .arg("nmap")
                    .spawn()
                    .expect("Failed");
        
        match Command::new("nmap")
                        .arg("-sC")
                        .arg("-sV")
                        .arg("-oA")
                        .arg(format!("nmap/{}", target_name)) 
                        .arg(rhost.to_string())
                        .arg("-p")
                        .arg("22,25,631,3306")
                        .spawn() {
            Ok(child) => {
                _ = child.wait_with_output(); // let output: Result<Output> if I want to do sth with it later
                println!("Nmap script scan finished.");
            }
            Err(_) => {
                println!("Nmap script scan failed.");
            }
        }
    }

    // async fn check_port(rhost: IpAddr, rport: u16, timeout: u64) {

    //     let delay: Duration = Duration::from_secs(timeout);
    //     let socket_address: SocketAddr = SocketAddr::new(rhost.clone(), rport);

    
    //     match tokio::time::timeout(delay, TcpStream::connect(&socket_address)).await {
    //         Ok(Ok(_)) => {
    //             println!("Port {} open.", rport);
    //             //attempts.push(try_conn);
    //         }
    //         _ => {
    //             //print!("Port {} closed.", rport);
    //         }
    //     }
    // }

    // pub async fn tcp_connect_scan(rhost: IpAddr) {

    //     stream::iter(get_ports()).for_each_concurrent(1000, |port| async move {
    //             check_port(rhost, port, 10).await;
    //     }).await;
    // }
}
