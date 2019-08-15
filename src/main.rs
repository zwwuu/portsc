use std::env;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::ops::RangeInclusive;
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;

use colored::Colorize;
use getopts::Options;

fn scan(tx: Sender<u16>, address: IpAddr, port: u16) {
    let socket = SocketAddr::new(address, port);
    match TcpStream::connect_timeout(&socket, Duration::new(1, 0)) {
        Ok(_) => {
            tx.send(port).unwrap();
            println!("{} is {}", port, "OPEN".green());
        }
        _ => {
            println!("{} is {}", port, "CLOSE".red());
        }
    };
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} IP PORT_RANGE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn init_options() -> Options {
    let mut opt = Options::new();
    opt.optflag("h", "help", "port scanner\nIP must be an valid IP address\nPORT_RANGE must be valid integers [0-65536], separate by ':' or '-'");
    opt.optflag("n", "no-color", "disable colored text");
    opt
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let opts = init_options();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") || matches.free.is_empty() || matches.free.len() < 2 {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("n") {
        colored::control::set_override(false);
    }

    let ip_address: IpAddr = matches.free[0].parse().unwrap();
    let v: Vec<&str> = matches.free[1].split(|c| c == ':' || c == '-').collect();
    let range: RangeInclusive<u16> = match v.len() {
        1 => RangeInclusive::new(v[0].parse().unwrap(), v[0].parse().unwrap()),
        _ => RangeInclusive::new(v[0].parse().unwrap(), v[1].parse().unwrap()),
    };

    let (tx, _rx) = channel();
    println!(
        "Checking port(s) on {} from {} to {}",
        ip_address,
        range.start(),
        range.end()
    );
    for i in range {
        let tx = tx.clone();
        scan(tx, ip_address, i);
    }
}
