use serde::{Deserialize, Serialize};
use std::cmp::Reverse;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rules {
    pub initial: Vec<String>,
    #[serde(rename = "final")]
    pub final_: Vec<String>,
    pub quit: Vec<String>,
    pub pre: Vec<Replacement>,
    pub post: Vec<Replacement>,
    #[serde(rename = "synon")]
    pub synonyms: Vec<Synonym>,
    #[serde(rename = "key")]
    pub keywords: Vec<Keyword>,
}

impl Rules {
    pub fn sort_keywords_by_reverse_weight(&mut self) {
        self.keywords.sort_by_key(|k| Reverse(k.weight));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Replacement {
    #[serde(rename = "in")]
    pub from: String,
    #[serde(rename = "out")]
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Synonym {
    pub label: String,
    pub list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keyword {
    pub word: String,
    pub weight: u8,
    #[serde(rename = "decomp")]
    pub decomposition: Vec<Decomposition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decomposition {
    pub pattern: String,
    pub reasmb: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_keywords_by_reverse_weight() {
        let keywords = vec![
            Keyword {
                word: "".to_string(),
                weight: 0,
                decomposition: vec![],
            },
            Keyword {
                word: "".to_string(),
                weight: 2,
                decomposition: vec![],
            },
            Keyword {
                word: "".to_string(),
                weight: 1,
                decomposition: vec![],
            },
        ];

        let mut rules = Rules {
            initial: vec![],
            final_: vec![],
            quit: vec![],
            pre: vec![],
            post: vec![],
            synonyms: vec![],
            keywords,
        };

        rules.sort_keywords_by_reverse_weight();

        let result: Vec<u8> = rules.keywords.iter().map(|k| k.weight).collect();
        let expected = vec![2, 1, 0];

        assert_eq!(expected, result);
    }
}
