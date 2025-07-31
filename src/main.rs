use std::env;
use std::fs;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use std::path::Path;

const PID_FILE: &str = "torify.pid";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: torify [start|stop|<cmd> ...]");
        std::process::exit(1);
    }

    match args[0].as_str() {
        "start" => {
            if Path::new(PID_FILE).exists() {
                println!("Tor is already running.");
                return;
            }

            let child = Command::new("tor.exe")
                .args(["--SocksPort", "9050"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to start tor.exe");

            let pid = child.id();
            fs::write(PID_FILE, pid.to_string()).expect("Failed to write PID file");
            println!("Tor started (PID {}). Waiting 5s to bootstrap...", pid);
            sleep(Duration::from_secs(5));
        }
        "stop" => {
            if let Ok(pid_str) = fs::read_to_string(PID_FILE) {
                if let Ok(pid) = pid_str.trim().parse::<u32>() {
                    let _ = Command::new("taskkill")
                        .args(["/PID", &pid.to_string(), "/F"])
                        .output();
                }
                let _ = fs::remove_file(PID_FILE);
                println!("Tor stopped.");
            } else {
                println!("Tor is not running.");
            }
        }
        _ => {
            let status = Command::new(&args[0])
                .args(&args[1..])
                .env("HTTP_PROXY", "socks5h://127.0.0.1:9050")
                .env("HTTPS_PROXY", "socks5h://127.0.0.1:9050")
                .env("ALL_PROXY", "socks5h://127.0.0.1:9050")
                .status()
                .expect("Failed to run command");

            std::process::exit(status.code().unwrap_or(1));
        }
    }
}

