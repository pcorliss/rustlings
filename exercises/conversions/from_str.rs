// This does practically the same thing that TryFrom<&str> does.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// Steps:
// 1. If the length of the provided string is 0, then return an error
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. Extract the other element from the split operation and parse it into a `usize` as the age
// If while parsing the age, something goes wrong, then return an error
// Otherwise, then return a Result of a Person object
impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
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
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        assert!("John,32".parse::<Person>().is_ok());
    }
    #[test]
    #[should_panic]
    fn missing_age() {
        "John".parse::<Person>().unwrap();
    }
}
