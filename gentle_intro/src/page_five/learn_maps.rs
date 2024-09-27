// Maps.
//  - sometime called as associative arrays or dicts
//  - let you look up values associated with a key.
//  - can be achived the same with array of tuples: but not good for large map
//  - `HashMap` does much better when there are lot of key/value pairs to be searched.

use std::collections::HashMap;


fn map_one() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    
    assert_eq!(map.contains_key("one"), true);
    assert_eq!(map.contains_key("four"), false);
    assert_eq!(map.get("two"), Some(&2));   // get returns reference to values.
    
    let two_value = map.get("two");
    
    println!("value of two -> {:?} => {}", two_value, two_value.unwrap());
}

// `get_mut`

fn map_two() {
    let mut map = HashMap::new();
    
    map.insert("one", 2);
    
    println!("value ONE before {}", map.get("one").unwrap());
    
    // Note: writable ref. takes place in it's block. otherwise we would have mutable borrow lasting until
    // the end, and rust won't allow you to borrow from 'map' like map.get("one");
    // 
    {
        let mref = map.get_mut("one").unwrap();
        *mref = 200;
    } 
    
    println!("value ONE after {}", map.get("one").unwrap());
    
}

fn map_three() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    
    // no-one calls unwrap except in example. so you can somethjing liek 
    
    if let Some(k) = map.get("one") {
        let res = k * 2;
        assert_eq!(res, 2);
    }
    
    if let Some(v) = map.get("three") {
        let res = v + 1;
        assert_eq!(res, 4);
    }
    
    match map.get_mut("two") {
        Some(mref) => *mref = 200,
        None => panic!("_now_ we can panic with error!")
    }
    
    // can iterate over the key/value pair but not in the order.
    for (k,v) in map.iter() {
        println!("key {k} value {v}");
    }
    
}

// Example : Counting words.
// - Text is count word frequency. 
// - `split_whitespace` can break text into the words. and words must contain punctuation.
// - `is_alphabetic` check for only alphabetic chars exclude other 
// - `entry`
// - `split` works on single-character delimiters, so any punctuation or extra spaces causes a new split

fn counting_words() {
    let mut word_count_map = HashMap::new();
    let text = "can break text into the words. and words must contain into text".to_string();
    for s in text.split(|c:char| ! c.is_alphabetic()) {
        let word = s.to_lowercase();
        let count = word_count_map.entry(word).or_insert(0);
        *count += 1;
        
        /* 
            // Or can be write in the way with single liner.        
            // word_count_map.entry(word).and_modify(|count| *count += 1).or_insert(1);
        */
    }
    
    println!("value of count dict {:?}",word_count_map);
    
    println!("Total of Unique Words {}", word_count_map.len());
    
    println!("values {:?} keys {:?}", word_count_map.values(), word_count_map.keys());
    
    for (k,v) in word_count_map.iter() {
        println!("'{k}' occurs {v} times!");
    }
    
    // Question: Find the 2 most comply used words.
    let mut word_entries: Vec<_> = word_count_map.into_iter().collect();
    println!("word entries {:?}", word_entries);
    word_entries.sort_by(|a,b| b.1.cmp(&a.1));
    println!("word entries {:?}", word_entries);
    for (word, count) in word_entries.iter().take(2)  {
        println!("{word} - {count}");
    }
    
}



pub fn main() {
    println!("LEARN MAPS");
    map_one();   
    map_two();
    map_three();
    counting_words();   
}