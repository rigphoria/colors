use std::collections::HashMap;
//use std::marker::PhantomData;
//use crate::program::{Beep, Program};
//use crate::datafactory::*;
use crate::data::*;

mod data;
mod structs;
mod dmcdata;

fn main(){
    // let mut fb = FooBar{
    //     color_map: HashMap::new(),
    //     map: HashMap::new()
    // };
    // fb.init();

    let mut data = ColorData{
        color_map: HashMap::new(),
        range_map: HashMap::new()
    };
    data.init();
}