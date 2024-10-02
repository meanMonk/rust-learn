use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    
    let words = words
        .to_lowercase()
        .split(|c: char| c.is_whitespace() || c == ',')
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();
    
    let mut word_count_mut = HashMap::new();
    
    for word in words {
        word_count_mut.entry(word).and_modify(|v| *v += 1).or_insert(1);
    }
    
    
    word_count_mut
}