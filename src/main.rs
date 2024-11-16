
use dotenv::dotenv;
mod modals;
use modals::client::GeminiClient;
use std::process::Command;
use::std::io::{ self, Write };

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

        print!("{}",commit_message);
        print!("Do you want to add this as the commit message(y/n):");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim().to_lowercase();

        if input == "yes"{
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
        }else if input == "no"{
            println!("You chose to exit.");
        }else{
            println!("Invalid response. Please answer 'yes' or 'no'.");
        }

        

    }else{
        let error_output = String::from_utf8_lossy(&gitdiff.stderr);
        eprintln!("Error running git diff: {}", error_output);
    }
    
    
    
    Ok(())
}