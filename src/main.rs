use colored::*;
use reqwest::blocking::Client;
use reqwest::header::LOCATION;
use reqwest::{Error, blocking::Response};
use std::{env, process::exit};

#[derive(Default)]
struct CheckResult {
    target: String,
    status: String,
    status_code: Option<u16>,
    time_taken: u128,
    redirected: bool,
    location: Option<String>,
    error: Option<String>,
}

impl CheckResult {
    fn print_result(&self) {
        println!("{}", format!("{:<12} {}", "Target:", self.target).yellow());
        println!(
            "{}",
            format!("{:<12} {}ms", "Time Taken:", self.time_taken).cyan()
        );
        println!("{}", format!("{:<12} {}", "Status:", self.status).blue());

        if let Some(err) = &self.error {
            println!("{}", format!("{:<12} {err}", "Error:").red());
            return;
        }

        if let Some(status_code) = self.status_code {
            println!(
                "{}",
                format!("{:<12} {status_code}", "Status Code:").white()
            );
        }

        if self.redirected {
            println!("{}", format!("{:<12} {}", "Redirected:", "Yes").green());

            if let Some(location) = &self.location {
                println!("{}", format!("{:<12} {location}", "Location:").purple());
            }
        } else {
            println!("{}", format!("{:<12} {}", "Redirected:", "No").green());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("{}", "Invalid number of arguments!".red());
        println!("{}", "Try: pulse help".on_white());
        exit(1);
    }

    if &args[1] == "help" {
        println!("{}", "Try: pulse <website>".bright_white());
        println!("{}", "\nUsage:".yellow());
        println!("{}", "  pulse www.google.com".bright_black());
        println!("{}", "  pulse https://github.com".bright_black());
        exit(0);
    }

    let url: String = normalize_url(&args[1]);
    let result: CheckResult = check_website(&url);
    result.print_result();
}

fn normalize_url(url: &str) -> String {
    if url.starts_with("https://") || url.starts_with("http://") {
        url.to_string()
    } else {
        format!("https://{url}")
    }
}

fn create_custom_client() -> Client {
    let client: Client = Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to make http custom client");

    client
}

fn get_location(resp: Response) -> String {
    let location: String = resp
        .headers()
        .get(LOCATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
        .to_string();

    location
}

fn check_website(url: &str) -> CheckResult {
    let client: Client = create_custom_client();
    let start: std::time::Instant = std::time::Instant::now();
    let response: Result<Response, Error> = client.get(url).send();
    let time_taken: u128 = start.elapsed().as_millis();

    match response {
        Err(error) => CheckResult {
            target: url.to_string(),
            status: String::from("Down"),
            time_taken,
            error: Some(error.to_string()),
            ..Default::default()
        },
        Ok(resp) => {
            let status: reqwest::StatusCode = resp.status();

            CheckResult {
                target: url.to_string(),
                status: status.to_string(),
                time_taken,
                status_code: Some(status.as_u16()),
                location: Some(get_location(resp)),
                redirected: status.is_redirection(),
                ..Default::default()
            }
        }
    }
}
