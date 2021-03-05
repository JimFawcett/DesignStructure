/////////////////////////////////////////////////////////////
// TypeErasureDataFlowStructure::Executive::main.rs        //
//   - Executive creates and uses all lower level parts    //
// Jim Fawcett, https://JimFawcett.github.io, 04 Mar 2021  //
/////////////////////////////////////////////////////////////
/*
  Note: 
    Executive only creates Input instance.  The rest of
    the pipeline self installs, e.g., Input creates Compute,
    and Compute creates Output.
*/
use input::*;
use compute::*;
use output::*;

fn main() {
    let putln = || println!();

    print!("\n  -- TypeErasureDataFlowStructure::Executive --\n");

    let mut lines = 0;

    type Comp = ComputeImpl<OutputImpl>;
    let mut inp = Input::<Comp>::new();
    
    let name = "./src/main.rs";
    lines += inp.do_input(name);
    putln();

    let name = "../Input/src/lib.rs";
    lines += inp.do_input(name);
    let name = "../Input/examples/test1.rs";
    lines += inp.do_input(name);
    putln();
 
    let name = "../Compute/src/lib.rs";
    lines += inp.do_input(name);
    let name = "../Compute/examples/test1.rs";
    lines += inp.do_input(name);
    putln();
 
    let name = "../Output/src/lib.rs";
    lines += inp.do_input(name);
    let name = "../Output/examples/test1.rs";
    lines += inp.do_input(name);
    putln();
 
    print!("\n  total lines: {}", lines);

    print!("\n\n  That's all Folks!\n\n");
}
