use std::collections::HashMap;
use csv::Error;
use std::marker::PhantomData;

use crate::structs::*;
use crate::dmcdata::get_data;

pub struct ColorData<'a>{
    pub color_map: HashMap<i32, Color<'a, i32>>,
    pub range_map: HashMap<i32, TempRange<'a>>
}

impl<'a> ColorData<'a>{
    pub fn init(&'a mut self){
        populate_color_data(&mut self.color_map);
    }
}

pub fn populate_color_data(map: &mut HashMap<i32, Color<i32>>) -> Result<(), Error>{
    let csv = get_data();
    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for result in reader.deserialize(){
        let color: ColorTemp = result?;

        let final_color: Color<i32> = Color{
            r: color.r,
            g: color.g,
            b: color.b,
            rgb_code: color.rgb_code,
            name: color.name,
            dmc: color.dmc,
            pd: PhantomData
        };
        //println!("{}", finalColor.name);

        map.insert(final_color.dmc, final_color);
    }

    // println!("{}", "ok");
    // for result in reader.records() {
    //
    //     //let mut record = result?;
    //     for result in reader.deserialize(){
    //
    //     }
    //
    //     // println!("{},{},{},{}.{},{},{}",
    //     //          &record[0], &record[1], &record[2], &record[3],
    //     //          &record[4], &record[5], &record[6]);
    //     // let dmcstr: &String = &record[0].parse().unwrap();
    //     // let dmc: i32 = dmcstr.parse().unwrap();
    //     // let rstr: &String = &record[1].parse().unwrap();
    //     // let r: i32 = rstr.parse().unwrap();
    //     // let gstr: &String = &record[2].parse().unwrap();
    //     // let g: i32 = gstr.parse().unwrap();
    //     // let bstr: &String = &record[3].parse().unwrap();
    //     // let b: i32 = bstr.parse().unwrap();
    //     // let rgbstr: &String = &record[4].parse().unwrap();
    //     // let rgb_code: String = rgbstr.parse().unwrap();
    //     // let namestr: &String = &record[5].parse().unwrap();
    //     // let name: String = namestr.parse().unwrap();
    //     //
    //     // let color = Color{
    //     //     dmc, r, g, b, rgb_code, name, pd: PhantomData
    //     // };
    //     // println!("{}", color.name);
    //     //map.insert(dmc, color);
    // }
    Ok(())
}

// pub fn add_all_dmc<'a>() -> Result<(), Box<dyn Error>>{
//     let mut reader = csv::Reader::from_reader(io::stdin());
//     for result in reader.records(){
//         let record = result?;
//         println!("{:?}", record);
//     }
//     Ok(());
// }

// pub fn add_color<'a>(color_map: &mut HashMap<i32, Color<i32>>,
//                  r: i32, g: i32, b: i32, rgb_code: String, dmc: i32){
//     if !color_map.contains_key(&dmc) {
//         let color = Color {
//             r, g, b, rgb_code, dmc, pd: PhantomData
//         };
//
//         color_map.insert(dmc, color);
//     } else {
//         panic!("trying to add the dmc color {} twice", dmc);
//     }
// }

pub fn create_range<'a>(color_map: &'a mut HashMap<i32, Color<'a, i32>>,
                    range_map: &'a mut HashMap<i32, TempRange<'a>>,
                    min: i32, max: i32, dmc1: i32, dmc2: i32,
                    dmc3: i32, dmc4: i32, dmc5: i32) -> TempRange<'a>{
    let range;
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