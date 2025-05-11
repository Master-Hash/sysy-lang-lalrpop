use std::collections::HashMap;

use crate::ast::{BType, Block, BlockItem, CompUnit, Decl, Exp, FuncType, Stmt};
use koopa::ir::builder_traits::*;
use koopa::ir::*;

impl Exp {
    // 这玩意，我认为不用改
    pub fn traverse_const_exp(&self, sym_table: &CascadeTable) -> i32 {
        let exp = self;
        match exp {
            Exp::Number(num) => *num,
            Exp::Paren(exp) => exp.traverse_const_exp(sym_table),
            Exp::PlusUnary(exp) => exp.traverse_const_exp(sym_table),
            Exp::MinusUnary(exp) => {
                let lhs = exp.traverse_const_exp(sym_table);
                -lhs
            }
            Exp::NotUnary(exp) => {
                let lhs = exp.traverse_const_exp(sym_table);
                if lhs == 0 { 1 } else { 0 }
            }
            Exp::MulBinary(lhs, rhs) => {
                let lhs = lhs.traverse_const_exp(sym_table);
                let rhs = rhs.traverse_const_exp(sym_table);
                lhs * rhs
            }
            Exp::DivBinary(lhs, rhs) => {
                let lhs = lhs.traverse_const_exp(sym_table);
                let rhs = rhs.traverse_const_exp(sym_table);
                lhs / rhs
            }
            Exp::ModBinary(lhs, rhs) => {
                let lhs = lhs.traverse_const_exp(sym_table);
                let rhs = rhs.traverse_const_exp(sym_table);
                lhs % rhs
            }
            Exp::AddBinary(lhs, rhs) => {
                let lhs = lhs.traverse_const_exp(sym_table);
                let rhs = rhs.traverse_const_exp(sym_table);
                lhs + rhs
            }
            Exp::SubBinary(lhs, rhs) => {
                let lhs = lhs.traverse_const_exp(sym_table);
                let rhs = rhs.traverse_const_exp(sym_table);
                lhs - rhs
            }
            Exp::LTBinary(lhs, rhs) => {
                let lhs = lhs.traverse_const_exp(sym_table);
                let rhs = rhs.traverse_const_exp(sym_table);
                if lhs < rhs { 1 } else { 0 }
            }
            Exp::LEBinary(lhs, rhs) => {
                let lhs = lhs.traverse_const_exp(sym_table);
                let rhs = rhs.traverse_const_exp(sym_table);
                if lhs <= rhs { 1 } else { 0 }
            }
            Exp::GTBinary(lhs, rhs) => {
                let lhs = lhs.traverse_const_exp(sym_table);
                let rhs = rhs.traverse_const_exp(sym_table);
                if lhs > rhs { 1 } else { 0 }
            }
            Exp::GEBinary(lhs, rhs) => {
                let lhs = lhs.traverse_const_exp(sym_table);
                let rhs = rhs.traverse_const_exp(sym_table);
                if lhs >= rhs { 1 } else { 0 }
            }
            Exp::EqBinary(lhs, rhs) => {
                let lhs = lhs.traverse_const_exp(sym_table);
                let rhs = rhs.traverse_const_exp(sym_table);
                if lhs == rhs { 1 } else { 0 }
            }
            Exp::NeBinary(lhs, rhs) => {
                let lhs = lhs.traverse_const_exp(sym_table);
                let rhs = rhs.traverse_const_exp(sym_table);
                if lhs != rhs { 1 } else { 0 }
            }
            Exp::LAndBinary(lhs, rhs) => {
                let lhs = lhs.traverse_const_exp(sym_table);
                let rhs = rhs.traverse_const_exp(sym_table);
                if lhs != 0 && rhs != 0 { 1 } else { 0 }
            }
            Exp::LOrBinary(lhs, rhs) => {
                let lhs = lhs.traverse_const_exp(sym_table);
                let rhs = rhs.traverse_const_exp(sym_table);
                if lhs != 0 || rhs != 0 { 1 } else { 0 }
            }
            Exp::LVal(ident) => {
                let sym = sym_table.get(ident);
                match sym {
                    Sym::Const { value } => *value,
                    Sym::Variable { value: _ } => panic!("Can't use variable in const expression"),
                }
            }
        }
    }

