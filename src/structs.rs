pub struct Color{
    pub rgb_values: RGB,
    pub rgb_code: String,
    pub dmc: i32
}

pub struct RGB{
    pub r: i32,
    pub g: i32,
    pub b: i32
}

pub struct TempRange<'a>{
    pub min: i32,
    pub max: i32,
    pub one: &'a Color,
    pub two: &'a Color,
    pub three: &'a Color,
    pub four: &'a Color,
    pub five: &'a Color
}