pub struct SimpleTokenizer;

impl SimpleTokenizer {
    pub fn tokenize(text: &str) -> Vec<String> {
        text
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() || c.is_whitespace() { c } else { ' ' })
            .collect::<String>()
            .split_whitespace()
            .map(|t| t.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let tokens = SimpleTokenizer::tokenize("Hello, world!");
        assert_eq!(tokens, vec!["hello", "world"])
    }
}