    fn traverse_exp(
        &self,
        // function_data: &mut FunctionData,
        // res: &mut Vec<Value>,
        // sym_table: &CascadeTable,
        context: &mut IRContext,
    ) -> Value {
        let sym_table = &mut context.current_cascade_table;
        let mut res = vec![];
        let exp = self;
        let zero = context
            .current_program
            .func_mut(context.current_func)
            .dfg_mut()
            .new_value()
            .integer(0);
        let v = match exp {
            Exp::Number(num) => context
                .current_program
                .func_mut(context.current_func)
                .dfg_mut()
                .new_value()
                .integer(*num),
            Exp::LVal(ident) => {
                let sym = sym_table.get(ident);
                match sym {
                    Sym::Const { value } => context
                        .current_program
                        .func_mut(context.current_func)
                        .dfg_mut()
                        .new_value()
                        .integer(*value),
                    Sym::Variable { value } => {
                        let l = context
                            .current_program
                            .func_mut(context.current_func)
                            .dfg_mut()
                            .new_value()
                            .load(*value);
                        res.push(l);
                        l
                    }
                }
                // context.current_program.func_mut(context.current_func).dfg_mut().new_value().integer(sym.value)
            }
            Exp::Paren(exp) => exp.traverse_exp(context),
            Exp::PlusUnary(exp) => exp.traverse_exp(context),
            Exp::MinusUnary(exp) => {
                let lhs = exp.traverse_exp(context);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Sub, zero, lhs);
                res.push(r);
                r
            }
            Exp::NotUnary(exp) => {
                let lhs = exp.traverse_exp(context);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Eq, lhs, zero);
                res.push(r);
                r
            }
            Exp::MulBinary(lhs, rhs) => {
                let lhs = lhs.traverse_exp(context);
                let rhs = rhs.traverse_exp(context);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Mul, lhs, rhs);
                res.push(r);
                r
            }
            Exp::DivBinary(lhs, rhs) => {
                let lhs = lhs.traverse_exp(context);
                let rhs = rhs.traverse_exp(context);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Div, lhs, rhs);
                res.push(r);
                r
            }
            Exp::ModBinary(lhs, rhs) => {
                let lhs = lhs.traverse_exp(context);
                let rhs = rhs.traverse_exp(context);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Mod, lhs, rhs);
                res.push(r);
                r
            }
            Exp::AddBinary(lhs, rhs) => {
                let lhs = lhs.traverse_exp(context);
                let rhs = rhs.traverse_exp(context);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Add, lhs, rhs);
                res.push(r);
                r
            }
            Exp::SubBinary(lhs, rhs) => {
                let lhs = lhs.traverse_exp(context);
                let rhs = rhs.traverse_exp(context);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Sub, lhs, rhs);
                res.push(r);
                r
            }
            Exp::LTBinary(lhs, rhs) => {
                let lhs = lhs.traverse_exp(context);
                let rhs = rhs.traverse_exp(context);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Lt, lhs, rhs);
                res.push(r);
                r
            }
            Exp::LEBinary(lhs, rhs) => {
                let lhs = lhs.traverse_exp(context);
                let rhs = rhs.traverse_exp(context);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Le, lhs, rhs);
                res.push(r);
                r
            }
            Exp::GTBinary(lhs, rhs) => {
                let lhs = lhs.traverse_exp(context);
                let rhs = rhs.traverse_exp(context);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Gt, lhs, rhs);
                res.push(r);
                r
            }
            Exp::GEBinary(lhs, rhs) => {
                let lhs = lhs.traverse_exp(context);
                let rhs = rhs.traverse_exp(context);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Ge, lhs, rhs);
                res.push(r);
                r
            }
            Exp::EqBinary(lhs, rhs) => {
                let lhs = lhs.traverse_exp(context);
                let rhs = rhs.traverse_exp(context);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Eq, lhs, rhs);
                res.push(r);
                r
            }
            Exp::NeBinary(lhs, rhs) => {
                let lhs = lhs.traverse_exp(context);
                let rhs = rhs.traverse_exp(context);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::NotEq, lhs, rhs);
                res.push(r);
                r
            }
            // todo 短路
            Exp::LAndBinary(lhs, rhs) => {
                let lhs = lhs.traverse_exp(context);
                let rhs = rhs.traverse_exp(context);
                let bit_lhs = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::NotEq, lhs, zero);
                let bit_rhs = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::NotEq, rhs, zero);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
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
                let lhs = lhs.traverse_exp(context);
                let rhs = rhs.traverse_exp(context);
                let bit_or_value = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::Or, lhs, rhs);
                let r = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .binary(BinaryOp::NotEq, bit_or_value, zero);
                res.push(bit_or_value);
                res.push(r);
                r
            }
        };
        context
            .current_program
            .func_mut(context.current_func)
            .layout_mut()
            .bb_mut(context.current_bblock)
            .insts_mut()
            .extend(res);
        v
    }
}

