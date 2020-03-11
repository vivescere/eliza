use eliza::Eliza;
use std::{env, io, process};

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

    Ok(())
}
