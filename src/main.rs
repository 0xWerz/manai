use std::env;
use std::io::{self, Write};

mod openai;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the OpenAI API key from the .env file
    dotenv::dotenv().ok();

    // Get the API key from the environment and check if it exists
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            print!("Enter your OpenAI API key: ");
            io::stdout().flush()?;
            let mut api_key = String::new();
            io::stdin().read_line(&mut api_key)?;
            let api_key = api_key.trim().to_owned();
            std::fs::write(".env", format!("OPENAI_API_KEY={}", api_key))?;
            api_key
        }
    };

    // Create an OpenAI chat client
    let openai_client = openai::OpenAiClient::new(&api_key)?;

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--interactive" {
        interactive_loop(&openai_client).await?;
    } else {
        if args.len() > 1 {
            let prompt = &args[1];
            let response = openai_client.generate_response(prompt.trim()).await?;
            println!("AI: {}", response);
        } else {
            println!("Please provide a prompt as a command-line argument or use --interactive for interactive mode.");
        }
    }

    Ok(())
}

async fn interactive_loop(openai_client: &openai::OpenAiClient) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Read user input
        print!("Prompt: ");
        io::stdout().flush()?;
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;

        // Generate AI response
        let response = openai_client.generate_response(user_input.trim()).await?;

        // Print AI response
        println!("AI: {}", response);
    }
}
