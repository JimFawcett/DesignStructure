/////////////////////////////////////////////////////////////
// DataFlowStructure::Output::test1.rs                     //
//   - Sends results to console                            //
// Jim Fawcett, https://JimFawcett.github.io, 04 Mar 2021  //
/////////////////////////////////////////////////////////////

use output::*;

fn main() {
    print!("\n  -- test Output --\n");
    let out = Output::new();
    out.do_output("SomeFile.rs", 3);

    print!("\n  That's all Folks!\n\n");
}