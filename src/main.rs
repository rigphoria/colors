use crate::program::{Beep, Program};

mod program;

fn main(){
    let color1 = Color{
        rgb_values: RGB{
            r: 20.0,
            g: 30.0,
            b: 40.0
        },
        rgb_code: String::from("#hf63hn"),
        dmc: 11111
    };
    let color2 = Color{
        rgb_values: RGB{
            r: 50.0,
            g: 60.0,
            b: 70.0
        },
        rgb_code: String::from("#hf512hn"),
        dmc: 22222
    };
    let color3 = Color{
        rgb_values: RGB{
            r: 50.0,
            g: 60.0,
            b: 70.0
        },
        rgb_code: String::from("#hf512hn"),
        dmc: 33333
    };
    let color4 = Color{
        rgb_values: RGB{
            r: 50.0,
            g: 60.0,
            b: 70.0
        },
        rgb_code: String::from("#hf512hn"),
        dmc: 44444
    };
    let color5 = Color{
        rgb_values: RGB{
            r: 50.0,
            g: 60.0,
            b: 70.0
        },
        rgb_code: String::from("#hf512hn"),
        dmc: 55555
    };

    let range = TempRange{
        min: 0.0,
        max: 14.0,
        one: color1,
        two: color2,
        three: color3,
        four: color4,
        five: color5
    };
    let divisor:  f32 = (range.max - range.min) / 5.0;
    println!("{}", divisor);

    let mut p = Program{count: 2000, leave: false};
    loop {
        if p.beep() == true{
            break;
        }
    }
}

struct Color{
    rgb_values: RGB,
    rgb_code: String,
    dmc: i32
}

struct RGB{
    r: f32,
    g: f32,
    b: f32
}

struct TempRange{
    min: f32,
    max: f32,
    one: Color,
    two: Color,
    three: Color,
    four: Color,
    five: Color
}