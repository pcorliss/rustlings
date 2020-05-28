// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return factorial of num
    // Do not use:
    // - return
    // For extra fun don't use:
    // - imperative style loops (for, while)
    // - additional variables
    // For the most fun don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    //
    // Recursive solution with matcher
    // match num {
    //     0 | 1 => 1,
    //     _ => num * factorial(num - 1),
    // }

    // Hint mentions rangers and iterators
    // Docs show this method
    //(1..(num + 1)).product()

    // Proper use of inclusive range
    (1..=num).product()

    // They probably wanted us to use the `fold` method ot do things manually
    // (1..=num).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
