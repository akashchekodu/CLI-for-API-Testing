// src/cli.rs
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Request Parser")]
#[command(about = "Parses an HTTP method, URL, and JSON parameters from the command line")]
pub struct Cli {
    /// HTTP method (e.g., GET or POST)
    #[arg(long,short='m')]
    pub method: String,
    
    /// The URL for the request
    #[arg(long,short='u')]
    pub url: String,
    
    /// JSON-like string with request parameters
    #[arg(long,short='b')]
    pub body: Option<String>,
    
    #[arg(long,short='a')]
    pub headers: Option<String>,

}
