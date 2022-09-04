use crate::instruction::Opcode;
pub mod opcode_parsers;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
}
