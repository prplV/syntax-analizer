use core::panic;
use std::sync::atomic::compiler_fence;

//what i need to add 
// 1 - const k
// 2 - hash-set for vars and its values
// 3 -  
use crate::errs::Errs;
use crate::core::{ DataTypes, Blocks } ;
pub struct Compiler { 
    str: String,
    prev_token: DataTypes, 
    current_block: Blocks,
}

impl Compiler {
    pub fn new(cmd: String) -> Self {
        Compiler {
            str: cmd, 
            prev_token: DataTypes::Null,
            current_block: Blocks::Lang,
        }
    }
    pub fn proccess(&mut self) -> Result<(), Errs>{
        
        for (i ,token) in self.str.split_ascii_whitespace().enumerate() {
            //println!("{}. {}", i, token);
            if i == self.str.split_ascii_whitespace().count() - 1 {
                if token != "конец" {
                    return Err(Errs::LangNoEndTerm);
                } else {
                    self.prev_token = DataTypes::EndTerm;
                }
            } else {
                
                match self.prev_token {
                    DataTypes::Null => {
                        match Compiler::define_term_type(token) {
                            Some(val) => {
                                if let DataTypes::StartTerm = val {
                                    self.prev_token = val;
                                } else {
                                    return Err(Errs::LangNoStartTerm);
                                }
                            },
                            None => return Err(Errs::LangNoStartTerm),
                        }
                    },
                    DataTypes::StartTerm => {
                        match Compiler::define_term_type(&token) {
                            Some(val) => {
                                if let DataTypes::FirstTerm = val {
                                    self.current_block = Blocks::Mn;
                                    self.prev_token = val;
                                    continue;
                                } else if let DataTypes::SecondTerm = val {
                                    self.current_block = Blocks::Mn;
                                    self.prev_token = val;
                                    continue;
                                } else {
                                    return Err(Errs::MnNoPrefixTerm);
                                }
                            },
                            None => return Err(Errs::MnNoPrefixTerm),
                        }
                    },

                    DataTypes::FirstTerm => todo!(),
                    DataTypes::SecondTerm => todo!(),

                    DataTypes::Int => todo!(),
                    DataTypes::Var => todo!(),

                    DataTypes::EndSlagTerm => todo!(),
                    DataTypes::IntWithComma => todo!(),
                    DataTypes::VarWithComma => todo!(),

                    DataTypes::DblDotTerm => todo!(),

                    DataTypes::Plus => todo!(),
                    DataTypes::Minus => todo!(),
                    DataTypes::Multiply => todo!(),
                    DataTypes::Divide => todo!(),
                    DataTypes::LogicalAnd => todo!(),
                    DataTypes::LogicalOr => todo!(),
                    DataTypes::LogicalNot => todo!(),
                    DataTypes::FuncAbs => todo!(),
                    DataTypes::FuncCos => todo!(),
                    DataTypes::FuncSin => todo!(),

                    // ---------

                    _ => todo!(),
                }

            }
        }

        
        Ok(())
    }
    pub fn define_int_type(wit: &str) -> Option<DataTypes> {
        for (i, num) in wit.chars().enumerate() {
            match num {
                '0' | '1' 
                | '2' | '3' 
                | '4' | '5' 
                | '6' | '7'  => continue,
                _ => {
                    if (i == wit.chars().count() - 1) && (num == ',') {
                        return Some(DataTypes::IntWithComma);
                    } else {
                        return None;
                    }
                },
            }
        }
        Some(DataTypes::Int)
    }
    pub fn define_var_type(wit: &str) -> Result<DataTypes, Errs> {
        for (i, sym) in wit.to_lowercase().chars().enumerate() {
            if i == 0 {
                if Compiler::verify_num(&sym) {
                    return Err(Errs::AtomImpermissibleVarStart);
                }
            } else if i == wit.char_indices().count() - 1 { 
                if sym == ',' {
                    return Ok(DataTypes::VarWithComma);
                }
            } else {
                if Compiler::verify_num(&sym) || Compiler::verify_let(&sym) {
                    continue;
                } else {
                    return Err(Errs::AtomImpermissibleVar);
                }
            }
        } return Ok(DataTypes::Var);
    }
    pub fn verify_num(wit: &char) -> bool {
        match wit {
            '0' | '1' 
            | '2' | '3' 
            | '4' | '5' 
            | '6' | '7'  => return true,
            _ => return false,
        }
    }
    pub fn verify_let(wit: &char) -> bool {    
        match wit {
            'а' | 'б' | 'в' | 'г' | 'д' | 'е' | 'ё' | 'ж' | 'з' 
            | 'и' | 'й' | 'к' | 'л' | 'м' | 'н' | 'о' | 'п' | 'р' | 
            'с' | 'т' | 'у' | 'ф' | 'к' | 'ц' | 'ч' | 'щ' | 'ш' | 'ъ' | 'ы' 
            | 'ь' | 'э' | 'ю' | 'я' => return true,
            _ => return false,
        }
    }
    pub fn define_math_op_type(wit: &str) -> Option<DataTypes> {
        Some(DataTypes::Null)
    }
    pub fn define_term_type(wit: &str) -> Option<DataTypes> {
        match wit.to_lowercase().as_str() {
            "начало" => return Some(DataTypes::StartTerm),
            "конец" => return Some(DataTypes::EndTerm),
            "первое" => return Some(DataTypes::FirstTerm),
            "второе" => return Some(DataTypes::SecondTerm),
            "конец слагаемого" => return Some(DataTypes::EndSlagTerm),
            ":" => return Some(DataTypes::DblDotTerm),
            "=" => return Some(DataTypes::EqTerm),
            _ => return None,
        }
    }
}