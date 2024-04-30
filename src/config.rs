use core::panic;

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
    // >= 2 ints without commas activates this field (for err type "второе 12 32 конец слагаемого") 
    int_enum_control: bool,
}

impl Compiler {
    pub fn new(cmd: String) -> Self {
        Compiler {
            str: cmd, 
            prev_token: DataTypes::Null,
            current_block: Blocks::Lang,
            int_enum_control: false,
        }
    }
    pub fn proccess(&mut self) -> Result<(), Errs>{
        
        for (i ,token) in self.str.split_ascii_whitespace().enumerate() {
            //println!("{}. {}", i, token);
            // stupid pipe needs to be deleted soon
            if i == self.str.split_ascii_whitespace().count() - 1 {
                if token != "конец" {
                    return Err(Errs::LangNoEndTerm);
                } else {
                    println!("end term");
                    self.prev_token = DataTypes::EndTerm;
                }
            } else {
                
                match self.prev_token {
                    DataTypes::Null => {
                        println!("null");
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
                        println!("start");
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

                    DataTypes::FirstTerm => {
                        println!("first");
                        match Compiler::define_var_type(&token) {
                            Ok(val) => self.prev_token = val,
                            Err(err) => {
                                if let Some(_) = Compiler::define_math_op_type(&token){
                                    return Err(err);
                                } else if let Ok(_) = Compiler::define_int_type(&token) {
                                    return Err(err);
                                } else if let Some(_) = Compiler::define_term_type(&token) {
                                    return Err(err);
                                }else {
                                    return Err(Errs::MnFirstWithoutVars);
                                }
                            },
                        }
                    },
                    DataTypes::SecondTerm => {
                        println!("second");
                        match Compiler::define_int_type(&token) {
                            Ok(val) => {
                                match val {
                                    DataTypes::Int => self.prev_token = val,
                                    _ => return Err(Errs::MnSecondWithoutInts),
                                }
                            },
                            Err(err) => {
                                if Compiler::check_outbound_int(&token) {
                                    return Err(err);
                                }
                                return Err(Errs::MnSecondWithoutInts);
                            },
                        }
                    },

                    DataTypes::Int => {
                        println!("int");
                        // mn 
                        if let Blocks::Mn = self.current_block {
                            // 1-0\0 int in second 
                            match Compiler::define_int_type(&token) {
                                Ok(val) => {
                                    match val {
                                        DataTypes::Int => {     
                                            // overvising "second"'s int enumeration with only 1 int with slag 
                                            self.int_enum_control = true;
                                            self.prev_token = val;
                                        },
                                        DataTypes::IntWithComma => {
                                            self.current_block = Blocks::Slag;
                                            self.int_enum_control = false;
                                            self.prev_token = val;
                                        },
                                        // rewrite to return error 
                                        // _ => panic!("Irregular behaviour"),
                                        _ => return Err(Errs::MnSecondWithoutInts),
                                    }
                                },
                                Err(_) => {
                                    //self.int_enum_control = false;
                                    //////////////////////////////////////////////////////
                                    if Compiler::check_outbound_int(&token) {
                                        return Err(Errs::AtomImpermissibleInt);
                                    }
                                    //////////////////////////////////////////////////////
                                    match Compiler::define_term_type(&token) {
                                        Some(val) => {
                                            match val {
                                                DataTypes::EndTerm => {
                                                    if self.int_enum_control == false {
                                                        return Err(Errs::SlagNoInts);
                                                    }
                                                    self.prev_token = DataTypes::EndSlagTerm1;
                                                    self.current_block = Blocks::Slag;
                                                },
                                                DataTypes::FirstTerm | DataTypes::SecondTerm => {
                                                    self.prev_token = val;
                                                }, 
                                                _ => return Err(Errs::MnSecondWithoutInts),
                                            }
                                        },
                                        None => return Err(Errs::MnSecondWithoutInts),
                                    }
                                },
                            }
                            // mn -> slag 
                            // 1) mn 1 int + slag 1 int 
                        }
                        // slag 
                        // last int -> end slag term 
                        else if let Blocks::Slag = self.current_block {
                            // int int check 
                            match Compiler::define_int_type(&token) {
                                Ok(_) => return Err(Errs::SlagIntsEnum),
                                Err(_) => {
                                    // pipe (pepe)
                                },
                            }
                            // term check 
                            match Compiler::define_term_type(&token) {
                                Some(val) => {
                                    match val {
                                        DataTypes::EndTerm => {
                                            self.prev_token = DataTypes::EndSlagTerm1;
                                        },
                                        _ => return Err(Errs::SlagEndTerm),
                                    }
                                },
                                None => return Err(Errs::SlagEndTerm),
                            }
                        }

                        // r.p. //////////////////////////////////////////
                        else if let Blocks::RightSide = self.current_block {
                            match Compiler::define_math_op_type(&token) {
                                Some(_val) => self.prev_token = _val,
                                None => {
                                    if let Ok(DataTypes::Int) = Compiler::define_int_type(&token) {
                                        self.current_block = Blocks::Oper;
                                        self.prev_token = DataTypes::Label;
                                    } else if let Some(DataTypes::EndTerm) = Compiler::define_term_type(&token) {
                                        self.prev_token = DataTypes::EndTerm;
                                        self.current_block = Blocks::Lang;
                                    } else {
                                        return Err(Errs::RightSideInt);
                                    }
                                },
                            }
                        }
                        // math ops 
                    },
                    DataTypes::IntWithComma => {
                        println!("int,");
                        // int 
                        // int, 
                        match Compiler::define_int_type(&token) {
                            Ok(val) => {
                                if let DataTypes::Label = val {
                                    return Err(Errs::SlagIntsEnum);
                                }
                                self.prev_token = val;
                            },
                            Err(_) => return Err(Errs::SlagAfterIntsWithCommaEnum),
                        }

                        
                        //todo!()
                    },              //vtoroe(...), slag (only one), label, 
                    DataTypes::Var => {
                        println!("var");
                        // first 
                        if let Blocks::Mn = self.current_block {
                            match Compiler::define_term_type(&token) {
                                Some(val) => {
                                    match val {
                                        DataTypes::FirstTerm | 
                                        DataTypes::SecondTerm => {
                                            self.prev_token = val;
                                        },
                                        // wrong error 
                                        _ => return Err(Errs::MnVarsEnumeration),
                                    }
                                },
                                // 'int,', int 
                                None => {
                                    match Compiler::define_int_type(&token) {
                                        Ok(_val) => {
                                            if let DataTypes::Label = _val {
                                                return Err(Errs::SlagIntsEnum);
                                            }
                                            self.prev_token = _val; 
                                            self.current_block = Blocks::Slag;
                                        },
                                        Err(_) => {
                                            if let Ok(_) = Compiler::define_var_type(&token) {
                                                return Err(Errs::MnFirstWithoutVars);
                                            }
                                            else {
                                                return Err(Errs::MnVarsEnumeration);
                                            }
                                        },//return Err(Errs::MnVarsEnumeration),
                                    }
                                }, //return Err(Errs::MnVarsEnumeration),
                            }
                        } else if let Blocks::Oper = self.current_block {
                            match Compiler::define_term_type(&token) {
                                Some(_val) => {
                                    match _val {
                                        DataTypes::EqTerm => {
                                            self.prev_token = _val;
                                        },
                                        _ => return Err(Errs::OperNoEqSymbol),
                                    }
                                },
                                None => {
                                    return Err(Errs::OperNoEqSymbol);
                                },
                            }
                        } ///////////////////////////// rp
                        else if let Blocks::RightSide = self.current_block {
                            match Compiler::define_math_op_type(&token) {
                                Some(_val) => self.prev_token = _val,
                                None => {
                                    if let Ok(DataTypes::Int) = Compiler::define_int_type(&token) {
                                        self.prev_token = DataTypes::Label;
                                        self.current_block = Blocks::Oper;
                                    } else if let Some(DataTypes::EndTerm) = Compiler::define_term_type(&token) {
                                        self.prev_token = DataTypes::EndTerm;
                                        self.current_block = Blocks::Lang;
                                    } else {
                                        return Err(Errs::RightSideVar)
                                    }
                                },
                            }
                        }
                    },
                    DataTypes::VarWithComma => {
                        println!("var,");
                        // var,
                        match Compiler::define_var_type(&token) {
                            Ok(_val) => {
                                self.prev_token = _val;
                            },
                            Err(er) => {
                                if let Ok(_) = Compiler::define_int_type(&token) {
                                    return Err(Errs::MnFirstWithoutVars);
                                } else if let Some(_) = Compiler::define_term_type(&token) {
                                    return Err(Errs::LangUsingReservedTokens);
                                } else {
                                    return Err(er)
                                }
                            }, // return Err(er),
                        }
                    },
                    
                    DataTypes::EndSlagTerm1 => {
                        println!("slag1");
                        match Compiler::define_term_type(&token) {
                            Some(val) => {
                                if let DataTypes::EndSlagTerm2 = val {
                                    self.prev_token = val;
                                } else {
                                    return Err(Errs::SlagEndTerm);
                                }
                            },
                            None => return Err(Errs::SlagEndTerm),
                        }
                    },
                    DataTypes::EndSlagTerm2 => {
                        println!("slag2");
                        match Compiler::define_int_type(&token) {
                            Ok(_val) => {
                                if let DataTypes::Int = _val {
                                    self.current_block = Blocks::Oper;
                                    self.prev_token = DataTypes::Label;
                                } else if let DataTypes::Label = _val {
                                    self.current_block = Blocks::Oper;
                                    self.prev_token = DataTypes::DblDotTerm;
                                } else {
                                    return Err(Errs::AtomImpermissibleLabel);
                                }
                            },
                            Err(_) => return Err(Errs::OperNoLabel),
                        }
                    },

                    DataTypes::Label => {
                        println!("label");
                        match Compiler::define_term_type(&token) {
                            Some(val) => {
                                if let DataTypes::DblDotTerm = val {
                                    self.prev_token = val;
                                } else {
                                    return Err(Errs::OperNoDblDotAfterLabel);
                                }
                            },
                            None => return Err(Errs::OperNoDblDotAfterLabel),
                        }
                    },

                    DataTypes::DblDotTerm => {
                        println!(":");
                        match Compiler::define_var_type(&token) {
                            Ok(_val) => self.prev_token = _val,
                            Err(_) => return Err(Errs::OperNoVarName),
                        }
                    },
                    ////// START RP
                    DataTypes::EqTerm => {
                        println!("=");
                        // -, func, var, int
                        if let Ok(DataTypes::Int) = Compiler::define_int_type(&token) {
                            self.prev_token = DataTypes::Int;
                        } else if let Ok(DataTypes::Var) = Compiler::define_var_type(&token) {
                            self.prev_token = DataTypes::Var;
                        } else {
                            match Compiler::define_math_op_type(&token) {
                                Some(_val) => {
                                    match _val {
                                        DataTypes::Minus | DataTypes::FuncSin 
                                        | DataTypes::FuncCos | DataTypes::FuncAbs => {
                                            self.prev_token = _val;
                                        }, 
                                        _ => return Err(Errs::RightSideStart),
                                    }
                                },
                                None => return Err(Errs::RightSideStart), 
                            }
                        }
                    },
                    DataTypes::Plus | DataTypes::Minus | DataTypes::Multiply|
                    DataTypes::Divide | DataTypes::LogicalAnd | DataTypes::LogicalOr |
                    DataTypes::LogicalNot | DataTypes::FuncAbs | DataTypes::FuncSin | 
                    DataTypes::FuncCos => {
                        println!("math-op");
                        // int, var, func 
                        if let Ok(DataTypes::Int) = Compiler::define_int_type(&token) {
                            self.prev_token = DataTypes::Int;
                            self.current_block = Blocks::RightSide;
                        } else if let Ok(DataTypes::Var) = Compiler::define_var_type(&token) {
                            self.prev_token = DataTypes::Var;
                            self.current_block = Blocks::RightSide;
                        } else {
                            match Compiler::define_math_op_type(&token) {
                                Some(_val) => {
                                    match _val {
                                        DataTypes::FuncSin 
                                        | DataTypes::FuncCos | DataTypes::FuncAbs => {
                                            self.prev_token = _val;
                                            self.current_block = Blocks::RightSide;
                                        }, 
                                        _ => {
                                            match Compiler::define_math_op_error(&self.prev_token) {
                                                Some(er) => return Err(er),
                                                None => panic!("impermissible behaviour"),
                                            };
                                        },//return Err(Errs::RightSideStart),
                                    }
                                },
                                None => {
                                    match Compiler::define_math_op_error(&self.prev_token) {
                                        Some(er) => return Err(er),
                                        None => panic!("impermissible behaviour"),
                                    };
                                }, 
                            }
                        }
                    },

                    // ---------
                    DataTypes::EndTerm => {
                        println!("end term");
                        break;
                    },
                    _ => todo!(),
                }

            }
        }

        
        Ok(())
    }
    pub fn define_int_type(wit: &str) -> Result<DataTypes, Errs> {
        for (i, num) in wit.chars().enumerate() {
            if Compiler::verify_num(&num) {
                continue;
            } else if (i == wit.chars().count() - 1) && (num == ',') && (i != 0) {
                return Ok(DataTypes::IntWithComma);
            } else if (i == wit.chars().count() - 1) && (num == ':') && (i != 0) {
                return Ok(DataTypes::Label);
            } else {
                return Err(Errs::AtomImpermissibleInt);
            }
        }
        Ok(DataTypes::Int)
    }
    // ???
    fn check_outbound_int(wit: &str) -> bool {
        // match wit {
        //     '8' | '9' => return true,
        //     _ => return false,
        // }
        for num in wit.chars() {
            match num {
                    '8' | '9' => return true,
                    _ => continue,
                }
        } return false;
    }
    pub fn define_var_type(wit: &str) -> Result<DataTypes, Errs> {
        if let Some(_) = Compiler::define_term_type(&wit) {
            return Err(Errs::LangUsingReservedTokens);
        }
        for (i, sym) in wit.to_lowercase().chars().enumerate() {
            if i == 0 {
                if !Compiler::verify_let(&sym) {
                    return Err(Errs::AtomImpermissibleVarStart);
                }
            } else if (i == wit.char_indices().count() - 1) { 
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
            'с' | 'т' | 'у' | 'ф' | 'х' | 'ц' | 'ч' | 'щ' | 'ш' | 'ъ' | 'ы' 
            | 'ь' | 'э' | 'ю' | 'я' => return true,
            _ => return false,
        }
    }
    fn define_math_op_error(wit: &DataTypes) -> Option<Errs> {
        match wit {
            DataTypes::Plus => Some(Errs::RightSidePlus),
            DataTypes::Minus => Some(Errs::RightSideMinus),
            DataTypes::Multiply => Some(Errs::RightSideMultiply),
            DataTypes::Divide => Some(Errs::RightSideDivide),
            DataTypes::LogicalNot => Some(Errs::RightSideLogNot),
            DataTypes::LogicalAnd => Some(Errs::RightSideLogAnd),
            DataTypes::LogicalOr => Some(Errs::RightSideLogOr),
            DataTypes::FuncSin => Some(Errs::RightSideFuncSin),
            DataTypes::FuncCos => Some(Errs::RightSideFuncCos),
            DataTypes::FuncAbs => Some(Errs::RightSideFuncAbs),
            _ => None,
        }
    }
    pub fn define_math_op_type(wit: &str) -> Option<DataTypes> {
        match wit.to_lowercase().as_str() {
            "+" => return Some(DataTypes::Plus),
            "-" => return Some(DataTypes::Minus),
            "*" => return Some(DataTypes::Multiply),
            "/" => return Some(DataTypes::Divide),
            "and" => return Some(DataTypes::LogicalAnd),
            "or" => return Some(DataTypes::LogicalOr),
            "not" => return Some(DataTypes::LogicalNot),
            "sin" => return Some(DataTypes::FuncSin),
            "cos" => return Some(DataTypes::FuncCos),
            "abs" => return Some(DataTypes::FuncAbs),
            _ => return None,
        }
    }
    pub fn define_term_type(wit: &str) -> Option<DataTypes> {
        match wit.to_lowercase().as_str() {
            "начало" => return Some(DataTypes::StartTerm),
            "конец" => return Some(DataTypes::EndTerm),
            "первое" => return Some(DataTypes::FirstTerm),
            "второе" => return Some(DataTypes::SecondTerm),
            //"конец" => return Some(DataTypes::EndSlagTerm1),
            "слагаемого" => return Some(DataTypes::EndSlagTerm2),
            ":" => return Some(DataTypes::DblDotTerm),
            "=" => return Some(DataTypes::EqTerm),
            _ => return None,
        }
    }
}