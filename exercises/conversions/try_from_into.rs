// TryFrom is a simple and safe type conversion that may fail in a controlled way under some circumstances.
// Basically, this is the same as From. The main difference is that this should return a Result type
// instead of the target type itself.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.TryFrom.html
use std::convert::{TryInto, TryFrom};

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// Your task is to complete this implementation
// in order for the line `let p = Person::try_from("Mark,20")` to compile
// and return an Ok result of inner type Person.
// Please note that you'll need to parse the age component into a `usize`
// with something like `"4".parse::<usize>()`. The outcome of this needs to
// be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return an error
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. Extract the other element from the split operation and parse it into a `usize` as the age
// If while parsing the age, something goes wrong, then return an error
// Otherwise, then return a Result of a Person object
impl TryFrom<&str> for Person {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let fields: Vec<&str> = s.split(',').collect();
        if fields.len() < 2 { return Err(format!("Unable to split on ',' from {}", s)) };

        let name = String::from(fields[0]);
        let age = fields[1].parse::<usize>();
        if age.is_err() { return Err(format!("Unable to parse age as usize from {}", fields[1])) };
        if name.len() < 1 { return Err(format!("Unable to extract a name from {}", s)) };

        Ok(
            Person {
                name: name,
                age: age.unwrap(),
            }
        )

    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::try_from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Result<Person, _> = "Gerald,70".try_into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::try_from("");
        assert!(p.is_err());
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::try_from("Mark,20");
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    #[should_panic]
    fn test_panic_empty_input() {
        let p: Person = "".try_into().unwrap();
    }
    #[test]
    #[should_panic]
    fn test_panic_bad_age() {
        let p = Person::try_from("Mark,twenty").unwrap();
    }
}
