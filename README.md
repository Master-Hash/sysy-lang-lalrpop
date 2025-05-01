# sysy-lang-lalrpop

[MaxXSoft 的编译原理课](https://pku-minic.github.io/online-doc/)作业，使用 Rust 和课程推荐的 Lalrpop 解析器生成器。选择 Lalrpop 而非 chumsky 是因为后者基于 PEG 文法，有时行为和直觉相去甚远（见龙书习题 4.4.5）。

目前 IR 进度 Lv4.2，代码生成部分 Lv2 已经完成，在完成 IR 前搁置。详见 CI。

本仓库不值得参考。更值得参考的仓库例如 [xhzgenius/Compiler-Lab](https://github.com/xhzgenius/Compiler-Lab)

如果你认为本仓库的 CI 有价值，或者我提供的 Docker 镜像工具链新于官方，或者单纯和 Ubuntu 有仇，欢迎来用我提供的镜像。

（Todo：上传我魔改的 Dockerfile）

（Todo：把 CI 用 matrix 语法重写）

## 开发文档：

* 测试脚本要求的编译器命令行界面：[compiler-dev/autotest](https://github.com/pku-minic/compiler-dev/tree/master/autotest)
* [测试样例](https://github.com/pku-minic/compiler-dev-test-cases/tree/master/testcases)
