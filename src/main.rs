mod cli;
mod request;

use cli::Cli;
use request::Request;
use clap::Parser;
use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // Attempt to create a Request object from the parsed CLI arguments
    match Request::new(&args.method, &args.url, args.json.as_deref()) {
        Ok(request) => {
            println!("Parsed Request: {:?}", request);

            let client = Client::new();

            match request.method.to_uppercase().as_str() {
                "POST" => {
                    // Making the POST request with JSON body
                    let res = client.post(&request.url)
                        .json(&request.params) // Send the parameters as JSON
                        .send()
                        .await?;

                    // Check the response status
                    if res.status().is_success() {
                        let response_body: serde_json::Value = res.json().await?;
                        println!("Response Body: {:?}", response_body);
                    } else {
                        eprintln!("Failed to make API call: {:?}", res.status());
                    }
                }
                "GET" => {
                    // Example for handling GET requests
                    let res = client.get(&request.url).send().await?;
                    if res.status().is_success() {
                        let response_body: serde_json::Value = res.json().await?;
                        println!("Response Body: {:?}", response_body);
                    } else {
                        eprintln!("Failed to make API call: {:?}", res.status());
                    }
                }
                _ => {
                    eprintln!("Method {} is not supported for API calls.", request.method);
                }
            }

            // Example of accessing specific fields in the parameters
            if let Some(field1) = request.params.get("field1") {
                println!("Field1: {:?}", field1);
            }
            if let Some(field2) = request.params.get("field2") {
                println!("Field2: {:?}", field2);
            }
        }
        Err(e) => eprintln!("Failed to parse JSON: {}", e),
    }

    Ok(())
}
