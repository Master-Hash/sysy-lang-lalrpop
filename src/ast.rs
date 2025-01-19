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
pub struct Block {
    pub stmt: Stmt,
}

#[derive(Debug)]
pub struct Stmt {
    pub exp: Exp,
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
    Exp(Box<Exp>),
    PlusUnary(Box<Exp>),
    MinusUnary(Box<Exp>),
    NotUnary(Box<Exp>),
}

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
