use std::{fs, thread, time::Duration};

use clap::{App, Arg};
use daemonize::Daemonize;
use notify_rust::Notification;

extern crate daemonize;
extern crate notify_rust;

fn main() {
    let matches = App::new("Google Calendar Notification Daemon")
        .version("0.0.1")
        .author("Atri Hegde")
        .about("A Google calendar notification daemon")
        .arg(
            Arg::with_name("action")
                .help("Action to perform: start, stop or status")
                .required(true)
                .index(1),
        )
        .get_matches();

    match matches.value_of("action").unwrap() {
        "start" => start_daemon(),
        "stop" => stop_daemon(),
        "status" => check_status(),
        _ => println!("Invalid action. Use 'start', 'stop' or 'status'."),
    }
}

fn start_daemon() {
    let daemonize = Daemonize::new()
        .pid_file("/tmp/gcnd.pid")
        .chown_pid_file(true)
        .working_directory("/tmp")
        .privileged_action(|| "Daemonization successful");

    match daemonize.start() {
        Ok(_) => {
            println!("Google Calendar Notification Daemon(gcnd) started.");
            daemon_action();
        }
        Err(e) => eprint!("Error starting daemon: {}", e),
    }
}

fn stop_daemon() {
    if let Ok(pid_str) = fs::read_to_string("/tmp/gcnd.pid") {
        if let Ok(pid) = pid_str.trim().parse::<i32>() {
            println!("Stopping the daemon...");
            if let Err(err) = std::process::Command::new("kill")
                .arg("-TERM")
                .arg(pid.to_string())
                .status()
            {
                eprintln!("Error stopping daemon: {}", err);
            }
        } else {
            println!("Invalid PID in the PID file!");
        }
    } else {
        println!("PID file not found. Daemon might not be running.");
    }
}

fn check_status() {
    if let Ok(pid_str) = fs::read_to_string("/tmp/gcnd.pid") {
        println!("Daemon running. PID: {}", pid_str)
    } else {
        println!("Daemon is not running.")
    }
}

fn daemon_action() {
    loop {
        let _ = Notification::new()
            .summary("Event title")
            .body("Event time + location")
            .show();

        thread::sleep(Duration::from_secs(60));
    }
}
