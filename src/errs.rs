#[derive(Debug)]
pub enum Errs {
    //  global  
    LangNoStartTerm,
    LangNoEndTerm,
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
    SlagNoInts,
    SlagEndTerm,                // нет "конец слагаемого"
    //  oper
    OperNoLabel,
    OperLabelNotInt,
    OperNoDblDotAfterLabel,     // нет :
    OperNoVarName,              // нет имени переменной после : 
    OperNoEqSymbol,             // нет = после имени переменной
    // right side 
    // --
}

impl Errs{
    pub fn print(&self) -> &str {
        match self {
            Errs::LangNoStartTerm => { return "Язык должен начинаться с терминала 'начало'"; },
            Errs::LangNoEndTerm => { return "Язык должен оканчиваться терминалом 'конец'"; },
            Errs::AtomImpermissibleVar => { return "В именах переменных могут быть использованы только буквы кириллицы и цифры восьмеричной системы счиления"; },
            Errs::AtomImpermissibleVarStart => { return "Имена переменных должны начинаться с буквы русского алфавита"; },
            Errs::AtomImpermissibleInt => { return "Целые числа могут состоять только из цифр восьмеричной системы счисления"; },
            Errs::AtomMathOperator => { return "Некорректная математическая операция"; },
            Errs::AtomImpermissibleLabel => { return "Некорректный вид метки"; },
            Errs::AtomImpermissibleNum => { return "Используемые целые числа должны быть восьмиричными"; },
            Errs::AtomImpermissibleLetter => { return "Используемые буквы должны быть из русского алфавита"; },
            Errs::MnNoPrefixTerm => { return "Множество должно начинаться с терминала (либо 'первое', либо 'второе')"; },
            Errs::MnVarsEnumeration => { return ""; },
            Errs::MnIntsEnumeration => { return ""; },
            Errs::MnFirstWithoutVars => { return "После терминала 'первое' должны идти переменные через ','"; },
            Errs::MnSecondWithoutInts => { return "После терминала 'второе' должны идти целые числа россыпью"; },
            Errs::SlagIntsEnum => { return "В слагаемом целые числа должны идти через ','"; },
            Errs::SlagNoInts => {return "В слагаемом должно быть хотя бы одно целое число";},
            Errs::SlagEndTerm => { return "Слагаемое должно оканчиваться терминалом 'конец слагаемого'"; },
            Errs::OperNoLabel => { return "Оператор должен начинаться с метки"; },
            Errs::OperLabelNotInt => { return "В операторе метка должна представлять из себя восьмиричное целое число" },
            Errs::OperNoDblDotAfterLabel => { return "После метки должно быть двоеточие"; },
            Errs::OperNoVarName => { return "После двоеточия должно быть имя переменной"; },
            Errs::OperNoEqSymbol => { return "После имени переменной ожидался терминал '='"; },
        }
    }
}