use crate::structs::{Color};

pub struct Program{
    pub count: i32,
    pub leave: bool
}

pub trait Beep{
    fn beep(&mut self) -> bool;
}

pub trait Init{
    fn init(&mut self) -> bool;
}

pub trait CreateColor{
    fn create_color(&mut self) -> Color;
}

impl Beep for Program{
    fn beep(&mut self) -> bool {
        println!("{}", "hai");
        self.count -= 1;
        if self.count > 0{
            return false;
        }
        return true;
    }
}

impl Init for Program{
    fn init(&mut self) -> bool{

        return false;
    }
}