use std::ops::Deref;
//use std::fmt::Display;

#[derive(Debug)]
struct Num (u16,u8);

impl Deref for Num {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.1 // only the item is drefferenced will be printed
    
}
}

fn main() {
    let my_numebr = Num(20,4);
    println!("the number is {}",*my_numebr+33);
    println!("the core number is {:?}", my_numebr);
    
    
}
