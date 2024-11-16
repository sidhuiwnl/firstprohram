use::std::path::PathBuf;
use::std::fs;
use anyhow::{Result, Context};
use dotenv::dotenv;
mod modals;
use modals::client::GeminiClient;
use std::process::Command;
use::std::io::{ self, Write };


fn get_config_path() -> PathBuf{
    if cfg!(windows) {
        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(appdata).join("commitai").join("config")
    }else {
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
        .join(".config")
        .join("commitai")
        .join("config")
    }
}

fn load_api_key() -> Result<String> {
    let config_path = get_config_path();
    let config = fs::read_to_string(&config_path)
        .context(format!("Failed to read config file at {:?}", config_path))?;
    
    for line in config.lines() {
        if line.starts_with("GOOGLE_API_KEY=") {
            return Ok(line.trim_start_matches("GOOGLE_API_KEY=").to_string());
        }
    }
    
    Err(anyhow::anyhow!("GOOGLE_API_KEY not found in config file"))
}


#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    let api_key = load_api_key()?;

    
    let client = GeminiClient::new(api_key);

    let status = Command::new("git")
        .args(["status","--porcelain"])
        .output()
        .context("Failed to check git status")?;

    if status.stdout.is_empty(){
        println!("No changes detected. Please stage some changes first.");
        return Ok(());
    };

    let gitdiff = Command::new("git")
        .args(["diff","--staged"])
        .output()
        .context("Failed to get git diff")?;
        
    if !gitdiff.status.success(){
        let error_output = String::from_utf8_lossy(&gitdiff.stderr);
        eprintln!("Error running git diff: {}", error_output);return Ok(());
    };

    let diff_output = String::from_utf8_lossy(&gitdiff.stdout);

    if diff_output.trim().is_empty() {
        println!("No staged changes found. Use 'git add' to stage changes.");
        return Ok(());
    };

     let prompt = format!(
        "Generate a complete, concise commit message in a single sentence based on the git diff. \
         Use present tense, be specific, and follow conventional commits format (feat/fix/docs/style/refactor/test/chore):\n\n{}",
        diff_output
    );

    let commit_message = client.generate_story(&prompt).await?;

    println!("\nProposed commit message:\n{}\n", commit_message);
    print!("Do you want to use this message? [y/n/e(edit)]: "); 
    io::stdout().flush()?;

    let mut input  = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().to_lowercase().as_str(){
        "y" | "yes" => {
            let status = Command::new("git")
                .args(["commit", "-m", &commit_message])
                .status()
                .context("Failed to commit changes")?;
            if status.success() {
                println!("✓ Commit successful");
            } else {
                eprintln!("✗ Commit failed");
            }
        }
        "n" | "no" =>{
            print!("Commit cancelled")
        }
        _ =>{
            println!("Invalid response. Commit cancelled.");
        }
    };

    

   Ok(())
}