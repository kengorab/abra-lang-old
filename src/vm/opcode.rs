#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    Constant = 0,
    IAdd,
    ISub,
    IMul,
    IDiv,
    FAdd,
    FSub,
    FMul,
    FDiv,
    I2F,
    F2I,
    Invert,
    StrConcat,
    T,
    F,
    And,
    Or,
    Negate,
    LT,
    LTE,
    GT,
    GTE,
    Eq,
    Neq,
    MkArr,
    Return,
}

impl From<u8> for Opcode {
    fn from(i: u8) -> Self {
        match i {
            0 => Opcode::Constant,
            1 => Opcode::IAdd,
            2 => Opcode::ISub,
            3 => Opcode::IMul,
            4 => Opcode::IDiv,
            5 => Opcode::FAdd,
            6 => Opcode::FSub,
            7 => Opcode::FMul,
            8 => Opcode::FDiv,
            9 => Opcode::I2F,
            10 => Opcode::F2I,
            11 => Opcode::Invert,
            12 => Opcode::StrConcat,
            13 => Opcode::T,
            14 => Opcode::F,
            15 => Opcode::And,
            16 => Opcode::Or,
            17 => Opcode::Negate,
            18 => Opcode::LT,
            19 => Opcode::LTE,
            20 => Opcode::GT,
            21 => Opcode::GTE,
            22 => Opcode::Eq,
            23 => Opcode::Neq,
            24 => Opcode::MkArr,
            25 => Opcode::Return,
            _ => unreachable!()
        }
    }
}
