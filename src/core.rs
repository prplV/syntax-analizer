#[derive(Debug)]
pub enum DataTypes {
    //  default dt
    Null, 
    //  error type 
    Unknown,
    //  Terms
    StartTerm,
    EndTerm,
    FirstTerm,
    SecondTerm,
    EndSlagTerm, 
    DblDotTerm,
    EqTerm,
    //  middle important
    Label,  // ?
    Int,    // ?
    Var,  
    IntWithComma, 
    VarWithComma,
    //  atomic 
    // Letter,
    // Number,
    // math opers
    Plus, 
    Minus,
    Multiply,
    Divide, 
    LogicalOr,
    LogicalAnd,
    LogicalNot, 
    FuncSin,
    FuncCos, 
    FuncAbs,
}
pub enum Blocks {
    Lang, 
    Mn, 
    Slag,
    Oper,
    RightSide,
}