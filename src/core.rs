#[derive(Debug)]
pub enum DataTypes {
    //  default dt
    Null, 
    //  error type 
    //  Terms
    StartTerm,
    EndTerm,
    FirstTerm,
    SecondTerm,
    //
    EndSlagTerm1, 
    EndSlagTerm2,
    //
    DblDotTerm,
    EqTerm,
    //  middle important
    Label,  // ?
    FullLabel,
    Int,   // ?
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
#[derive(Debug)]
pub enum Blocks {
    Lang, 
    Mn, 
    Slag,
    Oper,
    RightSide,
}