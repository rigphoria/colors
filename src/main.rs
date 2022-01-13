fn main() {
    let color1 = Color{
        rgb_values: Vec3{
            r: 20,
            g: 30,
            b: 40
        },
        rgb_code: String::from("#hf63hn"),
        dmc: 1156
    };
    println!("{}", color1.dmc);
}

struct Color{
    rgb_values: Vec3,
    rgb_code: String,
    dmc: i32
}

struct Vec3{
    r: i32,
    g: i32,
    b: i32
}