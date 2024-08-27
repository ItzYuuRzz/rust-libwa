use neon::prelude::*;
use reqwest::blocking::Client;
use serde_json::{from_str, json, Value};

fn send_kopolot_request(mut cx: FunctionContext) -> JsResult<JsString> {
    let body = json!({
        "ask": cx.argument::<JsString>(0)?.value(&mut cx),
        "apikey": "WhoIsSC2",
    });

    let client = Client::new();
    let response = client
        .post("http://0.0.0.0:1234/chat")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .unwrap();

    if response.status().is_success() {
        let json: Value = from_str(&response.text().unwrap()).unwrap();

        if let Some(content) = json["content"].as_str() {
            return Ok(cx.string(content));
        }
    }

    cx.throw_error("Request failed")
}

fn generate_content(mut cx: FunctionContext) -> JsResult<JsString> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro-exp-0801:generateContent?key={}",
        "AIzaSyCJ42EvIbSONRh2nM-txNWPX7jfcJ-RoHU"
    );

    let body = json!({
        "contents": [
            {
                "role": "user",
                "parts": [
                    {
                        "text": cx.argument::<JsString>(0)?.value(&mut cx),
                    },
                ],
            },
        ],
        "generationConfig": {
            "temperature": 1,
            "topK": 64,
            "topP": 0.95,
            "maxOutputTokens": 8192,
            "responseMimeType": "text/plain",
        },
        "safetySettings": [
            {
                "category": "HARM_CATEGORY_HARASSMENT",
                "threshold": "BLOCK_MEDIUM_AND_ABOVE",
            },
            {
                "category": "HARM_CATEGORY_HATE_SPEECH",
                "threshold": "BLOCK_MEDIUM_AND_ABOVE",
            },
            {
                "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT",
                "threshold": "BLOCK_MEDIUM_AND_ABOVE",
            },
            {
                "category": "HARM_CATEGORY_DANGEROUS_CONTENT",
                "threshold": "BLOCK_MEDIUM_AND_ABOVE",
            },
        ],
    });

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .unwrap();

    if response.status().is_success() {
        let text = response.text().unwrap();
        let json: Value = from_str(&text).unwrap();

        if let Some(candidates) = json["candidates"].as_array() {
            if let Some(first_candidate) = candidates.first() {
                if let Some(content) = first_candidate["content"]["parts"].as_array() {
                    if let Some(first_part) = content.first() {
                        if let Some(text) = first_part["text"].as_str() {
                            return Ok(cx.string(text));
                        }
                    }
                }
            }
        }
    }

    cx.throw_error("Request failed")
}

#[neon::main]
pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generateContent", generate_content);
    cx.export_function("sendKopolotRequest", send_kopolot_request);
    Ok(())
}
