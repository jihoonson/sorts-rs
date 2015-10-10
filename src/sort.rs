pub trait Sort {
    fn new() -> Self;
    fn name(&self) -> &String;
    fn sort(&self, numbers: &mut Vec<u32>);
}
