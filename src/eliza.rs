use crate::rules::Rules;

pub struct Eliza {
    rules: Rules,
}

impl Eliza {
    pub fn new(mut rules: Rules) -> Self {
        rules.sort_keywords_by_reverse_weight();
        Self { rules }
    }

    pub fn greeting(&self) -> &str {
        "GREETINGS"
    }

    pub fn interact(&self, input: &str) -> Response {
        Response::normal("INTERACT".to_string())
    }
}

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
