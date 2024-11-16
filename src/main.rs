
use anyhow::{Result, Context};
use dotenv::dotenv;
mod modals;
use modals::client::GeminiClient;
use std::process::Command;
use::std::io::{ self, Write };

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY").unwrap_or_else(|_| "none".to_string());
    
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