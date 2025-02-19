use reqwest::blocking::Client;
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::{BufReader, Write}};

pub struct TTS;

impl TTS {
    pub fn speak(text: &str) {
        let api_key = "api key"; // 🔹 Replace with your Azure API key
        let region = "southeastasia"; // 🔹 Replace with your Azure region
        let file_path = "output.wav";

        if TTS::generate_speech(text, api_key, region, file_path) {
            println!("✅ Speech saved to {}", file_path);
            TTS::play_audio(file_path);
        } else {
            eprintln!("❌ Failed to generate speech!");
        }
    }

    fn generate_speech(text: &str, api_key: &str, region: &str, file_path: &str) -> bool {
        let url = format!(
            "https://{}.tts.speech.microsoft.com/cognitiveservices/v1",
            region
        );

        let xml_body = format!(
            r#"<?xml version='1.0' encoding='utf-8'?>
            <speak version='1.0' xmlns='http://www.w3.org/2001/10/synthesis' xml:lang='id-ID'>
                <voice name='id-ID-GadisNeural'>{}</voice>
            </speak>"#,
            text
        );

        let client = Client::new();
        let response = client
            .post(&url)
            .header("Ocp-Apim-Subscription-Key", api_key)
            .header("Content-Type", "application/ssml+xml")
            .header("X-Microsoft-OutputFormat", "riff-16khz-16bit-mono-pcm") // 🔹 Ensure correct format
            .header("User-Agent", "RustTTS")
            .body(xml_body)
            .send();

        match response {
            Ok(mut resp) => {
                if resp.status().is_success() {
                    let mut file = File::create(file_path).expect("❌ Failed to create output file");
                    let mut audio_data = Vec::new();
                    resp.copy_to(&mut audio_data).expect("❌ Failed to read audio data");

                    file.write_all(&audio_data).expect("❌ Failed to save audio");
                    true
                } else {
                    let status = resp.status();
                    let error_body = resp.text().unwrap_or_default();
                    eprintln!("❌ Azure API Error: Status {}, Body: {}", status, error_body);
                    false
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to connect to Azure TTS: {}", e);
                false
            }
        }
    }

    fn play_audio(file_path: &str) {
        let (_stream, stream_handle) = OutputStream::try_default().expect("❌ Failed to create audio stream");
        let sink = Sink::try_new(&stream_handle).expect("❌ Failed to create sink");

        let file = File::open(file_path).expect("❌ Failed to open output file");
        let source = Decoder::new(BufReader::new(file)).expect("❌ Failed to decode audio");

        sink.append(source);
        sink.sleep_until_end();
    }
}
