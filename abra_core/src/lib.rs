extern crate abra_native;
extern crate itertools;
extern crate permutate;
extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::vm::compiler::{Metadata, Module};
use crate::typechecker::typed_ast::TypedAstNode;
use crate::common::display_error::DisplayError;

pub mod builtins;
pub mod common;
pub mod lexer;
pub mod parser;
pub mod typechecker;
pub mod vm;

pub enum Error {
    LexerError(lexer::lexer_error::LexerError),
    ParseError(parser::parse_error::ParseError),
    TypecheckerError(typechecker::typechecker_error::TypecheckerError),
    InterpretError(vm::vm::InterpretError),
}

impl DisplayError for Error {
    fn message_for_error(&self, lines: &Vec<&str>) -> String {
        match self {
            Error::LexerError(e) => e.message_for_error(lines),
            Error::ParseError(e) => e.message_for_error(lines),
            Error::TypecheckerError(e) => e.message_for_error(lines),
            Error::InterpretError(_) => "Runtime error!".to_string()
        }
    }
}

pub fn typecheck(input: &String) -> Result<Vec<TypedAstNode>, Error> {
    match lexer::lexer::tokenize(input) {
        Err(e) => Err(Error::LexerError(e)),
        Ok(tokens) => match parser::parser::parse(tokens) {
            Err(e) => Err(Error::ParseError(e)),
            Ok(ast) => {
                match typechecker::typechecker::typecheck(ast) {
                    Err(e) => Err(Error::TypecheckerError(e)),
                    Ok((_, nodes)) => Ok(nodes)
                }
            }
        }
    }
}

pub fn compile(module_path: String, input: &String) -> Result<(Module, Metadata), Error> {
    let typed_ast_nodes = typecheck(input)?;
    let result = vm::compiler::compile(module_path, typed_ast_nodes).unwrap();
    Ok(result)
}

pub fn compile_and_disassemble(module_path: String, input: &String) -> Result<String, Error> {
    let (compiled_module, metadata) = compile(module_path, input)?;
    Ok(vm::disassembler::disassemble(compiled_module, metadata))
}
