//request.rs

use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub body: HashMap<String, Value>,
    pub headers: HashMap<String, String>, // Ensure this field exists
    pub url_params: HashMap<String, Value>, // New field for URL parameters
}

impl Request {
    // Parses custom JSON string into a HashMap
    pub fn parse_custom_json(input: &str) -> Result<HashMap<String, Value>, String> {
        let trimmed = input.trim_start_matches('{').trim_end_matches('}');
        let mut map = HashMap::new();

        for pair in trimmed.split(", ") {
            let parts: Vec<&str> = pair.split(": ").collect();
            if parts.len() != 2 {
                return Err(format!("Invalid key-value pair: {}", pair));
            }
            let key = parts[0].trim().trim_matches('"').to_string();
            let value_str = parts[1].trim().trim_matches('"');

            // Use serde_json's Value directly
            let value = if let Ok(int_val) = value_str.parse::<i32>() {
                Value::Number(int_val.into())
            } else {
                Value::String(value_str.to_string())
            };

            map.insert(key, value);
        }

        Ok(map)
    }

    // Parse GET parameters if needed (implement if required)
    pub fn parse_get(input: &str) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        // Implement parsing logic for query parameters here, if applicable
        // Example: Split input by '&' and extract key-value pairs
        for pair in input.split('&') {
            let parts: Vec<&str> = pair.split('=').collect();
            if parts.len() == 2 {
                let key = parts[0].to_string();
                let value = Value::String(parts[1].to_string());
                map.insert(key, value);
            }
        }
        map
    }

    pub fn parse_headers(input: &str) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        for header in input.split(';') {
            let parts: Vec<&str> = header.trim().split(':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();
                headers.insert(key, value);
            }
        }
        headers
    }

    pub fn new(method: &str, url: &str, body_arg: Option<&str>, url_params_arg: Option<&str>, headers_arg: Option<&str>) -> Result<Self, String> {
        let url_params = url_params_arg.map_or_else(HashMap::new, |p| Self::parse_custom_json(p).unwrap_or_default());

        let body = if method.eq_ignore_ascii_case("GET") {
            HashMap::new()
        } else if let Some(json) = body_arg {
            Self::parse_custom_json(json)?
        } else {
            HashMap::new()
        };

        // Combine body parameters with URL parameters
        let mut combined_params = url_params.clone();
        combined_params.extend(url_params.clone());

        let headers = headers_arg.map_or_else(HashMap::new, |h| Self::parse_headers(h));

        Ok(Request {
            method: method.to_string(),
            url: url.to_string(),
            body,
            headers,
            url_params : combined_params,
        })
    }
}
