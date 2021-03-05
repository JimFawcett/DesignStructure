/////////////////////////////////////////////////////////////
// PlugInDataFlowStructure::Input::test1.rs                //
//   - Attempts to return line count from file             //
// Jim Fawcett, https://JimFawcett.github.io, 04 Mar 2021  //
/////////////////////////////////////////////////////////////

use input::*;
use std::fs::*;

struct MockCompute {}
impl Compute for MockCompute {
    fn new() -> MockCompute {
        MockCompute{}
    }
    fn do_compute(&mut self, name: &str, _file:File) {
        print!("\n  {} doing mock computation", name);
    }
    fn lines(&self) -> usize {
        0
    }
}
fn main() {
    print!("\n  -- input::test1 --\n");

    let mut inp = Input::<MockCompute>::new();
    let name = "./src/lib.rs";
    let lines = inp.do_input(name);
    print!("\n  received {} lines from compute", lines);
    print!("\n\n  That's all Folks!\n\n");
}