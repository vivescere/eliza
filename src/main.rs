use eliza::Eliza;
use std::{
    env,
    io::{self, Write},
    process,
};

mod eliza;
mod rules;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = match args.len() {
        2 => &args[1],
        _ => {
            eprintln!("Usage: {} [rules-path]", args[0]);
            process::exit(1);
        }
    };

    let rules = rules::Rules::from_json_file(&filename)?;
    let eliza = Eliza::new(rules);

    println!("{}", eliza.greeting());

    while let Some(input) = prompt("> ")? {
        let response = eliza.interact(&input);
        println!("{}", response.message);

        if response.is_farewell {
            break;
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
