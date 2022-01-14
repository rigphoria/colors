pub struct Program{
    pub count: i32,
    pub leave: bool
}

pub trait Beep{
    fn beep(&mut self) -> bool;
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