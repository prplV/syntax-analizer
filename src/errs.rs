#[derive(Debug)]
pub enum Errs {
    // COLLAPSE
    ImpermissibleBehaviour,
    //  global  
    LangNoStartTerm,
    LangNoEndTerm,
    LangUsingReservedTokens,
    //  atomic  
    AtomImpermissibleVar,
    AtomImpermissibleVarStart,
    AtomImpermissibleInt,
    AtomMathOperator,           // fe ++ etc
    AtomImpermissibleLabel,
    AtomImpermissibleNum,       // > 7
    AtomImpermissibleLetter,    // не из русского алфавита 
    //  mn  
    MnNoPrefixTerm,             // нет терминала 
    MnVarsEnumeration,
    MnIntsEnumeration,
    MnFirstWithoutVars,
    MnSecondWithoutInts,
    //  slag
    SlagIntsEnum,
    SlagAfterIntsWithCommaEnum,
    SlagNoInts,
    SlagEndTerm,                // нет "конец слагаемого"
    //  oper
    OperNoLabel,
    OperLabelNotInt,
    OperNoDblDotAfterLabel,     // нет :
    OperNoVarName,              // нет имени переменной после : 
    OperNoEqSymbol,             // нет = после имени переменной
    // right side 
    RightSideStart,
    RightSidePlus,
    RightSideMinus,
    RightSideMultiply,
    RightSideDivide,
    RightSideLogAnd,
    RightSideLogOr,
    RightSideLogNot,
    RightSideFuncSin,
    RightSideFuncCos,
    RightSideFuncAbs,
    RightSideVar,
    RightSideInt,
    // --
}

impl Errs{
    pub fn print(&self) -> &str {
        match self {
            Errs::ImpermissibleBehaviour => { return "Impermissible Behaviour! Foreign token"; },
            Errs::LangNoStartTerm => { return "Язык должен начинаться с терминала 'начало'"; },
            Errs::LangNoEndTerm => { return "Язык должен оканчиваться терминалом 'конец'"; },
            Errs::LangUsingReservedTokens => {return "Использование зарезервированных слов-терминалов в качестве имен переменных недопустимо"},
            Errs::AtomImpermissibleVar => { return "В именах переменных могут быть использованы только буквы кириллицы и цифры восьмеричной системы счиления"; },
            Errs::AtomImpermissibleVarStart => { return "Имена переменных должны начинаться с буквы русского алфавита"; },
            Errs::AtomImpermissibleInt => { return "Целые числа могут состоять только из цифр восьмеричной системы счисления"; },
            Errs::AtomMathOperator => { return "Некорректная математическая операция"; },
            Errs::AtomImpermissibleLabel => { return "Некорректный вид метки"; },
            Errs::AtomImpermissibleNum => { return "Используемые целые числа должны быть восьмиричными"; },
            Errs::AtomImpermissibleLetter => { return "Используемые буквы должны быть из русского алфавита"; },
            Errs::MnNoPrefixTerm => { return "Множество должно начинаться с терминала (либо 'первое', либо 'второе')"; },
            Errs::MnVarsEnumeration => { return "После множества может быть либо слагаемое, либо еще одно множество"; },
            Errs::MnIntsEnumeration => { return ""; },
            Errs::MnFirstWithoutVars => { return "После терминала 'первое' должны идти переменные (сочетение букв кириллицы и цифр восьмиричной системы счисления с первой буквой) через ','"; },
            Errs::MnSecondWithoutInts => { return "После терминала 'второе' должны идти целые числа россыпью"; },
            Errs::SlagIntsEnum => { return "В слагаемом должны быть записаны целые числа через ','"; },
            Errs::SlagAfterIntsWithCommaEnum => {return "В слагаемом после запятой ожидается целое";}
            Errs::SlagNoInts => {return "В слагаемом должно быть хотя бы одно целое число";},
            Errs::SlagEndTerm => { return "Слагаемое должно оканчиваться терминалом 'конец слагаемого'"; },
            Errs::OperNoLabel => { return "Оператор должен начинаться с целого числа, являющегося меткой"; },
            Errs::OperLabelNotInt => { return "В операторе метка должна представлять из себя восьмиричное целое число" },
            Errs::OperNoDblDotAfterLabel => { return "В операторе после метки должно быть двоеточие"; },
            Errs::OperNoVarName => { return "В операторе после двоеточия должно быть имя переменной"; },
            Errs::OperNoEqSymbol => { return "В операторе после имени переменной должен быть терминал '='"; },
            Errs::RightSideStart => { return "Правая часть может начинаться только с аддитивной операции '-', фукнций, целого числа или переменной "; },
            Errs::RightSidePlus => { return "В правой части после операции '+' может стоять либо переменная, либо целое число, либо функция"; },
            Errs::RightSideMinus => { return "В правой части после операции '-' может стоять либо переменная, либо целое число, либо функция"; },
            Errs::RightSideMultiply => { return "В правой части после операции '*' может стоять либо переменная, либо целое число, либо функция"; },
            Errs::RightSideDivide => { return "В правой части после операции '/' может стоять либо переменная, либо целое число, либо функция"; },
            Errs::RightSideLogNot => { return "В правой части после операции 'not' может стоять либо переменная, либо целое число, либо функция"; },
            Errs::RightSideLogAnd => { return "В правой части после операции 'and' может стоять либо переменная, либо целое число, либо функция"; },
            Errs::RightSideLogOr => { return "В правой части после операции 'or' может стоять либо переменная, либо целое число, либо функция"; },
            Errs::RightSideFuncSin => { return "В правой части после операции 'sin' может стоять либо переменная, либо целое число, либо функция"; },
            Errs::RightSideFuncCos => { return "В правой части после операции 'cos' может стоять либо переменная, либо целое число, либо функция"; },
            Errs::RightSideFuncAbs => { return "В правой части после операции 'abs' может стоять либо переменная, либо целое число, либо функция"; },
            Errs::RightSideVar => { return "В правой части после переменной может стоять либо функция, либо математическая операция, либо метка в начале блока оператор"; },
            Errs::RightSideInt => { return "В правой части после целого числа может стоять либо функция, либо математическая операция, либо метка в начале блока оператор"; },
        }
    }
}