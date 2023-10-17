use std::net::{IpAddr};
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::process::{Command, Stdio};
use std::fs::{File, self};
// use std::time::Duration;
// use async_std::net::TcpStream;
// use futures::{stream, StreamExt};


pub struct Target {
    ip_addr: IpAddr,
    name: String,
    open_ports: String
}

pub struct Scanner {
    target: Target,
    nmap_flags: Vec<String>,
}

impl Target {
    pub fn new(ip_addr: IpAddr, name: String, open_ports: String) -> Target {
        Target {
            ip_addr,
            name,
            open_ports
        }
    }

    pub fn ip(&self) -> &IpAddr {
        &self.ip_addr
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn open_ports(&self) -> &str {
        &self.open_ports
    }
}

impl Scanner {
    pub fn new(target: Target, nmap_flags: Vec<String>) -> Scanner {
        Scanner {
            target,
            nmap_flags
        }
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn target_info(&self) -> String {
        format!("{} ({})", self.target().ip().to_string(), self.target().name())
    }

    pub fn vec_as_str(&self, open_ports_vec: &Vec<u16>) -> String {

        // Create Vec<String> from Vec<u16>
        let ports: Vec<String> = open_ports_vec
                        .into_iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>();
    
        ports.join(",")
    }

    fn parse_ports(&self, path_str: String) -> String {

        let mut open_ports_vec: Vec<u16> = Vec::new();

        // Read the nmap output
        // Try to get handle on the file
        match File::open(Path::new(&path_str)) {
            
            Ok(file) => {

                println!("\n\n[*] Parsing the namp full-port scan output.");
                
                let lines = BufReader::new(&file).lines();

                for line in lines {
                    
                    // Try to read the line
                    match line {

                        Ok(ln) => {
                            
                            // Look for open ports
                            if ln.contains("open") {
                                
                                let splitted: Vec<&str> = ln.split("/").collect();

                                match splitted[0].parse() {

                                    Ok(port) => open_ports_vec.push(port),
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

        self.vec_as_str(&open_ports_vec)
    }
    
    fn get_ports(&self) -> String {
    
        let outfile: String = format!("./nmap/{}{}", &self.target().name(), "-all-ports");
        let path_str: String = format!("{}{}", &outfile, ".nmap");

        let mut speed_flag = "";
        let mut min_rate_flag = "";
        let mut min_rate_flag_value = "";
        let mut verbose_flag = "";
        let mut ping_flag = "";

        for i in 0..self.nmap_flags.len() {

            if self.nmap_flags[i].contains("-T") {
                speed_flag = &self.nmap_flags[i];
            }
    
            if self.nmap_flags[i] == "--min-rate" {
                min_rate_flag = "--min-rate";
                min_rate_flag_value = "10000";
            }
    
            if self.nmap_flags[i] == "-v" {
                verbose_flag = "-v";
            }
    
            if self.nmap_flags[i] == "-Pn" {
                ping_flag = "-Pn"
            }
        }

        println!("\n");

        match fs::create_dir_all("./nmap") {
            Ok(_) => println!("[+] Created nmap directory"),
            Err(_) => {}
        }

        println!("[*] Running full-port scan.\n\n");
        
        // Run nmap SYN scan on all ports
        match Command::new("sudo")
                        .stderr(Stdio::null())
                        .arg("nmap")
                        .arg("-sS")
                        .arg("-p-")
                        .arg("-oA")
                        .arg(&outfile) 
                        .arg(&self.target().ip().to_string())
                        .arg(&speed_flag)
                        .arg(&min_rate_flag)
                        .arg(&min_rate_flag_value)
                        .arg(&verbose_flag)
                        .arg(&ping_flag)
                        .spawn() {
            Ok(child) => {
                
                // Wait untill nmap finishes
                _ = child.wait_with_output();

                self.parse_ports(path_str) 
            }
            Err(_) => panic!("Nmap port scan failed.")
        }
    }

    fn run_script_scan(&self) {

        println!("[*] Running script scan on ports: {}.\n", self.target().open_ports());
        
        match Command::new("nmap")
                        .stdout(Stdio::null()) 
                        .arg("-sC")
                        .arg("-sV")
                        .arg("-oA")
                        .arg("-Pn")
                        .arg(format!("nmap/{}", self.target().name())) 
                        .arg(self.target().ip().to_string())
                        .arg("-p")
                        .arg(self.target().open_ports())
                        .spawn() {
            Ok(child) => {
                _ = child.wait_with_output(); // let output: Result<Output> if I want to do sth with it later
            }
            Err(_) => {
                println!("[x] Nmap script scan failed.");
            }
        }

        println!("Files:");

        match fs::read_dir("./nmap") {
            Ok(dir_list) => {
                for f in dir_list {
                    println!("{}", f.unwrap().path().display());
                }
            }
            Err(_) => {}
        }
    }

    pub fn run(&mut self) {

        self.target.open_ports = self.get_ports();
        self.run_script_scan();

        println!("\n[+] Scanning of the target machine ({}) finished.\n", self.target().ip());
    }
}
//     async fn check_port(&self, rhost: IpAddr, rport: u16, timeout: u64) {

//         let delay: Duration = Duration::from_secs(timeout);
//         let socket_address: SocketAddr = SocketAddr::new(rhost.clone(), rport);


//         match tokio::time::timeout(delay, TcpStream::connect(&socket_address)).await {
//             Ok(Ok(_)) => println!("Port {} open.", rport),
//             _ => {}
//         }
//     }

//     pub async fn tcp_connect_scan(&self, rhost: IpAddr) {

//         stream::iter(1..65535).for_each_concurrent(1000, |port| async move {
//                 self.check_port(rhost, port, 10).await;
//         }).await;
//     }
// }
