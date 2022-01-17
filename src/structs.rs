use std::marker::PhantomData;

pub struct Color<'a, T: 'a>{
    pub r: i32, pub g: i32, pub b: i32,
    pub rgb_code: String,
    pub dmc: i32,
    pub pd: PhantomData<&'a T>
}

pub struct TempRange<'a>{
    pub min: i32,
    pub max: i32,
    pub one: &'a Color<'a, i32>,
    pub two: &'a Color<'a, i32>,
    pub three: &'a Color<'a, i32>,
    pub four: &'a Color<'a, i32>,
    pub five: &'a Color<'a, i32>
}