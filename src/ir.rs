use crate::ast::{CompUnit, Exp, FuncType};
use koopa::ir::builder_traits::*;
use koopa::ir::*;

fn traverse_exp(exp: &Exp, function_data: &mut FunctionData, res: &mut Vec<Value>) -> Value {
    let zero = function_data.dfg_mut().new_value().integer(0);
    match exp {
        Exp::Number(num) => function_data.dfg_mut().new_value().integer(*num),
        Exp::Paren(exp) => traverse_exp(exp, function_data, res),
        Exp::PlusUnary(exp) => traverse_exp(exp, function_data, res),
        Exp::MinusUnary(exp) => {
            let lhs = traverse_exp(exp, function_data, res);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Sub, zero, lhs);
            res.push(r);
            r
        }
        Exp::NotUnary(exp) => {
            let lhs = traverse_exp(exp, function_data, res);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Eq, lhs, zero);
            res.push(r);
            r
        }
        Exp::MulBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res);
            let rhs = traverse_exp(rhs, function_data, res);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Mul, lhs, rhs);
            res.push(r);
            r
        }
        Exp::DivBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res);
            let rhs = traverse_exp(rhs, function_data, res);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Div, lhs, rhs);
            res.push(r);
            r
        }
        Exp::ModBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res);
            let rhs = traverse_exp(rhs, function_data, res);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Mod, lhs, rhs);
            res.push(r);
            r
        }
        Exp::AddBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res);
            let rhs = traverse_exp(rhs, function_data, res);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Add, lhs, rhs);
            res.push(r);
            r
        }
        Exp::SubBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res);
            let rhs = traverse_exp(rhs, function_data, res);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Sub, lhs, rhs);
            res.push(r);
            r
        }
        Exp::LTBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res);
            let rhs = traverse_exp(rhs, function_data, res);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Lt, lhs, rhs);
            res.push(r);
            r
        }
        Exp::LEBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res);
            let rhs = traverse_exp(rhs, function_data, res);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Le, lhs, rhs);
            res.push(r);
            r
        }
        Exp::GTBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res);
            let rhs = traverse_exp(rhs, function_data, res);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Gt, lhs, rhs);
            res.push(r);
            r
        }
        Exp::GEBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res);
            let rhs = traverse_exp(rhs, function_data, res);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Ge, lhs, rhs);
            res.push(r);
            r
        }
        Exp::EqBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res);
            let rhs = traverse_exp(rhs, function_data, res);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::Eq, lhs, rhs);
            res.push(r);
            r
        }
        Exp::NeBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res);
            let rhs = traverse_exp(rhs, function_data, res);
            let r = function_data
                .dfg_mut()
                .new_value()
                .binary(BinaryOp::NotEq, lhs, rhs);
            res.push(r);
            r
        }
        // todo 短路
        Exp::LAndBinary(lhs, rhs) => {
            let lhs = traverse_exp(lhs, function_data, res);
            let rhs = traverse_exp(rhs, function_data, res);
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
            let lhs = traverse_exp(lhs, function_data, res);
            let rhs = traverse_exp(rhs, function_data, res);
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
                _ => todo!(),
            },
        ));
        let main_data = program.func_mut(m);

        let entry = main_data
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%entry".into()));
        main_data.layout_mut().bbs_mut().extend([entry]);

        let exp_tree = &self.func_def.block.stmt.exp;
        let mut res: Vec<Value> = Vec::new();
        let to_return = traverse_exp(exp_tree, main_data, &mut res);
        let ret = main_data.dfg_mut().new_value().ret(Some(to_return));
        res.push(ret);

        main_data.layout_mut().bb_mut(entry).insts_mut().extend(res);
        program
    }
}
