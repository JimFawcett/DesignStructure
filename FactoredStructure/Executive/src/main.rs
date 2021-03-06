/////////////////////////////////////////////////////////////
// FactoredStructure::Executive::main.rs                   //
//   - Executive creates and uses all lower level parts    //
// Jim Fawcett, https://JimFawcett.github.io, 04 Mar 2021  //
/////////////////////////////////////////////////////////////
#![allow(dead_code)]
#![allow(unused_imports)]
use input::*;
use compute::*;
use output::*;

/*-- module of command line utilities --*/
mod exec_proc_mod;
use exec_proc_mod::{parse_cmdln, show_cmdln};

/*-- run using command line args --*/
fn use_args_for_run() -> usize {
    let args = parse_cmdln();
    let mut total_lines:usize = 0;

    for name in &args {
        let mut lines = 0;
        let mut inp = Input::new();
        let mut cmp = Compute::new();
        let out = Output::new();
    
        let opt = inp.do_input(name);
        if let Some(file) = opt {
            cmp.do_compute(name, file);
            lines = cmp.lines();
            out.do_output(name, lines);
        }
        else {
            print!("\n  couldn't process {:?}", name);
        }
        total_lines += lines;   
    }
    total_lines        
}
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
/*-- run test_ops for each package in this project --*/
fn run_test_ops() -> usize {
    let name = "../Executive/src/main.rs";
    let mut lines = 0;
    lines += test_ops(name);
    let name = "../Executive/src/exec_proc_mod.rs";
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

    lines
}
/*-- Executive processing --*/
fn main() {

    print!("\n  -- FactoredStructure::Executive --\n");

    let lines = run_test_ops();
    // let lines = use_args_for_run();

    print!("\n  total lines: {}", lines);

    print!("\n\n  That's all Folks!\n\n");
}
