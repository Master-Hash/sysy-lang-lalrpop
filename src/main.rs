mod ast;
// mod codegen;
mod ir;
use koopa::back::KoopaGenerator;
use koopa::ir::ValueKind;
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

    // 此为可执行文件名
    args.next();

    let mode = args.next().unwrap();
    let input = args.next().unwrap();

    // 永远为 -o
    args.next();

    let output = args.next().unwrap();

    // 读取输入文件
    let input = read_to_string(input)?;

    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast = sysy::CompUnitParser::new().parse(&input).unwrap();

    // 输出解析得到的 AST
    println!("{:#?}", ast);

    let program = ast.new_ir();

    // 输出 IR
    let mut gen = KoopaGenerator::new(Vec::new());
    gen.generate_on(&program).unwrap();
    let text_form_ir = std::str::from_utf8(&gen.writer()).unwrap().to_string();
    println!("{}", text_form_ir);

    // 输出到文件
    match mode.as_str() {
        "-koopa" => {
            std::fs::write(output, &text_form_ir)?;
        }
        "-riscv" => {
            let mut res = "  .text\n".to_string();
            for &func in program.func_layout() {
                let func_data = program.func(func);
                match func_data.name() {
                    "@main" => res.push_str("  .globl main\nmain:\n"),
                    _ => panic!(),
                }
                // 遍历基本块列表
                for (&bb, node) in func_data.layout().bbs() {
                    // 一些必要的处理
                    // ...
                    // 遍历指令列表
                    for &inst in node.insts().keys() {
                        let value_data = func_data.dfg().value(inst);
                        match value_data.kind() {
                            ValueKind::Return(ret) => {
                                println!("{:?}", value_data);
                                let fuck = ret.value().unwrap();
                                match func_data.dfg().value(fuck).kind() {
                                    ValueKind::Integer(shit) => {
                                        res.push_str(&format!(
                                            "  li a0, {}\n  ret\n",
                                            &shit.value()
                                        ));
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            _ => unreachable!(),
                        }
                        // 访问指令
                        // ...
                    }
                }
            }
            println!("{}", &res);
            // panic!("RISC-V backend not implemented");
            std::fs::write(output, &res)?;
        }
        "-perf" => {
            panic!("Performance analysis not implemented");
        }
        _ => {
            panic!("Invalid mode");
        }
    }

    Ok(())
}
