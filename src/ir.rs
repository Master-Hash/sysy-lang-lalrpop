use std::collections::HashMap;

use crate::ast::{BType, BlockItem, CompUnit, Decl, Exp, FuncType, Stmt};
use koopa::ir::builder_traits::*;
use koopa::ir::*;

fn traverse_const_exp(exp: &Exp, sym_table: &mut HashMap<String, Sym>) -> i32 {
    match exp {
        Exp::Number(num) => *num,
        Exp::Paren(exp) => traverse_const_exp(exp, sym_table),
        Exp::PlusUnary(exp) => traverse_const_exp(exp, sym_table),
        Exp::MinusUnary(exp) => {
            let lhs = traverse_const_exp(exp, sym_table);
            -lhs
        }
        Exp::NotUnary(exp) => {
            let lhs = traverse_const_exp(exp, sym_table);
            if lhs == 0 { 1 } else { 0 }
        }
        Exp::MulBinary(lhs, rhs) => {
            let lhs = traverse_const_exp(lhs, sym_table);
            let rhs = traverse_const_exp(rhs, sym_table);
            lhs * rhs
        }
        Exp::DivBinary(lhs, rhs) => {
            let lhs = traverse_const_exp(lhs, sym_table);
            let rhs = traverse_const_exp(rhs, sym_table);
            lhs / rhs
        }
        Exp::ModBinary(lhs, rhs) => {
            let lhs = traverse_const_exp(lhs, sym_table);
            let rhs = traverse_const_exp(rhs, sym_table);
            lhs % rhs
        }
        Exp::AddBinary(lhs, rhs) => {
            let lhs = traverse_const_exp(lhs, sym_table);
            let rhs = traverse_const_exp(rhs, sym_table);
            lhs + rhs
        }
        Exp::SubBinary(lhs, rhs) => {
            let lhs = traverse_const_exp(lhs, sym_table);
            let rhs = traverse_const_exp(rhs, sym_table);
            lhs - rhs
        }
        Exp::LTBinary(lhs, rhs) => {
            let lhs = traverse_const_exp(lhs, sym_table);
            let rhs = traverse_const_exp(rhs, sym_table);
            if lhs < rhs { 1 } else { 0 }
        }
        Exp::LEBinary(lhs, rhs) => {
            let lhs = traverse_const_exp(lhs, sym_table);
            let rhs = traverse_const_exp(rhs, sym_table);
            if lhs <= rhs { 1 } else { 0 }
        }
        Exp::GTBinary(lhs, rhs) => {
            let lhs = traverse_const_exp(lhs, sym_table);
            let rhs = traverse_const_exp(rhs, sym_table);
            if lhs > rhs { 1 } else { 0 }
        }
        Exp::GEBinary(lhs, rhs) => {
            let lhs = traverse_const_exp(lhs, sym_table);
            let rhs = traverse_const_exp(rhs, sym_table);
            if lhs >= rhs { 1 } else { 0 }
        }
        Exp::EqBinary(lhs, rhs) => {
            let lhs = traverse_const_exp(lhs, sym_table);
            let rhs = traverse_const_exp(rhs, sym_table);
            if lhs == rhs { 1 } else { 0 }
        }
        Exp::NeBinary(lhs, rhs) => {
            let lhs = traverse_const_exp(lhs, sym_table);
            let rhs = traverse_const_exp(rhs, sym_table);
            if lhs != rhs { 1 } else { 0 }
        }
        Exp::LAndBinary(lhs, rhs) => {
            let lhs = traverse_const_exp(lhs, sym_table);
            let rhs = traverse_const_exp(rhs, sym_table);
            if lhs != 0 && rhs != 0 { 1 } else { 0 }
        }
        Exp::LOrBinary(lhs, rhs) => {
            let lhs = traverse_const_exp(lhs, sym_table);
            let rhs = traverse_const_exp(rhs, sym_table);
            if lhs != 0 || rhs != 0 { 1 } else { 0 }
        }
        Exp::LVal(ident) => {
            let sym = sym_table.get(ident).unwrap();
            match sym {
                Sym::Const { value } => *value,
                Sym::Variable { value } => panic!("Can't use variable in const expression"),
            }
        }
    }
}

