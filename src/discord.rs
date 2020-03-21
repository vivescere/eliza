use crate::eliza::Eliza;
use crate::rules::Rules;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::sync::Mutex;

pub struct Handler {
    pub eliza: Mutex<Option<Eliza>>,
    pub bot_id: u64,
    // TODO: error type
    pub store: Box<dyn Fn(&Rules) -> bool + Sync + Send>,
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        println!(
            "msg = {{author: {}, content: '{}', mentions: [{}]}}",
            &msg.author,
            &msg.content,
            msg.mentions
                .iter()
                .map(|m| m.id.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );

        if msg.mentions.len() != 1 {
            return;
        }

        if msg.mentions[0].id != self.bot_id {
            return;
        }

        if msg.content.contains("!load_rules") {
            let url = {
                let mut url = msg.content.replace("!load_rules", "");
                url = url.replace(&format!("<@!{}>", self.bot_id), "");
                url.trim().to_string()
            };

            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, &format!("Loading rules at {} ...", &url))
            {
                println!("Error sending message: {:?}", why);
            }

            // TODO: map
            let body: Result<String, _> = match reqwest::blocking::get(&url) {
                Ok(response) => response.text(),
                Err(e) => Err(e),
            };

            let response = match body {
                Ok(body) => {
                    let rules: Rules =
                        serde_json::from_str(&body).expect("Could not parse JSON file.");

                    if (self.store)(&rules) {
                        let new_eliza = Eliza::new(rules);
                        let mut guard = self.eliza.lock().unwrap();
                        *guard = Some(new_eliza);
                        "OK!"
                    } else {
                        eprintln!("Failed to store new rules.");
                        "Internal error.."
                    }
                }
                Err(_) => "Error loading rules.. Please check your URL.",
            };

            if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content.contains("!ping") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                println!("Error sending message: {:?}", why);
            }
        } else {
            // Generate response
            let guard = self.eliza.lock().unwrap();

            let message = msg
                .content
                .replace(&format!("<@!{}>", self.bot_id), "")
                .trim()
                .to_string();

            println!("MESSAGE: {}", message);

            let response = match guard.as_ref() {
                Some(eliza) => eliza.interact(&message).message,
                None => "Error: no rules defined".to_string(),
            };

            if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
