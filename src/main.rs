use discord::Handler;
use eliza::Eliza;
use rules::Rules;
use serenity::prelude::*;
use std::{
    env, fs,
    io::{self, ErrorKind, Write},
    process,
    sync::Mutex,
};

mod discord;
mod eliza;
mod pattern;
mod rules;

fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();

    let mode = match args.len() {
        3 => &args[1],
        _ => {
            eprintln!("Usage: {} cli [rules-path]", args[0]);
            eprintln!("Usage: {} discord [rule-storage-file]", args[0]);
            process::exit(1);
        }
    };

    if mode == "discord" {
        let rule_storage_file = args[2].to_string();

        let eliza = match fs::read_to_string(&rule_storage_file) {
            Ok(string) => Some(Eliza::new(
                serde_json::from_str(&string).expect("could not parse JSON file"),
            )),
            Err(err) if err.kind() == ErrorKind::NotFound => None,
            Err(err) => {
                eprintln!("Failed to read file '{}'.", rule_storage_file);
                eprintln!("Error: {}", err);
                process::exit(2);
            }
        };

        let handler = Handler {
            eliza: Mutex::new(eliza),
            store: Box::new(move |rules| match serde_json::to_string(rules) {
                // TODO: map
                Ok(serialized) => match fs::write(&rule_storage_file, serialized) {
                    Ok(_) => true,
                    Err(e) => {
                        eprintln!("Could not serialize : {}", e);
                        false
                    }
                },
                Err(e) => {
                    eprintln!("Could not serialize : {}", e);
                    false
                }
            }),
            bot_id: env::var("DISCORD_BOT_ID")
                .expect("Expected the bot id in the environment")
                .parse()
                .expect("The specified bot id is not a valid number."),
        };

        let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let mut client = Client::new(&token, handler).expect("Could not create serenity client");

        if let Err(why) = client.start() {
            println!("Client error: {:?}", why);
        }
    } else if mode == "cli" {
        let filename = &args[2];

        let rule_contents = fs::read_to_string(&filename)?;
        let rules: Rules =
            serde_json::from_str(&rule_contents).expect("Could not parse JSON file.");
        let eliza = Eliza::new(rules);

        println!("{}", eliza.greeting());

        while let Some(input) = prompt("> ")? {
            let response = eliza.interact(&input);
            println!("{}", response.message);

            if response.is_farewell {
                break;
            }
        }
    }

    Ok(())
}

/// Prompts the user for a line of text, and returns it.
fn prompt(prompt: &str) -> io::Result<Option<String>> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();

    match io::stdin().read_line(&mut input)? {
        0 => Ok(None),
        _ => Ok(Some(input.to_string())),
    }
}
