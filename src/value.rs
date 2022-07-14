#[derive(Copy, Clone)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
}

pub fn print(value: &Value) {
    match value {
        Value::Bool(b) => print!("{}", b),
        Value::Nil => print!("nil"),
        Value::Number(num) => print!("{}", num),
    }
}
