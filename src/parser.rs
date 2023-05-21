use crate::constants::*;
use crate::utils::*;

use sexp::Atom::*;
use sexp::*;

use im::HashSet;
use regex::Regex;


pub fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => { Expr::Number(*n)}
        Sexp::Atom(S(boolean)) if boolean == "true" => Expr::Boolean(true),
        Sexp::Atom(S(boolean)) if boolean == "false" => Expr::Boolean(false),
        Sexp::Atom(S(id)) => Expr::Id(id.to_string()),
        Sexp::List(vec) => {
            match &vec[..] {
                [Sexp::Atom(S(op)), e] if op == "add1" => Expr::UnOp(Op1::Add1, Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::UnOp(Op1::Sub1, Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "isnum" => Expr::UnOp(Op1::IsNum, Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "isbool" => Expr::UnOp(Op1::IsBool, Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "print" => Expr::UnOp(Op1::Print, Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e1, e2] if op == "+" => Expr::BinOp(Op2::Plus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == "-" => Expr::BinOp(Op2::Minus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == "*" => Expr::BinOp(Op2::Times, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == ">" => Expr::BinOp(Op2::Greater, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == "<" => Expr::BinOp(Op2::Less, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == "=" => Expr::BinOp(Op2::Equal, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == ">=" => Expr::BinOp(Op2::GreaterEqual, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == "<=" => Expr::BinOp(Op2::LessEqual, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(command)), e1, e2, e3] if command == "if" => Expr::If(Box::new(parse_expr(e1)), Box::new(parse_expr(e2)), Box::new(parse_expr(e3))),
                [Sexp::Atom(S(command)), e] if command == "loop" => Expr::Loop(Box::new(parse_expr(e))),
                [Sexp::Atom(S(command)), e] if command == "break" => Expr::Break(Box::new(parse_expr(e))),
                [Sexp::Atom(S(command)), _e, ..] if command == "block" => {

                    Expr::Block(parse_block(&Sexp::List((&vec[1..]).to_vec())))
                }
                [Sexp::Atom(S(command)), Sexp::Atom(S(id)), e] if command == "set!" => Expr::Set(id.to_string(), Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), bindings, e] if op == "let" => {Expr::Let(parse_bind(bindings), Box::new(parse_expr(e))) },
                [Sexp::Atom(S(op)), args@..] if op == "array" => {
                    let mut args_vec = Vec::new();
                    for arg in args.iter() {
                        args_vec.push(parse_expr(arg));
                    }
                    Expr::Array(args_vec)
                },
                [Sexp::Atom(S(op)), e1, e2] if op == "getIndex" => Expr::GetIndex(Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2, e3] if op == "setIndex" => Expr::SetIndex(Box::new(parse_expr(e1)), Box::new(parse_expr(e2)), Box::new(parse_expr(e3))),
                [Sexp::Atom(S(op)), e1] if op == "len" => Expr::Len(Box::new(parse_expr(e1))),
                [Sexp::Atom(S(op)), e1, e2] if op == "append" => Expr::Append(Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(id)), args@..] => {
                    let mut args_vec = Vec::new();
                    for arg in args.iter() {
                        args_vec.push(parse_expr(arg));
                    }
                    Expr::Call(id.to_string(), args_vec)
                }

                _ => panic!("Invalid"),
            }
        },
        _ => panic!("Invalid"),
    }
}


// to check
fn parse_block(s: &Sexp) -> Vec<Expr> {
    let mut block_vec = Vec::new();
    match s {
        Sexp::List(exprs) => {
          for expr in exprs.iter() {
            block_vec.push(parse_expr(expr));
              println!("{:?}", block_vec);
          }
          block_vec
        }
        _ => panic!("Invalid"),
    }
}

fn parse_bind(s: &Sexp) -> Vec<(String, Expr)> {
    let mut bindings_vec = Vec::new();
    match s {
        Sexp::List(bindings) => {
          for binding in bindings.iter() {
            let current_binding = match binding {
                Sexp::List(vec) => {
                    match &vec[..] {
                        [Sexp::Atom(S(id)), e] => {
                            if KEYWORDS.contains(&id.as_ref()) {
                                panic!("Invalid : binding overlaps with reserved keyword")
                            }
                          match id.as_ref() {
                            "input" => {
                              panic!("Invalid : binding overlaps with reserved keyword input")
                            }

                            _ => {
                                vec![(id.to_string(), parse_expr(e))]
                            },

                          }
                        },
                        _ => panic!("Invalid"),
                    }
                },
                _ => panic!("Invalid"),
            };
            bindings_vec.extend(current_binding);
          }
          bindings_vec
        }
        _ => panic!("Invalid"),
    }
}

fn parse_definition(s: &Sexp) -> Definition {
    match s {
        Sexp::List(def_vec) => match &def_vec[..] {
            [Sexp::Atom(S(keyword)), Sexp::List(name_vec), body] if keyword == "fun" => match &name_vec[..] {
                [Sexp::Atom(S(fun_name)), exprs @ ..] => {
                    let mut args: Vec<String> = vec![];
                    for arg in exprs {
                        match arg {
                            Sexp::Atom(S(arg_name)) => args.push(arg_name.clone()),
                            _ => panic!("Invalid : bad arg"),
                        }
                    }
                    Definition {
                        name: fun_name.clone(),
                        args,
                        body: parse_expr(body),
                    }
                },
                _ => panic!("Invalid : Bad fundef"),
            },
            _ => panic!("Invalid : Bad fundef"),
        },
        _ => panic!("Invalid : Bad fundef"),
    }
}

pub fn parse_program(s: &Sexp) -> Program {
    let re = Regex::new(r"^[a-zA-Z_]+$").unwrap();
    match s {
        Sexp::List(vec) => {
            let mut function_set = HashSet::new();
            function_set.insert("throw_error".to_string());
            function_set.insert("our_code_starts_here".to_string());
            function_set.extend(KEYWORDS.iter().map(|s| s.to_string()));

            let mut defs: Vec<Definition> = vec![];
            for def_or_exp in vec {
                if is_def(def_or_exp) {
                    let definition = parse_definition(def_or_exp);
                    if function_set.contains(&definition.name) && re.is_match(&definition.name) {
                        panic!("Invalid : duplicate function name");
                    }
                    defs.push(parse_definition(def_or_exp));
                    function_set.insert(definition.name.clone());
                } else {
                    if defs.len() + 1 != vec.len() {
                        panic!("Invalid : functions defined after main");
                    }
                    return Program {
                        defs,
                        main: parse_expr(def_or_exp),
                    };
                }
            }
            panic!("Invalid : missing main");
        }
        _ => panic!("Invalid : not a program"),
    }
}