fn traverse_exp(
    exp: &Exp,
    function_data: &mut FunctionData,
    res: &mut Vec<Value>,
    sym_table: &mut HashMap<String, Sym>,
) -> Value {
    let zero = function_data.dfg_mut().new_value().integer(0);
    match exp {
        Exp::Number(num) => function_data.dfg_mut().new_value().integer(*num),
        Exp::LVal(ident) => {
            let sym = sym_table.get(ident).unwrap();
            match sym {
                Sym::Const { value } => function_data.dfg_mut().new_value().integer(*value),
                Sym::Variable { value } => {
                    let l = function_data.dfg_mut().new_value().load(*value);
                    res.push(l);
                    l
                }
            }
            // function_data.dfg_mut().new_value().integer(sym.value)
        }
        Exp::Paren(exp) => traverse_exp(exp, function_data, res, sym_table),
        Exp::PlusUnary(exp) => traverse_exp(exp, function_data, res, sym_table),
        Exp::MinusUnary(exp) => {
            let lhs = traverse_exp(exp, function_data, res, sym_table);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Sub, zero, lhs);
            res.push(r);
            r
        }
        Exp::NotUnary(exp) => {
            let lhs = traverse_exp(exp, function_data, res, sym_table);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Eq, lhs, zero);
            res.push(r);
            r
        }
        Exp::MulBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res, sym_table);
            let rhs = traverse_exp(rhs, function_data, res, sym_table);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Mul, lhs, rhs);
            res.push(r);
            r
        }
        Exp::DivBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res, sym_table);
            let rhs = traverse_exp(rhs, function_data, res, sym_table);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Div, lhs, rhs);
            res.push(r);
            r
        }
        Exp::ModBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res, sym_table);
            let rhs = traverse_exp(rhs, function_data, res, sym_table);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Mod, lhs, rhs);
            res.push(r);
            r
        }
        Exp::AddBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res, sym_table);
            let rhs = traverse_exp(rhs, function_data, res, sym_table);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Add, lhs, rhs);
            res.push(r);
            r
        }
        Exp::SubBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res, sym_table);
            let rhs = traverse_exp(rhs, function_data, res, sym_table);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Sub, lhs, rhs);
            res.push(r);
            r
        }
        Exp::LTBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res, sym_table);
            let rhs = traverse_exp(rhs, function_data, res, sym_table);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Lt, lhs, rhs);
            res.push(r);
            r
        }
        Exp::LEBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res, sym_table);
            let rhs = traverse_exp(rhs, function_data, res, sym_table);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Le, lhs, rhs);
            res.push(r);
            r
        }
        Exp::GTBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res, sym_table);
            let rhs = traverse_exp(rhs, function_data, res, sym_table);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Gt, lhs, rhs);
            res.push(r);
            r
        }
        Exp::GEBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res, sym_table);
            let rhs = traverse_exp(rhs, function_data, res, sym_table);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Ge, lhs, rhs);
            res.push(r);
            r
        }
        Exp::EqBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res, sym_table);
            let rhs = traverse_exp(rhs, function_data, res, sym_table);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Eq, lhs, rhs);
            res.push(r);
            r
        }
        Exp::NeBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res, sym_table);
            let rhs = traverse_exp(rhs, function_data, res, sym_table);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::NotEq, lhs, rhs);
            res.push(r);
            r
        }
        // todo 短路
        Exp::LAndBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res, sym_table);
            let rhs = traverse_exp(rhs, function_data, res, sym_table);
            let bit_lhs = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::NotEq, lhs, zero);
            let bit_rhs = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::NotEq, rhs, zero);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::And, bit_lhs, bit_rhs);
            res.push(bit_lhs);
            res.push(bit_rhs);
            res.push(r);
            r
        }
        // todo 短路
        Exp::LOrBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res, sym_table);
            let rhs = traverse_exp(rhs, function_data, res, sym_table);
            let bit_or_value = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Or, lhs, rhs);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::NotEq, bit_or_value, zero);
            res.push(bit_or_value);
            res.push(r);
            r
        }
    }
}

// https://docs.rs/koopa/latest/koopa/ir/
impl CompUnit {
    pub fn new_ir(&self) -> Program {
        let mut program = Program::new();
        let m: Function = program.new_func(FunctionData::new(
            format!("@{}", self.func_def.ident),
            vec![],
            match self.func_def.func_type {
                FuncType::Int => Type::get_i32(),
            },
        ));
        let main_data = program.func_mut(m);

        let entry = main_data
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%entry".into()));
        main_data.layout_mut().bbs_mut().extend([entry]);

        let items = &self.func_def.block.items;
        let mut res: Vec<Value> = Vec::new();
        let mut sym_table: HashMap<String, Sym> = HashMap::new();

        for item in items {
            match item {
                BlockItem::Stmt(stmt) => match stmt {
                    Stmt::Return(exp) => {
                        let to_return = traverse_exp(&exp, main_data, &mut res, &mut sym_table);
                        let ret = main_data.dfg_mut().new_value().ret(Some(to_return));
                        res.push(ret);
                    }
                    Stmt::Assign { ident, exp } => {
                        let sym = sym_table.get(ident).unwrap().clone();
                        match sym {
                            Sym::Const { value: _ } => panic!("Can't assign to const"),
                            Sym::Variable { value } => {
                                let v = traverse_exp(exp, main_data, &mut res, &mut sym_table);
                                let s = main_data.dfg_mut().new_value().store(v, value);
                                res.push(s);
                            }
                        }
                    }
                },
                BlockItem::Decl(decl) => match decl {
                    Decl::ConstDecl { b_type, const_def } => match b_type {
                        BType::Int => {
                            for const_def in const_def {
                                let value: i32 =
                                    traverse_const_exp(&const_def.const_exp.0, &mut sym_table);
                                sym_table.insert(const_def.ident.clone(), Sym::Const { value });
                            }
                        }
                    },
                    Decl::VariableDecl { b_type, var_def } => match b_type {
                        BType::Int => {
                            for var_def in var_def {
                                let store = main_data.dfg_mut().new_value().alloc(Type::get_i32());
                                res.push(store);
                                if let Some(exp) = &var_def.exp {
                                    let v = traverse_exp(exp, main_data, &mut res, &mut sym_table);
                                    let s = main_data.dfg_mut().new_value().store(v, store);
                                    res.push(s);
                                }
                                sym_table
                                    .insert(var_def.ident.clone(), Sym::Variable { value: store });
                            }
                        }
                    },
                },
            }
        }

        main_data.layout_mut().bb_mut(entry).insts_mut().extend(res);
        program
    }
}

#[derive(Debug, Clone)]
enum Sym {
    Const { value: i32 },
    Variable { value: Value },
}
