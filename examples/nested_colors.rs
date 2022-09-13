extern crate painted;

use painted::*;

/*
 * This example use painted strings in a nested way (at line 14). It shows that painted is able to
 * keep the correct color on the “!lalalala” part.
 */

fn main() {
    let world = "world".bold();
    let hello_world = format!("Hello, {}!", world);
    println!("{}", hello_world);
    let hello_world = format!("Hello, {}!lalalala", world).red();
    println!("{}", hello_world);
}
