use std::collections::HashMap;
//use std::marker::PhantomData;
//use crate::program::{Beep, Program};
//use crate::datafactory::*;

//mod program;
//mod structs;
//mod datafactory;

fn main(){
    let mut fb = FooBar{
        color_map: HashMap::new(),
        map: HashMap::new()
    };
    fb.init();
}

pub struct Color{
    r: i32,
    g: i32,
    b: i32
}

pub struct FooBar<'a>{
    pub color_map: HashMap<i32, Color>,
    pub map: HashMap<i32, &'a Color>
}

impl<'a> FooBar<'a>{
    pub fn init(&'a mut self) {
        self.color_map.insert(1, Color{r: 1, g: 2, b: 3});

        if let Some(val) = self.color_map.get(&1){
            self.map.insert(1, val);
        }
    }
}