mod tts;
use serde_json::Value;
use tts::TTS;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};
struct GenerateText;

impl GenerateText {
    fn connect(prompt: &str) -> String {
        let host = "localhost:11434";
        let endpoint = "/api/generate";

        let mut stream = TcpStream::connect(host).expect("Failed to connect to Ollama");

        let request_body = format!(
            r#"{{"model": "deepseek-r1", "prompt": "{}", "stream": false}}"#,
            prompt
        );

        let request = format!(
            "POST {} HTTP/1.1\r\n\
            Host: {}\r\n\
            Content-Type: application/json\r\n\
            Content-Length: {}\r\n\
            Connection: close\r\n\
            \r\n\
            {}",
            endpoint, host, request_body.len(), request_body
        );

        stream
            .write_all(request.as_bytes())
            .expect("Failed to write to stream");

        let reader = BufReader::new(stream);
        let mut response_body = String::new();
        let mut is_body = false;

        for line in reader.lines() {
            let line = line.expect("Failed to read line");

            if line.is_empty() {
                is_body = true;
                continue;
            }

            if is_body {
                response_body.push_str(&line);
            }
        }

        // Extract only the JSON part from response_body
        if let Some(start) = response_body.find('{') {
            if let Some(end) = response_body.rfind('}') {
                response_body = response_body[start..=end].to_string();
            }
        }

        // Try to parse JSON safely
        let parsed: Value = match serde_json::from_str(&response_body) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                return "Error: Failed to parse AI response".to_string();
            }
        };

        // Extract and clean response
        if let Some(ai_response) = parsed["response"].as_str() {
            let clean_response = ai_response.replace("<think>", "").replace("</think>", "").trim().to_string();
            
            println!("{}", clean_response);
            return clean_response;
        }

        "Error: No valid response from AI".to_string()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    TTS::speak(&GenerateText::connect(&args[1..].join(" ")));
}