impl Stmt {
    fn new_ir(&self, context: &mut IRContext) -> IsReturn {
        // let main_data = function_data;
        match self {
            Stmt::Return(exp) => {
                let mut res: Vec<Value> = vec![];
                if let Some(exp) = exp {
                    let to_return = exp.traverse_exp(context);
                    let ret = context
                        .current_program
                        .func_mut(context.current_func)
                        .dfg_mut()
                        .new_value()
                        .ret(Some(to_return));
                    res.push(ret);
                } else {
                    let ret = context
                        .current_program
                        .func_mut(context.current_func)
                        .dfg_mut()
                        .new_value()
                        .ret(None);
                    res.push(ret);
                }
                context
                    .current_program
                    .func_mut(context.current_func)
                    .layout_mut()
                    .bb_mut(context.current_bblock)
                    .insts_mut()
                    .extend(res);
                return IsReturn::Yes;
            }
            Stmt::Assign { ident, exp } => {
                // rust 没有什么好办法标记可变借用的结构体部分字段不可变
                // 无奈
                let mut res: Vec<Value> = vec![];
                let sym = context.current_cascade_table.get(ident).clone();
                match sym {
                    Sym::Const { value: _ } => panic!("Can't assign to const"),
                    Sym::Variable { value } => {
                        let v = exp.traverse_exp(context);
                        let s = context
                            .current_program
                            .func_mut(context.current_func)
                            .dfg_mut()
                            .new_value()
                            .store(v, value);
                        res.push(s);
                    }
                }
                context
                    .current_program
                    .func_mut(context.current_func)
                    .layout_mut()
                    .bb_mut(context.current_bblock)
                    .insts_mut()
                    .extend(res);
            }
            Stmt::Exp(_exp) => {
                if let Some(exp) = _exp {
                    let _v = exp.traverse_exp(context);
                }
            }
            Stmt::Block(block) => {
                return block.new_ir(context);
            }
            Stmt::If {
                cond,
                then_stmt,
                else_stmt,
            } => {
                let cond = cond.traverse_exp(context);
                let then_bb = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_bb()
                    .basic_block(None);
                let else_bb = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_bb()
                    .basic_block(None);
                let end_bb = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_bb()
                    .basic_block(None);

                context
                    .current_program
                    .func_mut(context.current_func)
                    .layout_mut()
                    .bbs_mut()
                    .extend([then_bb, else_bb, end_bb]);

                let cond_inst = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .branch(cond, then_bb, else_bb);
                let jump_end_inst = context
                    .current_program
                    .func_mut(context.current_func)
                    .dfg_mut()
                    .new_value()
                    .jump(end_bb);

                let _ = context
                    .current_program
                    .func_mut(context.current_func)
                    .layout_mut()
                    .bb_mut(context.current_bblock)
                    .insts_mut()
                    .push_key_back(cond_inst);

                context.current_bblock = then_bb;
                let t = then_stmt.new_ir(context);
                if let IsReturn::No = t {
                    let _ = context
                        .current_program
                        .func_mut(context.current_func)
                        .layout_mut()
                        .bb_mut(context.current_bblock)
                        .insts_mut()
                        .push_key_back(jump_end_inst);
                }

                context.current_bblock = else_bb;
                if let Some(e) = else_stmt {
                    let e = e.new_ir(context);
                    if let IsReturn::No = e {
                        let _ = context
                            .current_program
                            .func_mut(context.current_func)
                            .layout_mut()
                            .bb_mut(context.current_bblock)
                            .insts_mut()
                            .push_key_back(jump_end_inst);
                    }
                }

                context.current_bblock = end_bb;
            }
        }
        IsReturn::No
        // res
    }
}

