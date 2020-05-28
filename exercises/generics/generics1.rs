// This shopping list program isn't compiling! 
// Use your knowledge of generics to fix it.

fn main() {
    // All three work. not sure which is better stylistically. Also not sure what answer satisfies
    // the original question
    // let mut shopping_list: Vec<_> = Vec::new();
    // let mut shopping_list: Vec<&str> = Vec::new();
    let mut shopping_list = Vec::new();
    shopping_list.push("milk");
}

