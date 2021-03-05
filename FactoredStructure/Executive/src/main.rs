/////////////////////////////////////////////////////////////
// FactoredStructure::Executive::main.rs                   //
//   - Executive creates and uses all lower level parts    //
// Jim Fawcett, https://JimFawcett.github.io, 04 Mar 2021  //
/////////////////////////////////////////////////////////////

use input::*;
use compute::*;
use output::*;

/*-- attempt to open file and count its lines of code --*/
fn test_ops(name: &str) -> usize {
    let mut inp = Input::new();
    let mut cmp = Compute::new();
    let out = Output::new();

    let mut lines:usize = 0;
    let opt = inp.do_input(name);
    if let Some(file) = opt {
        cmp.do_compute(name, file);
        lines = cmp.lines();
        out.do_output(name, lines);
    }
    else {
        print!("\n  couldn't process {:?}", name);
    }
    lines
}
/*-- apply test_ops to each of the parts of this app --*/
fn main() {

    print!("\n  -- FactoredStructure::Executive --\n");

    let name = "../Executive/src/main.rs";
    let mut lines = 0;
    lines += test_ops(name);
    println!();

    let name = "../Input/src/lib.rs";
    lines += test_ops(name);
    let name = "../Input/examples/test1.rs";
    lines += test_ops(name);
    println!();
 
    let name = "../Compute/src/lib.rs";
    lines += test_ops(name);
    let name = "../Compute/examples/test1.rs";
    lines += test_ops(name);
    println!();
 
    let name = "../Output/src/lib.rs";
    lines += test_ops(name);
    let name = "../Output/examples/test1.rs";
    lines += test_ops(name);
    println!();
 
    let name = "no-exist";
    lines += test_ops(name);
    println!();
 
    print!("\n  total lines: {}", lines);

    print!("\n\n  That's all Folks!\n\n");
}
