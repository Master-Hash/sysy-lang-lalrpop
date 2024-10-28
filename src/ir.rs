use crate::ast::{CompUnit, FuncType};
use koopa::ir::builder_traits::*;
use koopa::ir::*;

impl CompUnit {
    pub fn new_ir(&self) -> Program {
        let mut program = Program::new();
        let m = program.new_func(FunctionData::new(
            format!("@{}", self.func_def.ident),
            vec![],
            match self.func_def.func_type {
                FuncType::Int => Type::get_i32(),
                _ => panic!(),
            },
        ));
        let main_data = program.func_mut(m);

        let entry = main_data
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%entry".into()));
        main_data.layout_mut().bbs_mut().extend([entry]);

        let v = main_data
            .dfg_mut()
            .new_value()
            .integer(self.func_def.block.stmt.num);
        let ret = main_data.dfg_mut().new_value().ret(Some(v));
        main_data
            .layout_mut()
            .bb_mut(entry)
            .insts_mut()
            .extend([ret]);
        program
    }
}
