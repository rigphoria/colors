//use core::panicking::panic;
use crate::structs::*;
use std::collections::HashMap;

pub struct ColorFactory<'a>{
    pub dummy: &'a i32,
    pub colorMap: HashMap<i32, Color>,
}

impl ColorFactory{
    pub fn init<'a>(mut self){
        create_color(&mut self.colorMap, 1000, String::from("1 blue"), 1, 2, 3);
        create_color(&mut self.colorMap, 1001, String::from("2 blue"), 1, 2, 3);
        create_color(&mut self.colorMap, 1002, String::from("3 blue"), 1, 2, 3);
        create_color(&mut self.colorMap, 1003, String::from("4 blue"), 1, 2, 3);
        create_color(&mut self.colorMap, 1004, String::from("5 blue"), 1, 2, 3);
        println!("what");
    }
}

fn create_color(_map: &mut HashMap<i32, Color>, _dmc: i32, _hex: String, _r: i32, _g: i32, _b: i32){
    if !_map.contains_key(&_dmc){
        let mut color = Color{
            rgb_values: RGB{
                r: _r,
                g: _g,
                b: _b
            },
            rgb_code: _hex,
            dmc: _dmc
        };
        _map.insert(_dmc, color);
    } else {
        panic!("tried to add multiple versions of the same dmc color");
    }
}

pub struct RangeFactory<'a>{
    pub colors: ColorFactory<'a>,
    pub rangeMap: HashMap<i32, TempRange<'a>>
}

impl<'a> RangeFactory{
    pub fn init(&mut self){
        self.colors.init();

        //create_range(&mut self.colors, &mut self.rangeMap, 0, 15, 1000, 1001, 1002, 1003, 1004);
    }
}
impl RangeFactory {
    fn create_range<'a>(_colors: &mut ColorFactory, _ranges: &mut HashMap<i32, TempRange>, _min: i32, _max: i32, _dmc1: i32, _dmc2: i32, _dmc3: i32, _dmc4: i32, _dmc5: i32) {
        if _colors.colorMap.contains_key(&_dmc1) &&
            _colors.colorMap.contains_key(&_dmc2) &&
            _colors.colorMap.contains_key(&_dmc3) &&
            _colors.colorMap.contains_key(&_dmc4) &&
            _colors.colorMap.contains_key(&_dmc5) {
            if let Some(c1) = _colors.colorMap.get(&_dmc1) {
                if let Some(c2) = _colors.colorMap.get(&_dmc2) {
                    if let Some(c3) = _colors.colorMap.get(&_dmc3) {
                        if let Some(c4) = _colors.colorMap.get(&_dmc4) {
                            if let Some(c5) = _colors.colorMap.get(&_dmc5) {
                                let range = TempRange {
                                    min: _min,
                                    max: _max,
                                    one: c1,
                                    two: c2,
                                    three: c3,
                                    four: c4,
                                    five: c5
                                };

                                _ranges.insert(1, range);
                            }
                        }
                    }
                }
            }
        } else {
            panic!("we tried to create a range with colors that don't exist");
        }
    }
}


// impl CreateColor for ColorFactory{
//     fn create_color(&mut self, _dmc: i32, hex: String, _r: i32, _g: i32, _b: i32){
//         if !self.map.contains_key(dmc){
//             let mut color = Color{
//                 rgb_values: RGB{
//                     r: _r,
//                     g: _g,
//                     b: _b
//                 },
//                 rgb_code: hex,
//                 dmc: _dmc
//             };
//
//             map.insert(_dmc, color);
//         } else {
//             //panic("tried to add multiple versions of the same dmc color");
//         }
//     }
// }

// pub trait AllInOne{
//     fn create_range(&mut self, _min: i32, _max: i32,
//         _oneDMC: i32, _oneHex: String, _oneR: i32, _oneG: i32, _oneB: i32,
//         _twoDMC: i32, _twoHex: String, _twoR: i32, _twoG: i32, _twoB: i32,
//         _threeDMC: i32, _threeHex: String, _threeR: i32, _threeG: i32, _threeB: i32,) -> TempRange;
// }

// fn create_range(_min: i32, _max: i32,
//     _oneDMC: i32, _oneHex: String, _oneR: i32, _oneG: i32, _oneB: i32,
//     _twoDMC: i32, _twoHex: String, _twoR: i32, _twoG: i32, _twoB: i32,
//     _threeDMC: i32, _threeHex: String, _threeR: i32, _threeG: i32, _threeB: i32) -> TempRange {
//     let mut c1 = Color{
//         dmc: _oneDMC,
//         rgb_code: _oneHex,
//         rgb_values: RGB{
//             r: _oneR, g: _oneG, b: _oneG
//         }
//     };
//     let mut c2 = Color{
//         dmc: _twoDMC,
//         rgb_code: _twoHex,
//         rgb_values: RGB{
//             r: _twoR, g: _twoG, b: _twoG
//         }
//     };
//     let mut c3 = Color{
//         dmc: _oneDMC,
//         rgb_code: _threeHex,
//         rgb_values: RGB{
//             r: _threeR, g: _threeG, b: _threeG
//         }
//     };
//
//     let mut range = TempRange{
//         min: _min, max: _max, one: c1, two: c2, three: c3
//     };
//
//     return range;
// }

// impl ColorFactory{
//     fn init(&mut self){
//         self.map.insert(1, TempRange{
//             create_range(1, "adhgasid", 1, 1, 1,
//                 1, "adhgasid", 1, 1, 1,
//                 1, "adhgasid", 1, 1, 1);
//         });
//     }
// }

// impl ColorFactory{
//     fn init() {
//         map.insert(1, TempRange{
//             create_range();
//         });
//     }
// }

// pub trait CreateTempRange{
//     fn create_range(&mut self, _min: i32, _max: i32, one: i32, two: i32, three: i32,
//         four: i32, five: i32) -> TempRange;
// }
//
// impl CreateTempRange for ColorFactory{
//     fn create_range(&mut self, _min: i32, _max: i32, one: i32, two: i32, three: i32, four: i32, five: i32) -> TempRange {
//         let c1 = self.map.get(&one).unwrap();
//         let c2 = self.map.get(&two).unwrap();
//         let c3 = self.map.get(&three).unwrap();
//         let c4 = self.map.get(&four).unwrap();
//         let c5 = self.map.get(&five).unwrap();
//
//
//         let mut range = TempRange{
//             min: _min,
//             max: _max,
//             one: c1,
//             two: c2.unwrap(),
//             three: c3.unwrap(),
//             four: c4.unwrap(),
//             five: c5.unwrap()
//         };
//
//         // if range.one ==  || range.two == null || range.three == null || range.four == null || range.five == null{
//         //     //panic("we tried to create a temprange with a dmc color that isn't in the hashmap");
//         // } //todo: check and make sure the dmc colors are actually available
//
//         return range;
//     }
// }