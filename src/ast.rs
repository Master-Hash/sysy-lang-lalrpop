#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}

#[derive(Debug)]
pub enum FuncType {
    Int,
}

#[derive(Debug)]
pub enum BType {
    Int,
}

#[derive(Debug)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    Stmt(Stmt),
    Decl(Decl),
}

// 这玩意是时候改名叫 Return Stmt 了
#[derive(Debug)]
pub struct Stmt {
    pub exp: Exp,
}

#[derive(Debug)]
pub enum Decl {
    ConstDecl {
        b_type: BType,
        const_def: Vec<ConstDef>,
    },
}

#[derive(Debug)]
pub struct ConstDef {
    pub ident: String,
    pub const_exp: ConstExp,
}

// Exp         ::= LOrExp;
// PrimaryExp  ::= "(" Exp ")" | Number;
// Number      ::= INT_CONST;
// UnaryExp    ::= PrimaryExp | UnaryOp UnaryExp;
// UnaryOp     ::= "+" | "-" | "!";
// MulExp      ::= UnaryExp | MulExp ("*" | "/" | "%") UnaryExp;
// AddExp      ::= MulExp | AddExp ("+" | "-") MulExp;
// RelExp      ::= AddExp | RelExp ("<" | ">" | "<=" | ">=") AddExp;
// EqExp       ::= RelExp | EqExp ("==" | "!=") RelExp;
// LAndExp     ::= EqExp | LAndExp "&&" EqExp;
// LOrExp      ::= LAndExp | LOrExp "||" LAndExp;

// #[derive(Debug)]
// pub struct Exp {
//     // pub l_or_exp: LOrExp,
//     pub unary_exp: UnaryExp,
// }

#[derive(Debug)]
pub enum Exp {
    Number(i32),
    LVal(String),
    Paren(Box<Exp>),
    PlusUnary(Box<Exp>),
    MinusUnary(Box<Exp>),
    NotUnary(Box<Exp>),
    MulBinary(Box<Exp>, Box<Exp>),
    DivBinary(Box<Exp>, Box<Exp>),
    ModBinary(Box<Exp>, Box<Exp>),
    AddBinary(Box<Exp>, Box<Exp>),
    SubBinary(Box<Exp>, Box<Exp>),
    LTBinary(Box<Exp>, Box<Exp>),
    LEBinary(Box<Exp>, Box<Exp>),
    GTBinary(Box<Exp>, Box<Exp>),
    GEBinary(Box<Exp>, Box<Exp>),
    EqBinary(Box<Exp>, Box<Exp>),
    NeBinary(Box<Exp>, Box<Exp>),
    LAndBinary(Box<Exp>, Box<Exp>),
    LOrBinary(Box<Exp>, Box<Exp>),
}

// 事实上计算常量仍然需要借助符号表
// 在解析时就算出来意义不大
// type ConstExp = i32;
#[derive(Debug)]
pub struct ConstExp(pub Exp);

// #[derive(Debug)]
// pub enum PrimaryExp {
//     Exp(Box<Exp>),
//     Number(i32),
// }

// #[derive(Debug)]
// pub enum UnaryExp {
//     Primary(PrimaryExp),
//     PlusUnary(Box<UnaryExp>),
//     MinusUnary(Box<UnaryExp>),
//     NotUnary(Box<UnaryExp>),
// }

// #[derive(Debug)]
// pub enum MulExp {
//     UnaryExp(UnaryExp),
//     BinaryMulExp(Box<MulExp>, UnaryExp),
// }

// #[derive(Debug)]

// pub enum AddExp {
//     MulExp(MulExp),
//     BinaryAddExp(Box<AddExp>, MulExp),
// }

// #[derive(Debug)]
// pub enum RelExp {
//     AddExp(AddExp),
//     BinaryRelExp(Box<RelExp>, AddExp),
// }

// #[derive(Debug)]
// pub enum EqExp {
//     RelExp(RelExp),
//     BinaryEqExp(Box<EqExp>, RelExp),
// }

// #[derive(Debug)]
// pub enum LAndExp {
//     EqExp(EqExp),
//     BinaryLAndExp(Box<LAndExp>, EqExp),
// }

// #[derive(Debug)]
// pub enum LOrExp {
//     LAndExp(LAndExp),
//     BinaryLOrExp(Box<LOrExp>, LAndExp),
// }
