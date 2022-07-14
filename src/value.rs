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

    pub fn is_falsey(self) -> bool {
        match self {
            Value::Nil => true,
            Value::Bool(b) => !b,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_is_not_falsey() {
        assert_eq!(Value::Bool(true).is_falsey(), false);
    }

    #[test]
    fn false_is_falsey() {
        assert_eq!(Value::Bool(false).is_falsey(), true);
    }

    #[test]
    fn nil_is_falsey() {
        assert_eq!(Value::Nil.is_falsey(), true);
    }

    #[test]
    fn numbers_are_not_falsey() {
        assert_eq!(Value::Number(3.14).is_falsey(), false);
    }
}
