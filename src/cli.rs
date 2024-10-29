// src/cli.rs
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Request Parser")]
#[command(about = "Parses an HTTP method, URL, and JSON parameters from the command line")]
pub struct Cli {
    /// HTTP method (e.g., GET or POST)
    pub method: String,

    /// The URL for the request
    pub url: String,

    /// JSON-like string with request parameters
    pub json: Option<String>,
}
