pub struct Color{
    rgb_values: RGB,
    rgb_code: String,
    dmc: i32
}

pub struct RGB{
    r: f32,
    g: f32,
    b: f32
}

pub struct TempRange{
    min: f32,
    max: f32,
    one: Color,
    two: Color,
    three: Color,
    four: Color,
    five: Color
}