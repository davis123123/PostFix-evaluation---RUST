use std::io;
use std::cmp::Ordering;
use std::string::String;
use std::vec::Vec;
use std::option::Option;
use std::ops::{Add, Sub, Mul, Div};

pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
}

pub enum Token {
    Operator(Operator),
    Operand(isize),
}

fn do_operation<F, V>(f: F, v1: V, v2: V) -> V
    where
        F: Fn(V, V) -> V,
        V: Add + Sub + Mul + Div {
    f(v1, v2)
}

pub fn eval(tokens: &[Token]) -> Option<isize> {
    let mut stack: Vec<isize> = Vec::new();
    let mut result:isize = 0;
    let mut numCounter = 0;
    let mut opCounter = 0;
    for tok in tokens.into_iter(){

        match *tok{
            Token::Operand(i) => {
                stack.push(i);
                //numCounter += 1;
                continue;
            },
            Token::Operator(ref Operator) =>{
                if stack.len() < 2{
                    return None;
                }
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                opCounter += 1;
                result = match *Operator{
                    Operator::Add =>{l + r},
                    Operator::Sub =>{l - r},
                    Operator::Mul =>{l * r},
                };
                stack.push(result);
            },
        };

     }//end for

        /*if opCounter >= numCounter {
            return None;
        }
        if opCounter != (numCounter - 1){
            return None;
        }*/

    if stack.len() !=  1{
        return None;
    }
    stack.pop()
}
/*let r = stack.pop().expect(return None);
//numCounter += 1;
//if stack.len() != 0{
//     numCounter += 1;
let l = stack.pop().expect(return None);
//    }

match *tok{
    Token::Operand(i) => {},
    Token::Operator(ref Operator) =>{
       opCounter += 1;
       result = match *Operator{
           Operator::Add =>{ do_operation(|l, r| l + r, l, r)},
           Operator::Sub =>{do_operation(|l, r| l - r, l, r)},
           Operator::Mul =>{do_operation(|l, r| l * r, l, r)},
       };
    },
};
stack.push(result);*/
