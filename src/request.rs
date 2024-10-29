use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub params: HashMap<String, Value>,
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

    pub fn new(method: &str, url: &str, json_arg: Option<&str>) -> Result<Self, String> {
        // Handle GET method separately; parse query parameters if applicable
        let params = if method.eq_ignore_ascii_case("GET") {
            // Extract query parameters from URL if needed
            let url_parts: Vec<&str> = url.split('?').collect();
            if url_parts.len() > 1 {
                Self::parse_get(url_parts[1]) // Parse query string
            } else {
                HashMap::new()
            }
        } else if let Some(json) = json_arg {
            Self::parse_custom_json(json)?
        } else {
            HashMap::new()
        };

        Ok(Request {
            method: method.to_string(),
            url: url.to_string(),
            params,
        })
    }
}
