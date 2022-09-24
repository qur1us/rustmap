use std::process::exit;

pub fn print_banner() {

    let banner: &'static str = "
_________________________________

rustmap v1.0 by qurius
_________________________________
";

    println!("{}", banner);
}

pub fn print_help() -> String {
    // TODO help message

    return String::from("help me!");
}

pub fn parse_args(args: Vec<String>) -> (String, String) {
    // TODO handle number of cmdline argumets
    
    if args.len() < 3 {
        println!("{}", print_help());
        exit(1);
    }
    
    return (args[1].clone(), args[2].clone());
}
