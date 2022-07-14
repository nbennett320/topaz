pub enum Opcode {
    Return = 0,
    Constant,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Nil,
    True,
    False,
    Not,
    Equal,
    Greater,
    Less,

    Unknown,
}

pub fn from_u8(x: u8) -> Opcode {
    match x {
        0 => Opcode::Return,
        1 => Opcode::Constant,
        2 => Opcode::Negate,
        3 => Opcode::Add,
        4 => Opcode::Subtract,
        5 => Opcode::Multiply,
        6 => Opcode::Divide,
        7 => Opcode::Nil,
        8 => Opcode::True,
        9 => Opcode::False,
        10 => Opcode::Not,
        11 => Opcode::Equal,
        12 => Opcode::Greater,
        13 => Opcode::Less,
        _ => Opcode::Unknown,
    }
}
