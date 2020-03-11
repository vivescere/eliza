use serde::Deserialize;
use std::{fs, io};

#[derive(Debug, Deserialize)]
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
    pub fn from_json_file(fname: &str) -> io::Result<Self> {
        let contents = fs::read_to_string(fname)?;
        Ok(serde_json::from_str(&contents).unwrap())
    }
}

#[derive(Debug, Deserialize)]
pub struct Replacement {
    #[serde(rename = "in")]
    pub from: String,
    #[serde(rename = "out")]
    pub to: String,
}

#[derive(Debug, Deserialize)]
pub struct Synonym {
    pub label: String,
    pub list: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Keyword {
    pub word: String,
    pub weight: u8,
    #[serde(rename = "decomp")]
    pub decomposition: Vec<Decomposition>,
}

#[derive(Debug, Deserialize)]
pub struct Decomposition {
    pub pattern: String,
    pub reasmb: Vec<String>,
}
