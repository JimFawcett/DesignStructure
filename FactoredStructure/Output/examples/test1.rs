// test1.rs

use output::*;

fn main() {
    print!("\n  -- test Output --\n");
    let out = Output::new();
    out.do_output("SomeFile.rs", 3);

    print!("\n  That's all Folks!\n\n");
}