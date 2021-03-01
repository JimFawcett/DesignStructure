// test1.rs

use input::*;

fn main() {
    print!("\n  -- input::test1 --\n");

    let mut inp = Input::new();
    let name = "./src/lib.rs";
    inp.do_input(name);

    print!("\n\n  That's all Folks!\n\n");
}