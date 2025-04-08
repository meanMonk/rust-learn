# Error handling

Most errors aren’t serious enough to require the program to stop entirely.
Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.
For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

## Further information

- [Error Handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Boxing errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)


### Learnings.

```rust 
enum Result<T, E> {
  Ok(T),
  Err(E)
}


use std::fs:File;
use std::io::ErrorKind;

fn main() {
  let greeting_file_result = File::open("hello.txt");
  
  let greenting_file = match greeting_file_result {
    Ok(file) => file ,
    Err(error) => panic!("failed to opened file : {error:?}");
  }

  /// ## Matching on different Errors. 
  
  let greeting_file = match greeting_file_result {
    Ok(file) => file, 
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc, 
        Err(e) => panic!("Problem creating the file {e:?}"),
      },
      other_error => {
        panic!("Problem opening the file {other_error?}");
      }
    },
  }
  
  
  /// ### Alternative to using match with Result<T,E> 
  
  let greeting_file = File::open("hello.txt")
    .unwrap_or_else(|error| {
      if error.kind() == ErrorKind::NotFound {
        File::create("hello.txt")
          .unwrap_or_else(|error| {
            panic!("Problem creating the file: {error:?}");
          })
      } else {
        panic!("Problem opening the file {error:?}");
      }
    })
  
  
  // Shortcuts for panic on Error `unwrap` and `expect`
  let greeting_file = File::open("hello.txt").unwrap();
  
  // `expect` - let us also choose the panic error message instead of unwrap we can provide good message
  let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
  
  // Propagating Errors.
  
  let username_file_result = File::open("hello.txt");
  
  let mut username_file =  match username_file_result {
    Ok(file) => file, 
    Err(e) => return Err(e),
  }
  
  let mut username = String::new();
  
  match username_file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(e) => Err(e),
  }
  
  // A shortcut for propagating errors: the ? operator
  
  let mut username_file =  File::open("hello.txt")?;
  let mut username = String::new();
  
  username_file.read_to_string(&mut username)?;
  
  Ok(username)
  
  // short hand implementation.
  
  let mut username =  String::new();
  
  File::open("hello.txt")?.read_to_string(&mut username)?;
  
  Ok(username)
}


fn read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}


// ### Where the ? operator can be used

fn main() {
  let greeting_file =  File::open("hello.txt")?;
}

fn last_char_of_first_line(text: &str) -> Option<char> {
  text.lines().next()?.chars().last()
}

// ## main can also return Result<(), E>.

fn main() -> Result<(), Box<dyn Error>> {
  let greeting_file = File::open("hello.txt")?;
  
  Ok(())
}



```
