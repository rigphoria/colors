use crate::program::{Beep, Program};

mod program;
mod structs;

fn main(){
    let mut p = Program{count: 2000, leave: false};
    loop {
        if p.beep() == true{
            break;
        }
    }
}