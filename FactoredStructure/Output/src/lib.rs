// FactoredStructure::output

#[derive(Debug)]
pub struct Output {

}
impl Output {
    pub fn new() -> Output {
        Output {}
    }
    pub fn do_output(&self, name: &str, lines: usize) {
        print!("\n  file {:?} has {} lines of code", name, lines);
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
