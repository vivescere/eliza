use crate::rules::Synonym;

/// Matches a pattern, and returns the matched groups.
///
/// This implementation supports stars, and synonyms. A synonym must start
/// with an '@'.
pub fn match_pattern<'a>(
    pattern: &str,
    input: &'a str,
    synonyms: &Vec<Synonym>,
) -> Option<Vec<&'a str>> {
    //pattern = pattern.trim(); // TODO: needed?

    if pattern == "*" {
        return Some(vec![input]);
    }

    let mut groups: Vec<&str> = Vec::new();
    let mut previous_pattern_part_is_star = false;
    let mut input_iter = input.split(" ").peekable();
    let mut input_index = 0;

    for pattern_part in pattern.split(" ") {
        if pattern_part == "*" {
            previous_pattern_part_is_star = true;
            continue;
        }

        let mut found = false;

        let start_index = input_index;
        let mut last_index = input_index;

        let mut has_space = false;

        while let Some(input_part) = input_iter.next() {
            last_index = input_index;

            input_index += input_part.len();

            if input_iter.peek().is_some() {
                has_space = true;
                input_index += 1;
            } else {
                has_space = false;
            }

            if is_compatible(input_part, pattern_part, synonyms) {
                found = true;
                break;
            }
        }

        // We could not find the pattern part, the pattern does not match
        if !found {
            return None;
        }

        if previous_pattern_part_is_star {
            // TODO: -1
            let end_index = if start_index == last_index {
                last_index
            } else {
                last_index - 1
            };
            groups.push(&input[start_index..end_index]);
        }

        if pattern_part.chars().next().unwrap() == '@' {
            if has_space {
                groups.push(&input[last_index..(input_index - 1)]);
            } else {
                groups.push(&input[last_index..input_index]);
            }
        }

        previous_pattern_part_is_star = false;
    }

    if previous_pattern_part_is_star {
        groups.push(&input[input_index..]);
    }

    Some(groups)
}

/// Checks if the given words matches the pattern part, that is either it is
/// the same, or it is a synonym.
fn is_compatible(word: &str, pattern_part: &str, synonyms: &Vec<Synonym>) -> bool {
    if word == pattern_part {
        true
    } else {
        match pattern_part.chars().next() {
            Some('@') => is_synonym(word, pattern_part, synonyms),
            _ => false,
        }
    }
}

/// Checks if the given word is a synonym associated with the given symbol.
fn is_synonym(word: &str, label: &str, synonyms: &Vec<Synonym>) -> bool {
    let synonym = synonyms
        .iter()
        .find(|s| s.label == label)
        .expect(&format!("unknown synonym {}", label));

    synonym.list.iter().any(|w| *w == word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_pattern_all() {
        let input = "hello there";
        let pattern = "*";
        let expected = Some(vec!["hello there"]);
        assert_eq!(expected, match_pattern(pattern, &input, &vec![]));
    }

    #[test]
    fn test_match_pattern_one_star() {
        // center
        let input = "hey i really like you";
        let pattern = "hey i * you";
        let expected = Some(vec!["really like"]);
        assert_eq!(expected, match_pattern(pattern, &input, &vec![]));

        // left
        let input = "really like you";
        let pattern = "* you";
        let expected = Some(vec!["really like"]);
        assert_eq!(expected, match_pattern(pattern, &input, &vec![]));

        // right
        let input = "i really like";
        let pattern = "i *";
        let expected = Some(vec!["really like"]);
        assert_eq!(expected, match_pattern(pattern, &input, &vec![]));
    }

    #[test]
    fn test_match_pattern_multiple_stars() {
        let input = "somehow i am really happy";
        let pattern = "* i am *";
        let expected = Some(vec!["somehow", "really happy"]);
        assert_eq!(expected, match_pattern(pattern, &input, &vec![]));

        let input = "how are you doing today";
        let pattern = "how * you * today";
        let expected = Some(vec!["are", "doing"]);
        assert_eq!(expected, match_pattern(pattern, &input, &vec![]));
    }

    #[test]
    fn test_match_pattern_synonym() {
        let synonyms = vec![Synonym {
            label: "@happy".to_string(),
            list: vec![
                "happy".to_string(),
                "elated".to_string(),
                "glad".to_string(),
                "better".to_string(),
            ],
        }];
        let input = "better";
        let pattern = "@happy";
        let expected = Some(vec!["better"]);
        assert_eq!(expected, match_pattern(pattern, &input, &synonyms));
    }

    #[test]
    fn test_match_pattern_synonym_and_stars() {
        let synonyms = vec![Synonym {
            label: "@be".to_string(),
            list: vec![
                "be".to_string(),
                "am".to_string(),
                "is".to_string(),
                "are".to_string(),
                "was".to_string(),
            ],
        }];
        let input = "it really was like hell";
        let pattern = "* @be * like *";
        let expected = Some(vec!["it really", "was", "", "hell"]);
        assert_eq!(expected, match_pattern(pattern, &input, &synonyms));
    }

    #[test]
    fn test_is_synonym() {
        let synonyms = vec![Synonym {
            label: "@happy".to_string(),
            list: vec![
                "happy".to_string(),
                "elated".to_string(),
                "glad".to_string(),
                "better".to_string(),
            ],
        }];

        assert!(!is_synonym("sad", "@happy", &synonyms));
        assert!(is_synonym("elated", "@happy", &synonyms));
    }

    #[test]
    #[should_panic]
    fn test_is_synonym_unknown_label() {
        is_synonym("happy", "@happy", &vec![]);
    }

    #[test]
    fn test_is_compatible_equals() {
        assert!(is_compatible("happy", "happy", &vec![]));
        assert!(!is_compatible("sad", "happy", &vec![]));
    }

    #[test]
    fn test_is_compatible_synonyms() {
        let synonyms = vec![Synonym {
            label: "@happy".to_string(),
            list: vec![
                "happy".to_string(),
                "elated".to_string(),
                "glad".to_string(),
                "better".to_string(),
            ],
        }];

        assert!(is_compatible("elated", "@happy", &synonyms));
        assert!(!is_compatible("sad", "@happy", &synonyms));
    }
}
