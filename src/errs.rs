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
    SlagEndTerm,                // нет "конец слагаемого"
    //  oper
    OperNoLabel,
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
            Errs::AtomImpermissibleVar => { return "В именах переменных могут быть использованы только буквы кириллицы и цифры восьмиричной системы счиления"; },
            Errs::AtomImpermissibleVarStart => { return "Имена переменных должны начинаться с буквы"; },
            Errs::AtomImpermissibleInt => { return "Некорректный вид целого числа"; },
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
            Errs::SlagEndTerm => { return "Слагаемое должно оканчиваться терминалом 'конец слагаемого'"; },
            Errs::OperNoLabel => { return "Оператор должен начинаться с метки"; },
            Errs::OperNoDblDotAfterLabel => { return "После метки должно быть двоеточие"; },
            Errs::OperNoVarName => { return "После двоеточия должно быть имя переменной"; },
            Errs::OperNoEqSymbol => { return "После имени переменной ожидался терминал '='"; },
            _ => {todo!()}
        }
    }
}