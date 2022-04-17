use std::ffi::CString;
use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue};
use abra_core::common::typed_ast_visitor::TypedAstVisitor;
use abra_core::lexer::tokens::Token;
use abra_core::parser::ast::UnaryOp;
use abra_core::typechecker::typechecker::TypedModule;
use abra_core::typechecker::typed_ast::{TypedAccessorNode, TypedArrayNode, TypedAssignmentNode, TypedBinaryNode, TypedBindingDeclNode, TypedEnumDeclNode, TypedForLoopNode, TypedFunctionDeclNode, TypedGroupedNode, TypedIdentifierNode, TypedIfNode, TypedImportNode, TypedIndexingNode, TypedInstantiationNode, TypedInvocationNode, TypedLambdaNode, TypedLiteralNode, TypedMapNode, TypedMatchNode, TypedReturnNode, TypedSetNode, TypedTupleNode, TypedTypeDeclNode, TypedUnaryNode, TypedWhileLoopNode};
use abra_core::typechecker::types::Type;

#[derive(Debug)]
pub enum CompilerError {}

pub struct Compiler<'a, 'ctx> {
    context: &'ctx Context,
    _builder: &'a Builder<'ctx>,
    _module: &'a Module<'ctx>,
    _cur_fn: FunctionValue<'ctx>,
}

#[cfg(not(test))]
pub type ModEntryFn = unsafe extern "C" fn() -> ();
#[cfg(test)]
pub type ModEntryFn = unsafe extern "C" fn() -> *const cty::c_char;

pub const ENTRY_FN_NAME: &str = "__mod_entry";

#[no_mangle]
pub extern "C" fn int_to_str(int: cty::int64_t) -> *const cty::c_char {
    CString::new(int.to_string()).unwrap().into_raw()
}
#[used]
static INT_TO_STR: [extern fn(cty::int64_t) -> *const cty::c_char; 1] = [int_to_str];

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    // If `test_mode` is true, the entrypoint function will return the last value in the module as
    // a string, rather than printing that string to stdout.
    pub fn compile_module(context: &'ctx Context, typed_module: TypedModule, test_mode: bool) -> Result<Module<'ctx>, CompilerError> {
        let builder = context.create_builder();
        let module = context.create_module("__main");

        let printf_type = context.i64_type().fn_type(&[context.i8_type().ptr_type(AddressSpace::Generic).into()], false);
        module.add_function("printf", printf_type, None);

        let entry_fn_type = if test_mode {
            context.i8_type().ptr_type(AddressSpace::Generic).fn_type(&[], false)
        } else {
            context.void_type().fn_type(&[], false)
        };
        let entry_fn = module.add_function( ENTRY_FN_NAME, entry_fn_type, None );
        let entry_fn_bb = context.append_basic_block(entry_fn, "entry_fn_bb");
        builder.position_at_end(entry_fn_bb);

        let mut compiler = Compiler {
            context: &context,
            _builder: &builder,
            _module: &module,
            _cur_fn: entry_fn,
        };

        let mut last_item = context.i64_type().const_int(0, false).as_basic_value_enum();
        for node in typed_module.typed_nodes {
            last_item = compiler.visit(node)?;
        }

        let ret_val = if test_mode {
            let int_to_str_type = context.i8_type().ptr_type(AddressSpace::Generic).fn_type(&[context.i64_type().into()], false);
            module.add_function("int_to_str", int_to_str_type, None);

            let int_to_str_fn = module.get_function("int_to_str").unwrap();
            builder.build_call(int_to_str_fn, &[last_item.into()], "").try_as_basic_value().left().unwrap()
        } else {
            let printf = module.get_function("printf").unwrap();
            builder.build_call(printf, &[builder.build_global_string_ptr("%lld\n", "fmt").as_basic_value_enum().into(), last_item.into()], "");
            context.i8_type().const_int(0, false).into()
        };

        builder.build_return(Some(&ret_val));

        Ok(module)
    }
}

impl<'a, 'ctx> TypedAstVisitor<BasicValueEnum<'ctx>, CompilerError> for Compiler<'a, 'ctx> {
    fn visit_literal(&mut self, _token: Token, node: TypedLiteralNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let value = match node {
            TypedLiteralNode::IntLiteral(v) => self.context.i64_type().const_int(v as u64, false).into(),
            TypedLiteralNode::FloatLiteral(_) |
            TypedLiteralNode::StringLiteral(_) |
            TypedLiteralNode::BoolLiteral(_) => todo!()
        };

        Ok(value)
    }

    fn visit_unary(&mut self, _token: Token, node: TypedUnaryNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        let value = match node.op {
            UnaryOp::Minus => {
                let is_int = if node.expr.get_type() == Type::Int { true } else { false };
                let value = self.visit(*node.expr)?;
                if is_int {
                    self._builder.build_int_neg(value.into_int_value(), "").into()
                } else {
                    todo!()
                }
            }
            UnaryOp::Negate => todo!()
        };

        Ok(value)
    }

    fn visit_binary(&mut self, _token: Token, _node: TypedBinaryNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_grouped(&mut self, _token: Token, _node: TypedGroupedNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_array(&mut self, _token: Token, _node: TypedArrayNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_tuple(&mut self, _token: Token, _node: TypedTupleNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_map(&mut self, _token: Token, _node: TypedMapNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_set(&mut self, _token: Token, _node: TypedSetNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_lambda(&mut self, _token: Token, _node: TypedLambdaNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_binding_decl(&mut self, _token: Token, _node: TypedBindingDeclNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_function_decl(&mut self, _token: Token, _node: TypedFunctionDeclNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_type_decl(&mut self, _token: Token, _node: TypedTypeDeclNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_enum_decl(&mut self, _token: Token, _node: TypedEnumDeclNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_identifier(&mut self, _token: Token, _node: TypedIdentifierNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_assignment(&mut self, _token: Token, _node: TypedAssignmentNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_indexing(&mut self, _token: Token, _node: TypedIndexingNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_if_statement(&mut self, _is_stmt: bool, _token: Token, _node: TypedIfNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_if_expression(&mut self, _token: Token, _node: TypedIfNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_invocation(&mut self, _token: Token, _node: TypedInvocationNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_instantiation(&mut self, _token: Token, _node: TypedInstantiationNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_accessor(&mut self, _token: Token, _node: TypedAccessorNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_for_loop(&mut self, _token: Token, _node: TypedForLoopNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_while_loop(&mut self, _token: Token, _node: TypedWhileLoopNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_break(&mut self, _token: Token) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_continue(&mut self, _token: Token) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_return(&mut self, _token: Token, _node: TypedReturnNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_match_statement(&mut self, _is_stmt: bool, _token: Token, _node: TypedMatchNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_match_expression(&mut self, _token: Token, _node: TypedMatchNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_import_statement(&mut self, _token: Token, _node: TypedImportNode) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }

    fn visit_nil(&mut self, _token: Token) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        todo!()
    }
}
