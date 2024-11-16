use std::io::{stdout, Write};
use tokio::time::{sleep,Duration};
use::crossterm::style::{Color,SetForegroundColor,ResetColor};
use crossterm::ExecutableCommand;
use reqwest::Client;
use serde_json::json;
use crate::modals::*;

pub struct GeminiClient{
    api_key : String,
    client : Client,
}

impl GeminiClient {
    pub fn new(api_key : String) -> Self {
        Self {
            api_key,
            client: Client::new()
        }
    }

    pub async fn generate_story(&self,prompt : &str) -> Result<String, reqwest::Error>{

        let handling_loading = tokio::spawn(async{
            let mut stdout = stdout();
            loop {
                stdout.execute(SetForegroundColor(Color::DarkGrey)).unwrap();
                print!("\rGenerating commit message...");
                stdout.execute(ResetColor).unwrap();
                stdout.flush().unwrap();
                sleep(Duration::from_millis(100)).await;
            }
        });

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-8b:generateContent?key={}",
            self.api_key
        );

        let body = json!({
            "contents" : [{
                "parts" : [{
                    "text" : prompt
                }]
            }]
        });

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        handling_loading.abort();
        print!("\r\x1b[K"); // Clear the line
        stdout().flush().unwrap();

        if response.status().is_success() {
            let gemini_response = response.json::<GeminiResponse>().await?;
            Ok(gemini_response
                .candidates
                .first()
                .and_then(|candidate| candidate.content.parts.first())
                .map(|part| part.text.clone())
                .unwrap_or_else(|| String::from("No response generated"))
            )
            
        }else {
            Err(reqwest::Error::from(response.error_for_status().unwrap_err()))
        }

        
        
        
    }


    
}