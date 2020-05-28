// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

fn main() {
    let mut res = 42;
    let option = Some(12);

    // This isn't really a loop which is why the below solution was confusing to me.
    // for x in option {
    //     res += x;
    // }

    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
