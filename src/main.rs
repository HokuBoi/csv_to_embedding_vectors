use std::env;
use std::fs::File;
use std::io::{Write, BufReader};
use csv::ReaderBuilder;
use reqwest::header::AUTHORIZATION;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("No CSV file provided".into());
    }

    // Open the csv file
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    let openai_api_key = env::var("OPENAI_API_KEY")?;

    let client = reqwest::Client::new();

    let mut index = 0;
    for result in csv_reader.records() {
        let record = result?;
        
        // Join all fields in the record into a single string
        let chunk = record.iter().collect::<Vec<&str>>().join(" ");
        
        // Skip processing for empty chunks
        if chunk.is_empty() {
            continue;
        }

        let res = client.post("https://api.openai.com/v1/embeddings")
            .header(AUTHORIZATION, format!("Bearer {}", openai_api_key))
            .json(&json!({
                "input": chunk,
                "model": "text-embedding-ada-002"
            }))
            .send()
            .await?;
        let text = res.text().await?;

        // Write the response to a file
        let mut file = File::create(format!("embedding_vector_{}.txt", index))?;
        file.write_all(text.as_bytes())?;

        index += 1;
    }
    Ok(())
}
