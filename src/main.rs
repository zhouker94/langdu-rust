use clap::Parser;
use curl::easy::{Easy, List};
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::io::Write;

/**
 * CLI arguments
 */
#[derive(Parser)]
struct Cli {
    // The path to the text file to read
    input_path: std::path::PathBuf,
    // The path to the mp3 file to save
    output_path: std::path::PathBuf,
}

/**
 * Configuration for Speech API, includes api and region
 */
struct SpeechApiConfig {
    speech_key: String,
    speech_region: String,
}

/**
 * Main function as entrypoint
 */
fn main() {
    // parse CLI arguments
    let args = Cli::parse();

    // default voice to use
    let mut voice_id = "en-US-JennyNeural";
    // regex for matching voice identifier, e.g. `[en-US-JennyNeural]`
    let voice_id_re = Regex::new(r"^\[[\w\-]+\]").unwrap();

    // read API configuration from environment variables
    let api_config = get_api_key_and_region();

    // buffer for bytes from synthesis speech
    let mut buf = Vec::new();

    // read input text file from local
    let content =
        std::fs::read_to_string(&args.input_path).expect("Cannot not read input text file");

    // process text line by line
    for line in content.lines() {
        let text: String;

        voice_id = match voice_id_re.find(line) {
            // when a new voice identifier found, replace the existing one
            Some(var) => {
                let new_voice_id = var.as_str();

                // remove identifier from text
                text = line.replace(new_voice_id, "");

                new_voice_id.trim_matches(|c| c == '[' || c == ']')
            }
            None => {
                text = line.to_string();
                voice_id
            }
        };

        // skip if line empty
        let text = text.trim();
        if text.is_empty() {
            continue;
        };

        // generate SSML as string
        let ssml = get_ssml(text, voice_id);

        // send request to tts service
        send_tts_request(&api_config, &ssml, &mut buf).expect("Cannot send request");
    }

    // write to mp3 file
    let mut file = File::create(&args.output_path).expect("Failed to create file");
    file.write_all(&buf).expect("Cannot write to mp3 file");
}

/**
 * Get `SPEECH_KEY` and `SPEECH_REGION` from environement variable, and generates SpeechApiConfig
 */
fn get_api_key_and_region() -> SpeechApiConfig {
    SpeechApiConfig {
        speech_key: std::env::var("SPEECH_KEY").expect("Cannot interpret speech key"),
        speech_region: std::env::var("SPEECH_REGION").expect("Cannot interpret speech region"),
    }
}

/**
 * Generate Speech Synthesis Markup Language (SSML)
 */
fn get_ssml(text: &str, voice_id: &str) -> String {
    format!(
        "<speak version='1.0' xml:lang='en-US'>
            <voice xml:lang='en-US' xml:gender='Female' name='{}'>
                {}
            </voice>
        </speak>",
        voice_id, text,
    )
}

/**
 * Send tts request and append bytes in response to buf
 */
fn send_tts_request(
    api_config: &SpeechApiConfig,
    ssml: &String,
    buf: &mut Vec<u8>,
) -> Result<(), curl::Error> {
    let SpeechApiConfig {
        speech_key,
        speech_region,
    } = api_config;

    let mut easy = Easy::new();
    easy.url(&format!(
        "https://{}.tts.speech.microsoft.com/cognitiveservices/v1",
        speech_region
    ))?;

    let mut headers = List::new();
    headers.append(&format!("Ocp-Apim-Subscription-Key: {}", speech_key))?;
    headers.append("Content-Type: application/ssml+xml")?;
    headers.append("X-Microsoft-OutputFormat: audio-16khz-128kbitrate-mono-mp3")?;
    headers.append("User-Agent: curl")?;
    easy.http_headers(headers)?;

    let mut data = ssml.as_bytes();
    easy.post_field_size(data.len() as u64)?;

    let mut transfer = easy.transfer();
    transfer.read_function(|buf| Ok(data.read(buf).unwrap_or(0)))?;
    transfer.write_function(|data| {
        buf.extend_from_slice(data);
        Ok(data.len())
    })?;
    transfer.perform()?;

    Ok(())
}
