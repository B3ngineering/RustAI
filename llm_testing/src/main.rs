use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

use csv::Reader;
use llm_chain::{executor, parameters, prompt, step::Step};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up the environment variable for the OpenAI API key
    let exec = executor!()?;

    // Declare the file and reader
    let file = File::open("data.csv")?;
    let mut reader = Reader::from_reader(file);
    
    // Parse the csv data into a string variable
    let mut csv_data = String::new();
    for result in reader.records() {
        let record = result?;
        csv_data.push_str(&record.iter().collect::<Vec<_>>().join(","));
        csv_data.push('\n');
    }
    

    // Set up the loop in which the user will ask questions to our helper
    loop {
        println!("Enter your prompt or use 'quit' to exit:");
        io::stdout().flush()?;
        let mut prompt = String::new();
        io::stdin().read_line(&mut prompt)?;
        prompt = prompt.trim().to_string();

        if prompt.to_lowercase() == "quit" {
            break;
        }

        let prompt_string = format!(
            "You are a data analyst tasked with analyzing a CSV file containing information about individuals, including their name, age, occupation, city, favorite sport, and annual income. Your goal is to provide clear and concise answers to the given questions based on the data provided.
            Question: {}\n\nCSV Data:\n{}",
            prompt, csv_data
        );

        let step = Step::for_prompt_template(prompt!("{}", &prompt_string));
        let res = step.run(&parameters!(), &exec).await?;
        println!("{}", res.to_immediate().await?.as_content());
    }
    
    Ok(())

}
