// lang = 'старт' mn...mn slag oper...oper 'конец'
// mn = 'первое' var','..var ! 'второе' cel...cel
// slag = cel','...cel 'конец слагаемого'
// oper = mark ':' var '=' r.p.
//
// r.p. = </ '-' /> block1['+' ! '-']...block1
// block1 = block2['*' ! '/']...block2
// block2 = block3['and' ! 'or']...block3
// block3 = </ 'not' /> block4...block4
// block4 = </ func...func /> block5
// block5 = cel ! var 
//
// func = 'sin' ! 'cos' ! 'tg'
// var = number  </ symbol...symbol />
// symbol = number ! letter
// mark = cel 
// cel = number...number 
// number = '0' ! '1' ! ... ! '7'
// letter = 'А' ! 'Б' ! .. ! 'Я'

// examples :
// начало первое пер1, пер2 второе 12 34 2 2 21 5 первое пер3, пер4, пер5 12, 123, 4, 5, 1, 1 конец слагаемого 12: пер2 = - sin cos 23 + 2 - 2 not 1 * 4 / tg пер1 and а241 конец

mod config;
mod errs;
mod core;
use config::Compiler;
extern crate evalmath;

use std::io::stdin;


// fn syntax_analizer(a: &str){
//     let b = a.trim().to_lowercase();
//     for (i, num) in b.split_whitespace().enumerate(){
//         println!("{i}) {num}");
//     }
// }
// fn detect_term(t: &str) -> Result<DataTypes, Errs>{
//     // match t{
//     //     "начало" => {Ok(DataTypes::TermStart(t))},
//     //     "конец" => {Ok(DataTypes::TermFinish(t))},
//     //     //"конец слагаемого" => {},
//     //     _ => {Err(Errs::NoStartTerm)}
//     // }
// }

fn main() {
    // let a = "abcnetborrownet56";
    // let b: Vec<&str> = a.split("net").collect();
    // println!("{:?}", b); 
    //syntax_analizer("начало первое о1324ти, вц123 второе 12 34 2 2 21 5 первое л2345, р2ау, влад 12, 123, 4, 5, 1, 1 конец слагаемого 12 : а12 = - sin cos 23 + 2 - 2 not 1 * 4 / tg 21 and а241  конец");
    
    // let er: Errs = Errs::NoStartTerm;
    // er.print();

    // let handler = detect_term("конпаец");

    // match handler{ 
    //      Ok(val) => { val.print() },
    //      Err(err) => {err.print()},
    // }



    let mut str = String::new();

    loop {
        stdin().read_line(&mut str).expect("error input()");
        let mut comp = Compiler::new(str.trim().to_string());
        match comp.proccess() {
            Ok(_) => { println!("valid cmd")},
            Err(val) => { println!("{}", val.print()) },
        }
        str.clear();
    }


    // terms as vars
    // let mut comp = Compiler::new(String::from("начало первое п2, апцп2412, рке52, хч2цац второе 12 второе 42 123 51 1 второе 5 4, 123, 512 конец слагаемого 12 : а12 = - sin cos 23 + 2 - 2 not 1 * 4 / abs 21 and а241 12: а3 = 2 + 2 конец"));
    // match comp.proccess() {
    //     Ok(_) => { println!("valid cmd")},
    //     Err(val) => { println!("{}", val.print()) },
    // }

    //term rec test             SUCCESSFULLY PASSED!
    // println!("{:?}", Compiler::define_term_type("первое"));
    // println!("{:?}", Compiler::define_term_type("второе"));
    // println!("{:?}", Compiler::define_term_type("конец слагаемого"));
    // println!("{:?}", Compiler::define_term_type(":"));
    // println!("{:?}", Compiler::define_term_type("="));
    // println!("{:?}", Compiler::define_term_type("првое"));

    // verifying funcs tests
    // println!("{:?}", Compiler::verify_var("абвгдеёжзийклмнопрстуфкцчщшъыьэюя"));
    // println!("{:?}", Compiler::define_int_type("f123"));
    // println!("{:?}", Compiler::define_int_type("4121,"));
    // println!("{:?}", Compiler::define_int_type("012"));
    // println!("{:?}", Compiler::define_int_type("8443"));
    // println!("{:?}", Compiler::define_int_type("012:"));
    // println!("{:?}", Compiler::define_int_type("+"));
    // println!("{:?}", Compiler::define_int_type("конец"));

    // defining funcs tests     SUCCESSFULLY PASSED!
    // match  Compiler::define_var_type("п23е412т") {
    //     Ok(val) => println!("{:?}", val),
    //     Err(er) => println!("{}", er.print()),
    // }
    // match  Compiler::define_var_type("2п23цапаф412т") {
    //     Ok(val) => println!("{:?}", val),
    //     Err(er) => println!("{}", er.print()),
    // }
    // match  Compiler::define_var_type("п24а12т,") {
    //     Ok(val) => println!("{:?}", val),
    //     Err(er) => println!("{}", er.print()),
    // }
    // match  Compiler::define_var_type("п24а!12т,") {
    //     Ok(val) => println!("{:?}", val),
    //     Err(er) => println!("{}", er.print()),
    // }
    // match  Compiler::define_var_type("кКкКкКкКкКк") {
    //     Ok(val) => println!("{:?}", val),
    //     Err(er) => println!("{}", er.print()),
    // }


    // println!("{:?}", Compiler::define_int_type("45"));
    // println!("{:?}", Compiler::define_int_type("13,"));
    // println!("{:?}", Compiler::define_int_type("5324512"));
    // println!("{:?}", Compiler::define_int_type("123562360"));
    // println!("{:?}", Compiler::define_int_type("123562360,"));
    // println!("{:?}", Compiler::define_int_type("132518"));
    // println!("{:?}", Compiler::define_int_type("1325,18"));
}
