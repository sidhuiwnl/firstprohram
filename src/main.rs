use dotenv::dotenv;
mod modals;
use modals::client::GeminiClient;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY").unwrap_or_else(|_| "none".to_string());
    
    let client = GeminiClient::new(api_key);

    let gitdiff = Command::new("git")
        .arg("diff")
        .output()
        .expect("Failed to execute git diff");

    if gitdiff.status.success(){
        let diff_output = String::from_utf8_lossy(&gitdiff.stdout);

        let prompt = format!("Generate a complete, concise commit message in a single sentence based on the git diff:\n\n{}",
            diff_output);

        let commit_message = match client.generate_story(&prompt).await {
            Ok(story) =>  { story },
            Err(e) => { return Err(e.into())}
        };

        let status = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(commit_message)
            .status()?;

        if status.success(){
            println!("Commit successful");
        }else {
            eprintln!("Commit failed");
        }

    }else{
        let error_output = String::from_utf8_lossy(&gitdiff.stderr);
        eprintln!("Error running git diff: {}", error_output);
    }
    
    
    
    Ok(())
}