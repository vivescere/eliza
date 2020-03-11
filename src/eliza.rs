use crate::rules::Rules;

pub struct Eliza {
    rules: Rules,
}

impl Eliza {
    pub fn new(mut rules: Rules) -> Self {
        rules.sort_keywords_by_reverse_weight();
        Self { rules }
    }
}
