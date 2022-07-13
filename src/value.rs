#[derive(Copy, Clone)]
pub enum Value {
    Number(f64),
}

pub fn print(value: &Value) {
    match value {
        Value::Number(num) => print!("{}", num),
    }
}
