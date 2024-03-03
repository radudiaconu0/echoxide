use chrono::Local;
use colored::*;

pub struct Log;

impl Log {
    pub fn info_title(message: &str) {
        println!("{}", message.bold().black().on_cyan());
    }

    pub fn success_title(message: String) {
        println!("{}", message.bold().black().on_green());
    }

    pub fn error_title(message: &str) {
        println!(
            "{}",
            Self::prefix_with_time(message).bold().black().on_red()
        );
    }

    pub fn warning_title(message: &str) {
        println!(
            "{}",
            Self::prefix_with_time(message).bold().black().on_yellow()
        );
    }

    pub fn cluster_title(message: &str) {
        println!(
            "{}",
            Self::prefix_with_time(message).bold().yellow().on_magenta()
        );
    }

    pub fn http_title(message: &str) {
        Self::info_title(&Self::prefix_with_time(message));
    }

    pub fn discover_title(message: &str) {
        println!(
            "{}",
            Self::prefix_with_time(message)
                .bold()
                .bright_black()
                .on_bright_cyan()
        );
    }

    // The websocket_title method uses the success_title as its basis.
    pub fn websocket_title(message: &str) {
        Self::success_title(Self::prefix_with_time(message));
    }

    pub fn webhook_sender_title(message: &str) {
        println!(
            "{}",
            Self::prefix_with_time(message).bold().white().on_black()
        );
    }

    pub fn info(message: String) {
        println!("{}", message.cyan());
    }

    pub fn success(message: &str) {
        println!("{}", message.green());
    }

    pub fn error(message: String) {
        println!("{}", message.red());
    }

    pub fn warning(message: &str) {
        println!("{}", message.yellow());
    }

    pub fn cluster(message: &str) {
        println!("{}", message.bold().magenta());
    }

    pub fn http(message: String) {
        Self::info(message);
    }

    pub fn discover(message: &str) {
        println!("{}", message.bold().bright_cyan());
    }

    pub fn websocket(message: &str) {
        Self::success(message);
    }

    pub fn webhook_sender(message: &str) {
        println!("{}", message.bold().white());
    }

    pub fn br() {
        println!();
    }
    fn prefix_with_time(message: &str) -> String {
        let now = Local::now();
        format!("[{}] {}", now.format("%Y-%m-%d %H:%M:%S"), message)
    }
}
