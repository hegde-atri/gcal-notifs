mod args;

use args::{Action, GcndArgs};
use clap::Parser;
use daemonize::Daemonize;
use notify_rust::Notification;
use rodio::Sink;
use std::{fs, thread, time::Duration};

extern crate daemonize;
extern crate notify_rust;

const SOUND_FILE: &[u8] = include_bytes!("../ping.mp3");

fn main() {
    let args = GcndArgs::parse();

    match args.action {
        Action::Start => start_daemon(),
        Action::Stop => stop_daemon(),
        Action::Status => check_status(),
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
    // Setup audio sink
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    loop {
        if let Err(err) = play_sound(&sink) {
            eprintln!("Error playing notification sound: {}", err);
        }
        let _ = Notification::new()
            .summary("Event title")
            .body("Event time + location")
            .show();

        thread::sleep(Duration::from_secs(60));
    }
}

fn play_sound(sink: &Sink) -> Result<(), rodio::decoder::DecoderError> {
    let source = rodio::Decoder::new(std::io::Cursor::new(SOUND_FILE))?;
    sink.append(source);
    Ok(())
}
