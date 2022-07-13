pub enum Opcode {
    Return = 0,
    Constant,
    Unknown,
}

pub fn from_u8(x: u8) -> Opcode {
    match x {
        0 => Opcode::Return,
        1 => Opcode::Constant,
        _ => Opcode::Unknown,
    }
}
