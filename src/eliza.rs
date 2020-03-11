use crate::rules::Rules;
use rand::seq::SliceRandom;

pub struct Eliza {
    rules: Rules,
}

impl Eliza {
    pub fn new(mut rules: Rules) -> Self {
        rules.sort_keywords_by_reverse_weight();
        Self { rules }
    }

    pub fn greeting(&self) -> &str {
        self.rules
            .initial
            .choose(&mut rand::thread_rng())
            .expect("initial rules should have at least one item")
    }

    pub fn interact(&self, input: &str) -> Response {
        let input = input.trim().to_lowercase();
        let words: Vec<&str> = input.split(" ").collect();

        if words.len() == 1 && self.rules.quit.iter().any(|w| w == words[0]) {
            let message = self
                .rules
                .final_
                .choose(&mut rand::thread_rng())
                .expect("final rules should have at least one item");

            return Response::farewell(message.to_string());
        }

        Response::normal("INTERACT".to_string())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    pub is_farewell: bool,
    pub message: String,
}

impl Response {
    fn normal(message: String) -> Self {
        Response {
            message,
            is_farewell: false,
        }
    }

    fn farewell(message: String) -> Self {
        Response {
            message,
            is_farewell: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interact_quit() {
        let rules = Rules {
            initial: vec![],
            final_: vec!["goodbye".to_string()],
            quit: vec!["quit".to_string()],
            pre: vec![],
            post: vec![],
            synonyms: vec![],
            keywords: vec![],
        };

        let eliza = Eliza::new(rules);

        assert!(!eliza.interact("hello quit").is_farewell);
        assert_eq!(
            Response::farewell("goodbye".to_string()),
            eliza.interact("quit")
        );
    }
}
