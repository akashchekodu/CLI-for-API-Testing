// main.rs

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
    match Request::new(&args.method, &args.url, args.body.as_deref(), args.headers.as_deref(), args.url_params.as_deref()) {
        Ok(request) => {
            println!("Parsed Request: {:?}", request);

            let client = Client::new();

            // Prepare the request builder
            let mut request_builder = match request.method.to_uppercase().as_str() {
                "POST" => client.post(&request.url),
                "GET" => client.get(&request.url),
                "PUT" => client.put(&request.url),
                "DELETE" => client.delete(&request.url),
                "PATCH" => client.patch(&request.url),
                _ => {
                    eprintln!("Method {} is not supported for API calls.", request.method);
                    return Ok(());
                }
            };

            // Add headers to the request builder
            for (key, value) in request.headers.iter() {
                request_builder = request_builder.header(key, value);
            }

            // Send the request with JSON body if applicable
            let res = if request.method.eq_ignore_ascii_case("GET") {
                request_builder.send().await?
            } else {
                request_builder.json(&request.body).send().await?
            };

            // Handle the response
            handle_response(res).await?;

            // Example of accessing specific fields in the parameters
            if let Some(field1) = request.body.get("field1") {
                println!("Field1: {:?}", field1);
            }
            if let Some(field2) = request.body.get("field2") {
                println!("Field2: {:?}", field2);
            }
        }
        Err(e) => eprintln!("Failed to parse JSON: {}", e),
    }

    Ok(())
}

async fn handle_response(res: reqwest::Response) -> Result<(), Box<dyn Error>> {
    if res.status().is_success() {
        let response_body: serde_json::Value = res.json().await?;
        println!("Response Body: {:?}", response_body);
    } else {
        eprintln!("Failed to make API call: {:?}", res.status());
    }
    Ok(())
}
