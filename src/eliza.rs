use crate::pattern::match_pattern;
use crate::rules::{Decomposition, Replacement, Rules};
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

        let input = Eliza::rewrite(&input, &self.rules.pre);

        let message = match self.apply_best_matching_pattern(&input) {
            Some((groups, decomposition)) => {
                let template = decomposition
                    .reasmb
                    .choose(&mut rand::thread_rng())
                    .expect("reasmb rules should have at least one item");

                self.format_template(&template, &groups)
            }
            None => self.random_response().to_string(),
        };

        Response::normal(message)
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

    fn apply_best_matching_pattern<'a, 'b>(
        &'b self,
        input: &'a str,
    ) -> Option<(Vec<&'a str>, &'b Decomposition)> {
        // NOTE: the rules are sorted in Eliza::new to have the highest weight first.

        for keyword in &self.rules.keywords {
            if !input.contains(&keyword.word) {
                continue;
            }

            for decomposition in &keyword.decomposition {
                let result = match_pattern(&decomposition.pattern, &input, &self.rules.synonyms);

                if let Some(groups) = result {
                    // println!(
                    //     "[keyword={}, pattern={}]",
                    //     keyword.word, decomposition.pattern
                    // );

                    return Some((groups, decomposition));
                }
            }
        }

        None
    }

    fn format_template(&self, template: &str, groups: &Vec<&str>) -> String {
        let mut output = String::new();

        for template_part in template.split(" ") {
            if template_part.starts_with("(") {
                // TODO: support group numbers > 9
                let group_number = &template_part[1..2];

                if let Ok(group_number) = group_number.parse::<usize>() {
                    let group = groups[group_number - 1];
                    output.push_str(&Eliza::rewrite(group, &self.rules.post));
                    output.push_str(" ");
                    continue;
                }
            }

            output.push_str(template_part);
            output.push_str(" ");
        }

        output.pop();
        output
    }

    fn rewrite(input: &str, replacements: &Vec<Replacement>) -> String {
        input
            .split(" ")
            .map(|w| match replacements.iter().find(|r| &r.from == w) {
                Some(r) => r.to.clone(),
                None => w.to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ")
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
    use crate::rules::Keyword;

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

    #[test]
    fn test_apply_best_matching_pattern() {
        let rules = Rules {
            initial: vec![],
            final_: vec![],
            quit: vec![],
            pre: vec![],
            post: vec![],
            synonyms: vec![],
            keywords: vec![
                Keyword {
                    word: "test".to_string(),
                    weight: 1,
                    decomposition: vec![Decomposition {
                        pattern: "test *".to_string(),
                        reasmb: vec!["test".to_string()],
                    }],
                },
                xnone(),
                Keyword {
                    word: "hello".to_string(),
                    weight: 2,
                    decomposition: vec![Decomposition {
                        pattern: "* hello".to_string(),
                        reasmb: vec!["hello".to_string()],
                    }],
                },
            ],
        };

        let eliza = Eliza::new(rules);

        let (groups, decomposition) = eliza.apply_best_matching_pattern("test hello").unwrap();
        assert_eq!(vec!["test"], groups);
        assert_eq!("hello".to_string(), decomposition.reasmb[0]);

        let (groups, decomposition) = eliza.apply_best_matching_pattern("test 123").unwrap();
        assert_eq!(vec!["123"], groups);
        assert_eq!("test".to_string(), decomposition.reasmb[0]);

        assert!(eliza.apply_best_matching_pattern("testing").is_none());
    }

    #[test]
    fn test_format_template() {
        let rules = Rules {
            initial: vec![],
            final_: vec![],
            quit: vec![],
            pre: vec![],
            post: vec![Replacement {
                from: "i".to_string(),
                to: "you".to_string(),
            }],
            synonyms: vec![],
            keywords: vec![],
        };

        let eliza = Eliza::new(rules);

        let output = eliza.format_template("test (1)", &vec!["123"]);
        assert_eq!("test 123", &output);

        let output = eliza.format_template("(1) test (3)", &vec!["123", "", "i"]);
        assert_eq!("123 test you", &output);
    }

    #[test]
    fn test_rewrite() {
        let replacements = vec![Replacement {
            from: "123".to_string(),
            to: "321".to_string(),
        }];

        let output = Eliza::rewrite("testing 123 hello world", &replacements);
        assert_eq!("testing 321 hello world", output);
    }

    #[test]
    fn test_interact_random_answer() {
        let rules = Rules {
            initial: vec![],
            final_: vec![],
            quit: vec![],
            pre: vec![],
            post: vec![],
            synonyms: vec![],
            keywords: vec![
                Keyword {
                    word: "test".to_string(),
                    weight: 1,
                    decomposition: vec![Decomposition {
                        pattern: "test *".to_string(),
                        reasmb: vec!["test".to_string()],
                    }],
                },
                xnone(),
                Keyword {
                    word: "hello".to_string(),
                    weight: 2,
                    decomposition: vec![Decomposition {
                        pattern: "* hello".to_string(),
                        reasmb: vec!["hello".to_string()],
                    }],
                },
            ],
        };

        let eliza = Eliza::new(rules);

        let expected = Response::normal("random".to_string());
        assert_eq!(expected, eliza.interact("world"));
    }

    #[test]
    fn test_interact_chooses_pattern_with_highest_weight() {
        let rules = Rules {
            initial: vec![],
            final_: vec![],
            quit: vec![],
            pre: vec![],
            post: vec![],
            synonyms: vec![],
            keywords: vec![
                Keyword {
                    word: "hello".to_string(),
                    weight: 1,
                    decomposition: vec![Decomposition {
                        pattern: "*".to_string(),
                        reasmb: vec!["hello".to_string()],
                    }],
                },
                xnone(),
                Keyword {
                    word: "world".to_string(),
                    weight: 2,
                    decomposition: vec![Decomposition {
                        pattern: "*".to_string(),
                        reasmb: vec!["world".to_string()],
                    }],
                },
            ],
        };

        let eliza = Eliza::new(rules);

        let expected = Response::normal("world".to_string());
        assert_eq!(expected, eliza.interact("hello world"));
    }

    #[test]
    fn test_interact_rewrites_groups() {
        let rules = Rules {
            initial: vec![],
            final_: vec![],
            quit: vec![],
            pre: vec![],
            post: vec![],
            synonyms: vec![],
            keywords: vec![
                xnone(),
                Keyword {
                    word: "world".to_string(),
                    weight: 1,
                    decomposition: vec![Decomposition {
                        pattern: "* world".to_string(),
                        reasmb: vec!["(1) world".to_string()],
                    }],
                },
            ],
        };

        let eliza = Eliza::new(rules);

        let expected = Response::normal("hello world".to_string());
        assert_eq!(expected, eliza.interact("hello world"));
    }

    #[test]
    fn test_new_sorts_keywords_in_reverse_weight_order() {
        let rules = Rules {
            initial: vec![],
            final_: vec![],
            quit: vec![],
            pre: vec![],
            post: vec![],
            synonyms: vec![],
            keywords: vec![
                Keyword {
                    word: "hello".to_string(),
                    weight: 1,
                    decomposition: vec![Decomposition {
                        pattern: "*".to_string(),
                        reasmb: vec!["hello".to_string()],
                    }],
                },
                xnone(),
                Keyword {
                    word: "world".to_string(),
                    weight: 2,
                    decomposition: vec![Decomposition {
                        pattern: "*".to_string(),
                        reasmb: vec!["world".to_string()],
                    }],
                },
            ],
        };

        let eliza = Eliza::new(rules);

        let weights: Vec<u8> = eliza.rules.keywords.iter().map(|k| k.weight).collect();
        let expected: Vec<u8> = vec![2, 1, 0];

        assert_eq!(expected, weights);
    }

    #[test]
    #[should_panic]
    fn test_greetings_panics_no_rules() {
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
        eliza.greeting();
    }

    #[test]
    fn test_greetings() {
        let rules = Rules {
            initial: vec!["hello".to_string(), "world".to_string()],
            final_: vec![],
            quit: vec![],
            pre: vec![],
            post: vec![],
            synonyms: vec![],
            keywords: vec![xnone()],
        };

        let eliza = Eliza::new(rules);
        assert!(vec!["hello", "world"].contains(&eliza.greeting()));
    }
}
