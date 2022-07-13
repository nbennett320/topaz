pub enum Opcode {
    Return = 0,
    Constant,
    Negate,
    Unknown,
}

pub fn from_u8(x: u8) -> Opcode {
    match x {
        0 => Opcode::Return,
        1 => Opcode::Constant,
        2 => Opcode::Negate,
        _ => Opcode::Unknown,
    }
}
