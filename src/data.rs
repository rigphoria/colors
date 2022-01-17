use std::collections::HashMap;
use std::hash::Hash;

use crate::structs::*;

pub struct ColorData<'a>{
    pub color_map: HashMap<i32, Color<'a, i32>>,
    pub range_map: HashMap<i32, TempRange<'a>>
}

impl<'a> ColorData<'a>{
    pub fn init(&'a mut self){
        create_range(&mut self.color_map, &mut self.range_map,
                    0, 10, 1000, 1001, 1002,
                    1003, 1004);
    }


}

pub fn create_range<'a>(color_map: &'a mut HashMap<i32, Color<'a, i32>>,
                    range_map: &'a mut HashMap<i32, TempRange<'a>>,
                    min: i32, max: i32, dmc1: i32, dmc2: i32,
                    dmc3: i32, dmc4: i32, dmc5: i32) -> TempRange<'a>{
    let mut range;
    if let Some(one) = color_map.get(&dmc1){
        if let Some(two) = color_map.get(&dmc2){
            if let Some(three) = color_map.get(&dmc3){
                if let Some(four) = color_map.get(&dmc4){
                    if let Some(five) = color_map.get(&dmc5){
                        range = TempRange{
                            min, max, one, two, three, four, five
                        };
                        range_map.insert(1, range);
                    }
                }
            }
        }
    }
    panic!("couldn't create temp range");
}