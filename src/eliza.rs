use crate::rules::{Decomposition, Keyword, Rules};
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

        if self.rules.quit.contains(&input) {
            let message = self
                .rules
                .final_
                .choose(&mut rand::thread_rng())
                .expect("final rules should have at least one item");

            return Response::farewell(message.to_string());
        }

        Response::normal(self.random_response().to_string())
    }

    fn random_response(&self) -> &str {
        let keyword = self
            .rules
            .keywords
            .iter()
            .find(|k| k.word == "xnone")
            .expect("The keyword 'xnone' was not found.");

        keyword.decomposition[0]
            .reasmb
            .choose(&mut rand::thread_rng())
            .expect("the 'xnone' keyword reasmb does not have any items")
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

    /// Simple 'xnone' keyword (which are used for random messages), which
    /// only has one message: 'random'.
    fn xnone() -> Keyword {
        Keyword {
            word: "xnone".to_string(),
            weight: 0,
            decomposition: vec![Decomposition {
                pattern: "*".to_string(),
                reasmb: vec!["random".to_string()],
            }],
        }
    }

    #[test]
    fn test_interact_quit() {
        let rules = Rules {
            initial: vec![],
            final_: vec!["goodbye".to_string()],
            quit: vec!["quit".to_string()],
            pre: vec![],
            post: vec![],
            synonyms: vec![],
            keywords: vec![xnone()],
        };

        let eliza = Eliza::new(rules);

        assert!(!eliza.interact("hello quit").is_farewell);
        assert_eq!(
            Response::farewell("goodbye".to_string()),
            eliza.interact("quit")
        );
    }

    #[test]
    #[should_panic]
    fn test_random_response_no_xnone() {
        let rules = Rules {
            initial: vec![],
            final_: vec![],
            quit: vec![],
            pre: vec![],
            post: vec![],
            synonyms: vec![],
            keywords: vec![],
        };

        let eliza = Eliza::new(rules);
        eliza.random_response();
    }

    #[test]
    fn test_random_response() {
        let rules = Rules {
            initial: vec![],
            final_: vec![],
            quit: vec![],
            pre: vec![],
            post: vec![],
            synonyms: vec![],
            keywords: vec![xnone()],
        };

        let eliza = Eliza::new(rules);
        assert_eq!("random", eliza.random_response());
    }
}
