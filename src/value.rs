#[derive(Copy, Clone)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
}

impl Value {
    pub fn print(&self) {
        match self {
            Value::Bool(b) => print!("{}", b),
            Value::Nil => print!("nil"),
            Value::Number(num) => print!("{}", num),
        }
    }
}
