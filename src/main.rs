mod ast;
use koopa::back::KoopaGenerator;
use koopa::ir::builder_traits::*;
use koopa::ir::*;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;

// mod ir;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);

fn main() -> Result<()> {
    // 解析命令行参数
    let mut args = args();
    args.next();
    let mode = args.next().unwrap();
    let input = args.next().unwrap();
    args.next();
    let output = args.next().unwrap();

    // 读取输入文件
    let input = read_to_string(input)?;

    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast = sysy::CompUnitParser::new().parse(&input).unwrap();

    // IR
    // fun @main(): i32 {  // main 函数的定义
    // %entry:             // 入口基本块
    // ret 0             // return 0
    // }
    let mut program = Program::new();
    let m = program.new_func(FunctionData::new(
        format!("@{}", ast.func_def.ident),
        vec![],
        match ast.func_def.func_type {
            ast::FuncType::Int => Type::get_i32(),
            _ => panic!(),
        },
    ));
    let main_data = program.func_mut(m);

    let entry = main_data
        .dfg_mut()
        .new_bb()
        .basic_block(Some("%entry".into()));
    main_data.layout_mut().bbs_mut().extend([entry]);

    let zero = main_data.dfg_mut().new_value().integer(0);
    let ret = main_data.dfg_mut().new_value().ret(Some(zero));
    main_data
        .layout_mut()
        .bb_mut(entry)
        .insts_mut()
        .extend([ret]);

    // 输出解析得到的 AST
    println!("{:#?}", ast);

    // 输出 IR
    let mut gen = KoopaGenerator::new(Vec::new());
    gen.generate_on(&program).unwrap();
    let text_form_ir = std::str::from_utf8(&gen.writer()).unwrap().to_string();
    println!("{}", text_form_ir);

    Ok(())
}
