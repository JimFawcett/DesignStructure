/////////////////////////////////////////////////////////////
// PluginDataFlowStructure::Executive::main.rs             //
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

    print!("\n  -- PluginDataFlowStructure::Executive --\n");

    let mut lines = 0;

    let mut inp = Input::<ComputeImpl>::new();
    let out = OutputImpl::new();
    let app = inp.get_app();
    app.set_output(Box::new(out));
    
    let name = "../Executive/src/main.rs";
    let srclines = inp.do_input(name);
    lines += srclines;
    putln();

    let name = "../Input/src/lib.rs";
    let srclines = inp.do_input(name);
    lines += srclines;
    let name = "../Input/examples/test1.rs";
    let srclines = inp.do_input(name);
    lines += srclines;
    putln();

    let name = "../Compute/src/lib.rs";
    let srclines = inp.do_input(name);
    lines += srclines;
    let name = "../Compute/examples/test1.rs";
    let srclines = inp.do_input(name);
    lines += srclines;
    putln();

    let name = "../Output/src/lib.rs";
    let srclines = inp.do_input(name);
    lines += srclines;
    let name = "../Output/examples/test1.rs";
    let srclines = inp.do_input(name);
    lines += srclines;
    putln();

    let name = "no-exist";
    let srclines = inp.do_input(name);
    lines += srclines;
    putln();

    print!("\n  total lines: {}", lines);

    print!("\n\n  That's all Folks!\n\n");
}