impl Block {
    pub fn new_ir(
        &self,
        // function_data: &mut FunctionData,
        // cascade_table: &mut CascadeTable,
        context: &mut IRContext,
    ) -> IsReturn {
        context.current_cascade_table.indent();

        for item in &self.items {
            match item {
                BlockItem::Stmt(stmt) => {
                    let x = stmt.new_ir(context);
                    if let IsReturn::Yes = x {
                        context.current_cascade_table.pop();
                        return x;
                    }
                }
                BlockItem::Decl(decl) => {
                    match decl {
                        Decl::ConstDecl { b_type, const_def } => match b_type {
                            BType::Int => {
                                for const_def in const_def {
                                    let value: i32 = const_def
                                        .const_exp
                                        .0
                                        .traverse_const_exp(&context.current_cascade_table);
                                    context
                                        .current_cascade_table
                                        .insert(const_def.ident.clone(), Sym::Const { value });
                                }
                            }
                        },
                        Decl::VariableDecl { b_type, var_def } => match b_type {
                            BType::Int => {
                                for var_def in var_def {
                                    // let sym_table = cascade_table.0.last_mut().unwrap();
                                    let allocation = context
                                        .current_program
                                        .func_mut(context.current_func)
                                        .dfg_mut()
                                        .new_value()
                                        .alloc(Type::get_i32());
                                    // res.push(allocation);
                                    let _ = context
                                        .current_program
                                        .func_mut(context.current_func)
                                        .layout_mut()
                                        .bb_mut(context.current_bblock)
                                        .insts_mut()
                                        .push_key_back(allocation);
                                    if let Some(exp) = &var_def.exp {
                                        let v = exp.traverse_exp(context);
                                        let s = context
                                            .current_program
                                            .func_mut(context.current_func)
                                            .dfg_mut()
                                            .new_value()
                                            .store(v, allocation);
                                        let _ = context
                                            .current_program
                                            .func_mut(context.current_func)
                                            .layout_mut()
                                            .bb_mut(context.current_bblock)
                                            .insts_mut()
                                            .push_key_back(s);
                                    }
                                    context.current_cascade_table.insert(
                                        var_def.ident.clone(),
                                        Sym::Variable { value: allocation },
                                    );
                                }
                            }
                        },
                    }
                }
            }
        }
        // todo!();
        context.current_cascade_table.pop();
        IsReturn::No
    }
}

// #[derive(Debug)]
pub struct IRContext {
    pub current_program: Program,
    pub current_func: Function,
    pub current_bblock: BasicBlock,
    pub current_cascade_table: CascadeTable,
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

        let block = &self.func_def.block;
        let cadcade_table = CascadeTable::new();

        let mut context = IRContext {
            current_program: program,
            current_func: m,
            current_bblock: entry,
            current_cascade_table: cadcade_table,
        };
        block.new_ir(&mut context);

        // main_data.layout_mut().bb_mut(entry).insts_mut().extend(res);
        context.current_program
    }
}

#[derive(Debug, Clone)]
enum Sym {
    Const { value: i32 },
    Variable { value: Value },
}

pub enum IsReturn {
    Yes,
    No,
}

pub struct CascadeTable(Vec<HashMap<String, Sym>>);

impl CascadeTable {
    fn new() -> Self {
        Self(vec![HashMap::new()])
    }

    fn insert(&mut self, key: String, value: Sym) {
        self.0.last_mut().unwrap().insert(key, value);
    }
    fn get(&self, key: &str) -> &Sym {
        for table in self.0.iter().rev() {
            if let Some(value) = table.get(key) {
                return value;
            }
        }
        panic!("Key not found: {}", key);
    }
    fn indent(&mut self) {
        self.0.push(HashMap::new());
    }
    fn pop(&mut self) {
        self.0.pop();
    }
}
