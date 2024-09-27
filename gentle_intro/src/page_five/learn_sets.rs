use std::{collections::{HashMap, HashSet}, error::Error, io};

use crate::greet;
// Maps
// - Maps are collection of key value which is nothing but dictionary.
// Sets
// - Sets are maps where you care only about the keys not any associated values.
//   - `insert` : only take one value
//   - `contains` : for testing wether a value is in set
//  Like all containers you can create a HashSet from an iterator.

// Repeated inserions of same key have no effect
// Order of value in a set are not important


fn make_sets(words: &str) -> HashSet<&str> {
    words.split_whitespace().collect()
}

fn sets_one() {
    let fruit = make_sets("apple orange mango banana pear apple");
    println!("Sets of fruit : {:?}", fruit); // Sets of fruit : {"pear", "orange", "apple", "mango", "banana"}
    
    // They would not be sets without usual operations.
    let colors = make_sets("orange brown black white apple mango");
    
    for c in fruit.intersection(&colors) {
        println!("{:?}", c);
    }
    // converting the iterator
    let intersect : Vec<_> = fruit.intersection(&colors).collect();
    
    println!("intersect {:?}",intersect);
}


// Example - Interactive command processing.

type CliResult = Result<String, String>;

struct Cli<'a, D>{
    data: D, 
    callbacks: HashMap<String, Box<dyn Fn(&mut D, &[&str]) -> CliResult + 'a >>
}

impl <'a, D: Sized> Cli <'a, D> {
    fn new(data: D) -> Cli<'a, D> {
        Cli { data: data, callbacks: HashMap::new() }
    }
    
    fn cmd<F>(&mut self, name: &str, callback: F) 
    where F: Fn(&mut D, &[&str]) -> CliResult + 'a
     {
        self.callbacks.insert(name.to_string(), Box::new(callback));
    }
    
    fn process(&mut self, line: &str) -> CliResult {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() == 0 {
            return Ok("".to_string());
        }
        
        match self.callbacks.get(parts[0]) {
            Some(callback) => callback(&mut self.data, &parts[1..]),
            None => Err("No such command".to_string())
        }
    }
    fn go(&mut self) {
        let mut buff = String::new();
        while  io::stdin().read_line(&mut buff).expect("Error") > 0{
            {
                let line = buff.trim_start();
                let res = self.process(line);
                println!("{:?}", res);
            }
            buff.clear();
        }
    }
    
    pub fn ok<T: ToString>(s:T) -> CliResult {
        Ok(s.to_string())
    }
    
    pub fn err<T: ToString>(s:T) -> CliResult {
        Err(s.to_string())
    }
    
}

fn interactive_program() {
    println!("Welcome to interactive program 🖥 !");
    
    struct  Data {
        answer: i32
    }
    
    let mut cli = Cli::new(Data{answer:42});
    
    cli.cmd("go",|data,args| {
        if args.len() == 0 { return Err("need 1 argument".to_string()); }
        data.answer = match args[0].parse::<i32>() {
            Ok(n) => n,
            Err(e) => return Err(e.description().to_string())
        };
        println!("got {:?}", args);
        Ok(data.answer.to_string())
    });

    cli.cmd("show",|data,_| {
        Ok(data.answer.to_string())
    });

    cli.go();

}

pub fn main() {
    greet::greet("SETS |  Learn sets");
    
    sets_one();
    
    // interactive_program();
    // Commenting out to avoid the go 
}