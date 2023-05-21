use crate::constants::*;
use crate::utils::*;

use im::HashMap;
use im::HashSet;



pub fn compile_program(prog: &Program) -> Vec<Instr> {

    let mut user_defined_functions = HashMap::new();
    for def in &prog.defs {
        user_defined_functions.insert(def.name.clone(), def.args.len() as i64);
    }

    let mut instrs = Vec::new();
    let env = HashMap::new();
    let context = Context {
        si: 2,
        env: &env.update("input".to_string(), Loc::LReg(Reg::RDI)),
        brake: "",
        target: Loc::LReg(Reg::RAX),
        user_defined_functions: &user_defined_functions,
        current_function: "main",
      };
    let mut labels = 0;

    let mut body = compile_to_instrs(&prog.main, &context, &mut labels);
    instrs.append(&mut body);
    instrs.push(Instr::IRet);
    for defn in &prog.defs {
        instrs.extend(compile_definitions(defn, &user_defined_functions));
    }
    return instrs;
}

fn compile_definitions(defn: &Definition, user_defined_functions: &HashMap<String, i64>) -> Vec<Instr> {
    let mut instrs = Vec::new();
    instrs.push(Instr::ILabel(defn.name.clone()));
    let mut env = HashMap::new();
    let mut args_set = HashSet::new();

    for (i, arg) in defn.args.iter().enumerate() {
        if KEYWORDS.contains(&arg.as_ref()) || arg == "input" {
            panic!("Invalid : keyword used as identifier")
        }
        else {
            if args_set.contains(arg) {
                panic!("Invalid : duplicate argument name");
            } else if arg == "input" {
                    panic!("Invalid : input used as argument name")
            }else {
                    env.insert(arg.clone(), Loc::LStack(-1 * (i as i64 + 1)));
                    args_set.insert(arg.clone());
            }
        }
    }
    let mut context = Context {
        si: 2,
        env: &env,
        brake: "",
        target: Loc::LReg(Reg::RAX),
        user_defined_functions,
        current_function: &*("fun_".to_owned() + &defn.name.clone())
    };
    let mut label = 0;
    let mut body = compile_to_instrs(&defn.body, &mut context, &mut label);
    if body.len() ==0 {
        panic!("Invalid : empty function body");
    }
    instrs.append(&mut body);
    instrs.push(Instr::IRet);
    return instrs;
}


fn compile_to_instrs(expression: &Expr, context: &Context, label_count: &mut i32) -> Vec<Instr> {
    return match expression {
        Expr::Number(n) => {
            compile_num(n, context)
        },
        Expr::Boolean(b) => {
            compile_boolean(b, context)
        },
        Expr::Id(id) => {
            compile_identifier(id, context)
        },
        Expr::Let(bindings, e) => {
            compile_bindings(bindings, e, context, label_count)
        },
        Expr::UnOp(op, e) => {
            compile_unary_operation(op, e, context, label_count)
        },
        Expr::BinOp(op, e1, e2) => {
            compile_binary_operation(op, e1, e2, context, label_count)
        },
        Expr::Block(exprs) => {
            compile_block_instruction(exprs, context, label_count)
        },
        Expr::Set(id, e) => {
            compile_set_instruction(id, e, context, label_count)
        },
        Expr::Break(e) => {
            compile_break_instruction(e, context, label_count)
        },
        Expr::If(cond, e1, e2) => {
            compile_if_block_instruction(cond, e1, e2, context, label_count)
        },
        Expr::Loop(e) => {
            compile_loop_instructions(e, context, label_count)
        },
        Expr::Call(id, args) => {
            compile_function_call_instruction(id, args, context, label_count)
        },
        Expr::Array(exprs) => {
            compile_array_instruction(exprs, context, label_count)
        },
        Expr::GetIndex(id, e) => {
            compile_get_index_instruction(id, e, context, label_count)
        },
        Expr::SetIndex(id, e1, e2) => {
            compile_set_index_instruction(id, e1, e2, context, label_count)
        },
        Expr::Len(id) => {
            compile_len_instruction(id, context, label_count)
        },
        Expr::Append(id, e) => {
            compile_append_instruction(id, e, context, label_count)
        },
    }
}


fn compile_num(n: &i64, c : &Context) -> Vec<Instr> {
    let mut instrs = Vec::new();
          let num = *n;
          if num < MIN  || num > MAX{
            panic!("Invalid: overflow");
          }
            instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VImm(num<<1)));
          instrs.extend(mov_target(&c.target, &Val::VReg(Reg::RAX)));
          instrs
}
fn compile_boolean(b: &bool, c: &Context) -> Vec<Instr> {
    let mut instrs = Vec::new();
    if *b {
        instrs.extend(mov_target(&c.target, &Val::VImm(TRUE)));
    } else {
        instrs.extend(mov_target(&c.target, &Val::VImm(FALSE)));
    }
    instrs
}
fn compile_identifier(id: &String, c: &Context) -> Vec<Instr> {
    if id == "input" {
        if c.current_function != "main" {
            panic!("invalid command line input is only allowed in main function");
        }
        return mov_target(&c.target, &Val::VReg(Reg::RDI))
    } else {
        if c.env.contains_key(id) {
                match c.env.get(id).unwrap() {
                    Loc::LReg(r) => mov_target(&c.target, &Val::VReg(*r)),
                    Loc::LStack(offset) => {
                        mov_target(&c.target, &Val::VStack(*offset))
                    },
                    Loc::LHeap(offset) => {
                        mov_target(&c.target, &Val::VHeap(*offset))
                    },
                    Loc::LAddr(addr) => {
                        mov_target(&c.target, &Val::VAddr(*addr))
                    }
                }
        } else {
                panic!("Invalid : Unbound variable identifier {}", id);
        }
    }

}

fn compile_bindings(bindings: &Vec<(String, Expr)>,
                    bound_expr: &Box<Expr>,
                    context: &Context,
                    label_count: &mut i32) -> Vec<Instr> {
    let mut instrs = vec![];
    let mut current_ids = HashSet::new();
    let mut si_new = context.si;
    let mut env_new = context.env.clone();
    if bindings.len() < 1 {
        panic!("Invalid");
    }

    for (var_id, value) in bindings.iter() {
        if current_ids.contains(var_id) {
            panic!("Invalid : Duplicate binding");
        } else {
            current_ids.insert(var_id);
            let binding_expr = compile_to_instrs(value,
                                                 &Context {
                                                     target: Loc::LStack(si_new),
                                                     env: &env_new,
                                                     si: si_new + 1,
                                                     ..*context
                                                 },
                                                 label_count);
            env_new = env_new.update(var_id.to_string(), Loc::LStack(si_new));
            instrs.extend(binding_expr);
            si_new = si_new + 1;
        }
    }
    instrs.extend(compile_to_instrs(bound_expr,
                                    &Context { si: si_new, env: &env_new, ..*context },
                                    label_count));
    return instrs;
}

fn compile_unary_operation(op: &Op1,
                           expression: &Box<Expr>,
                           context: &Context,
                           label_count: &mut i32) -> Vec<Instr> {
    let mut instrs = compile_to_instrs(expression,
                                       &Context{
                                           target: Loc::LReg(Reg::RAX),
                                           ..*context
                                       },
                                       label_count);
    instrs.extend(mov_target(&context.target, &Val::VReg(Reg::RAX)));
    match op {
        Op1::Add1 => {
            instrs.extend(check_dtype_num_single(&Val::VReg(Reg::RAX)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_OVERFLOW)));
            instrs.push(Instr::IAdd(Val::VReg(Reg::RAX), Val::VImm(VALUE_1)));
            instrs.push(Instr::IJo(ROUTINE_ERROR.to_string()));
        },
        Op1::Sub1 => {
            instrs.extend(check_dtype_num_single(&Val::VReg(Reg::RAX)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_OVERFLOW)));
            instrs.push(Instr::ISub(Val::VReg(Reg::RAX), Val::VImm(VALUE_1)));
            instrs.push(Instr::IJo(ROUTINE_ERROR.to_string()));
        },
        Op1::IsBool => {
            instrs.push(Instr::ITest(Val::VReg(Reg::RAX), Val::VImm(1)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VImm(TRUE)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(FALSE)));
            instrs.push(Instr::ICmove(Val::VReg(Reg::RAX), Val::VReg(Reg::RBX)));
        },
        Op1::IsNum => {
            instrs.push(Instr::ITest(Val::VReg(Reg::RAX), Val::VImm(1)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VImm(FALSE)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(TRUE)));
            instrs.push(Instr::ICmove(Val::VReg(Reg::RAX), Val::VReg(Reg::RBX)));
        },
        Op1::Print => {
            let offset;
            if context.si % 2 == 0{
                offset = (context.si + 1) * OFFSET_SCALE;
            } else {
                offset = (context.si) * OFFSET_SCALE;
            }

            instrs.extend(mov_target(&Loc::LStack(context.si), &Val::VReg(Reg::RDI)));
            instrs.push(Instr::ISub(Val::VReg(Reg::RSP), Val::VImm(offset)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RDI), &Val::VReg(Reg::RAX)));
            instrs.push(Instr::ICall(ROUTINE_PRINT.to_string()));
            instrs.push(Instr::IAdd(Val::VReg(Reg::RSP), Val::VImm(offset)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RDI), &Val::VStack(context.si)))
        },
    }
    instrs.extend(mov_target(&context.target, &Val::VReg(Reg::RAX)));
    instrs
}

fn compile_binary_operation(op: &Op2,
                            e1: &Box<Expr>,
                            e2: &Box<Expr>,
                            context: &Context,
                            label_count: &mut i32) -> Vec<Instr> {
    let save_e1_ctxt = Context { target: Loc::LReg(Reg::RAX), ..*context };
    let e1_instrs = compile_to_instrs(e1, &save_e1_ctxt, label_count);
    let e2_ctxt = Context {
        si: context.si + 1,
        target: Loc::LReg(Reg::RAX),
        ..*context
    };
    let stack_offset = context.si;
    let mut instrs = vec![];
    instrs.extend(e1_instrs);
    instrs.extend(mov_target(&Loc::LStack(context.si), &Val::VReg(Reg::RAX)));
    instrs.extend(compile_to_instrs(e2, &e2_ctxt, label_count));
    match op {
        Op2::Plus => {
            instrs.extend(check_dtype_num(&Val::VReg(Reg::RAX), &Val::VStack(stack_offset)));
            instrs.push(Instr::IAdd(Val::VReg(Reg::RAX), Val::VStack(stack_offset)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_OVERFLOW)));
            instrs.push(Instr::IJo(ROUTINE_ERROR.to_string()));
        },
        Op2::Minus => {
            instrs.extend(check_dtype_num(&Val::VReg(Reg::RAX), &Val::VStack(stack_offset)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VStack(stack_offset)));
            instrs.push(Instr::ISub(Val::VReg(Reg::RBX), Val::VReg(Reg::RAX)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VReg(Reg::RBX)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_OVERFLOW)));
            instrs.push(Instr::IJo(ROUTINE_ERROR.to_string()));
        },
        Op2::Times => {
            instrs.extend(check_dtype_num(&Val::VReg(Reg::RAX), &Val::VStack(stack_offset)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_OVERFLOW)));
            instrs.push(Instr::ISar(Val::VReg(Reg::RAX), Val::VImm(1)));
            instrs.push(Instr::IMul(Val::VReg(Reg::RAX), Val::VStack(stack_offset)));
            instrs.push(Instr::IJo(ROUTINE_ERROR.to_string()));
        },
        Op2::Less => {
            instrs.extend(check_dtype_num(&Val::VReg(Reg::RAX), &Val::VStack(stack_offset)));
            let jump_label = new_label(label_count, "less_than", &context.current_function.clone());
            let end_label = new_label(label_count, "end_less_than", &context.current_function.clone());
            instrs.push(Instr::ICmp(Val::VStack(stack_offset), Val::VReg(Reg::RAX)));
            instrs.push(Instr::IJl(jump_label.clone()));
            instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VImm(FALSE)));
            instrs.push(Instr::IJmp(end_label.clone()));
            instrs.push(Instr::ILabel(jump_label));
            instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VImm(TRUE)));
            instrs.push(Instr::ILabel(end_label));
        },
        Op2::Equal => {
            let boolean_check = new_label(label_count, "boolean_check", &context.current_function.clone());
            let end_check = new_label(label_count, "end_check", &context.current_function.clone());
            instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_INVALID_ARGUMENT)));
            instrs.push(Instr::ITest(Val::VReg(Reg::RAX), Val::VImm(1)));
            instrs.push(Instr::IJnz(boolean_check.clone()));
            instrs.push(Instr::ITest(Val::VStack(stack_offset), Val::VImm(1)));
            instrs.push(Instr::IJnz(ROUTINE_ERROR.to_string()));
            instrs.push(Instr::IJmp(end_check.clone()));

            instrs.push(Instr::ILabel(boolean_check.clone()));
            instrs.push(Instr::ITest(Val::VStack(stack_offset), Val::VImm(1)));
            instrs.push(Instr::IJz(ROUTINE_ERROR.to_string()));
            instrs.push(Instr::ILabel(end_check.clone()));

            instrs.push(Instr::ICmp(Val::VReg(Reg::RAX), Val::VStack(stack_offset)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VImm(FALSE)));
            instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(TRUE)));
            instrs.push(Instr::ICmove(Val::VReg(Reg::RAX), Val::VReg(Reg::RBX)));
        },
        Op2::Greater => {
            instrs.extend(check_dtype_num(&Val::VReg(Reg::RAX), &Val::VStack(stack_offset)));
            let jump_label = new_label(label_count, "greater_than", &context.current_function.clone());
            let end_label = new_label(label_count, "end_greater_than", &context.current_function.clone());
            instrs.push(Instr::ICmp(Val::VStack(stack_offset), Val::VReg(Reg::RAX)));
            instrs.push(Instr::IJg(jump_label.clone()));
            instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VImm(FALSE)));
            instrs.push(Instr::IJmp(end_label.clone()));
            instrs.push(Instr::ILabel(jump_label));
            instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VImm(TRUE)));
            instrs.push(Instr::ILabel(end_label));
        },
        Op2::GreaterEqual => {
            instrs.extend(check_dtype_num(&Val::VReg(Reg::RAX), &Val::VStack(stack_offset)));
            let jump_label = new_label(label_count, "greater_equal_than", &context.current_function.clone());
            let end_label = new_label(label_count, "end_greater_equal_than", &context.current_function.clone());
            instrs.push(Instr::ICmp(Val::VStack(stack_offset), Val::VReg(Reg::RAX)));
            instrs.push(Instr::IJge(jump_label.clone()));
            instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VImm(FALSE)));
            instrs.push(Instr::IJmp(end_label.clone()));
            instrs.push(Instr::ILabel(jump_label));
            instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VImm(TRUE)));
            instrs.push(Instr::ILabel(end_label));
        }
        Op2::LessEqual => {
            instrs.extend(check_dtype_num(&Val::VReg(Reg::RAX), &Val::VStack(stack_offset)));
            let jump_label = new_label(label_count, "less_equal_than", &context.current_function.clone());
            let end_label = new_label(label_count, "end_less_equal_than", &context.current_function.clone());
            instrs.push(Instr::ICmp(Val::VStack(stack_offset), Val::VReg(Reg::RAX)));
            instrs.push(Instr::IJle(jump_label.clone()));
            instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VImm(FALSE)));
            instrs.push(Instr::IJmp(end_label.clone()));
            instrs.push(Instr::ILabel(jump_label));
            instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VImm(TRUE)));
            instrs.push(Instr::ILabel(end_label));
        }
    }
    instrs.extend(mov_target(&context.target, &Val::VReg(Reg::RAX)));
    return instrs;
}

fn compile_set_instruction(id: &str, expression: &Expr, context: &Context, label_count: &mut i32) -> Vec<Instr> {
    let mut instrs = vec![];
    if context.env.contains_key(id) {
        let variable_loc = context.env.get(id).unwrap();
        let val_ctxt = Context { target: *variable_loc, si: context.si+1, ..*context };
        instrs.extend(compile_to_instrs(expression, &val_ctxt, label_count));
        match variable_loc {
            Loc::LStack(offset) => {
                instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VStack(*offset)));
            },
            Loc::LReg(reg) => {
                instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VReg(*reg)));
            }
            Loc::LHeap(offset) => {
                instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VHeap(*offset)));
            },
            Loc::LAddr(reg) => {
                instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VAddr(*reg)));
            }
        }
    } else {
        panic!("Invalid : Unbound variable identifier {}", id);
    }
    return instrs;
}

fn compile_block_instruction(exprs: &Vec<Expr>, context: &Context, label_count: &mut i32) -> Vec<Instr> {
    let mut instrs = vec![];
    for e in exprs {
        instrs.extend(compile_to_instrs(e, context, label_count));
    }
    return instrs;
}

fn compile_break_instruction(expr: &Box<Expr>, context: &Context, label_count: &mut i32) -> Vec<Instr> {
    let label_parts: Vec<&str> = context.brake.split("_").collect();
    if label_parts.len() > 1 && *label_parts[label_parts.len() - 2].to_string() == "loopend".to_string() {
        let nctxt = Context { target: Loc::LReg(Reg::RAX), ..*context };
        let mut instrs = compile_to_instrs(expr, &nctxt, label_count);
        instrs.push(Instr::IJmp(context.brake.to_string().to_owned()));
        return instrs;
    }
    else {
        panic!("Invalid : break outside of loop");
    }
}

fn compile_if_block_instruction(condition: &Box<Expr>,
                                then_block: &Box<Expr>,
                                else_block: &Box<Expr>,
                                context: &Context,
                                label_count: &mut i32) -> Vec<Instr> {
    let mut instrs = vec![];
    let end_label = new_label(label_count, "ifend", &context.current_function.clone());
    let else_label = new_label(label_count, "ifelse", &context.current_function.clone());
    let cond_ctxt = Context { target: Loc::LReg(Reg::RAX), ..*context };
    let cond_instrs = compile_to_instrs(condition, &cond_ctxt, label_count);
    let thn_instrs = compile_to_instrs(then_block, context, label_count);
    let els_instrs = compile_to_instrs(else_block, context, label_count);

    instrs.extend(cond_instrs);
    instrs.push(Instr::ICmp(Val::VReg(Reg::RAX), Val::VImm(FALSE)));
    instrs.push(Instr::IJe(else_label.to_string()));
    instrs.extend(thn_instrs);
    instrs.push(Instr::IJmp(end_label.to_string()));
    instrs.push(Instr::ILabel(else_label.to_string()));
    instrs.extend(els_instrs);
    instrs.push(Instr::ILabel(end_label.to_string()));
    instrs
}

fn compile_loop_instructions(body: &Box<Expr>,
                             context: &Context,
                             label_count: &mut i32) -> Vec<Instr> {
    let mut instrs = vec![];
    let start_label = new_label(label_count, "loop", &context.current_function.clone());
    let end_label = new_label(label_count, "loopend", &context.current_function.clone());
    let loop_ctxt = Context { brake: &end_label.to_string(), ..*context };
    let loop_instrs = compile_to_instrs(body, &loop_ctxt, label_count);


    instrs.push(Instr::ILabel(start_label.to_string()));
    instrs.extend(loop_instrs);
    instrs.push(Instr::IJmp(start_label.to_string()));
    instrs.push(Instr::ILabel(end_label.to_string()));
    return instrs;
}

fn compile_function_call_instruction(id: &String,
                                     args: &Vec<Expr>,
                                     context: &Context,
                                     label_count: &mut i32) -> Vec<Instr> {
    let mut instrs = vec![];
    // make sure just before the call instruction the rsp si is a odd multiple of 8
    // This ensure that the stack pointer is a multiple of 16 after the call instruction
    let required_args_count = match context.user_defined_functions.get(id) {
        Some(f) => *f,
        None => panic!("Invalid : Undefined function {}", id),
    };

    if args.len() as i64 == required_args_count {
        let mut offset = context.si + required_args_count + 1;
        if offset % 2 == 0 {
            offset += 1;
        }
        let mut stack_offset = offset;

        for (index, arg) in args.iter().enumerate() {
            let index_to_write = stack_offset + -1 * (index as i64);
            let arg_ctxt = Context { target: Loc::LStack(index_to_write), si: stack_offset + 1 + (index as i64), ..*context };
            instrs.extend(compile_to_instrs(arg, &arg_ctxt, label_count));
        }
        stack_offset *= OFFSET_SCALE;

        instrs.push(Instr::ISub(Val::VReg(Reg::RSP), Val::VImm(stack_offset)));
        instrs.extend(mov_target(&Loc::LStack(-1 * (offset - 1)), &Val::VReg(Reg::RDI)));
        instrs.push(Instr::ICall(id.to_string()));
        instrs.extend(mov_target(&context.target, &Val::VReg(Reg::RAX)));
        instrs.extend(mov_target(&Loc::LReg(Reg::RDI), &Val::VStack(-1 * (offset - 1))));
        instrs.push(Instr::IAdd(Val::VReg(Reg::RSP), Val::VImm(stack_offset)));
        instrs.extend(mov_target(&context.target, &Val::VReg(Reg::RAX)));
        instrs
    } else {
        panic!("Invalid insufficient number of arguments passed to the function {}-> expected {} but got {}.",
               id, required_args_count, args.len());
    }
}

fn compile_array_instruction(exprs: &Vec<Expr>, context: &Context, label_count: &mut i32) -> Vec<Instr> {
    // the first element of the array is the size of the array
    // the rest of the elements are the elements of the array

    let mut instrs = vec![];
    let mut heap_index = 0;

    let mut stack_offset = context.si;
    for expr in exprs {
        let expr_instrs = compile_to_instrs(expr,
                                                &Context {
                                                    target: Loc::LStack(stack_offset),
                                                    si: stack_offset,
                                                    ..*context
                                                },
                                                label_count);
        stack_offset += 1;
        instrs.extend(expr_instrs);
    }
    let size = exprs.len() as i64;
    instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VImm(size << 1)));
    instrs.extend(mov_target(&Loc::LHeap(heap_index), &Val::VReg(Reg::RAX)));
    heap_index += 1;
    for offset in context.si..stack_offset {
        instrs.extend(mov_target(&Loc::LHeap(heap_index), &Val::VStack(offset)));
        heap_index += 1;
    }

    let heap_offset = heap_index * OFFSET_SCALE;
    instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VReg(Reg::R15)));
    instrs.push(Instr::IAdd(Val::VReg(Reg::RAX), Val::VImm(1)));
    instrs.extend(mov_target(&context.target, &Val::VReg(Reg::RAX)));
    instrs.push(Instr::IAdd(Val::VReg(Reg::R15), Val::VImm(heap_offset)));

    return instrs;
}
fn compile_get_index_instruction(array: &Box<Expr>,
                                  index: &Box<Expr>,
                                  context: &Context,
                                  label_count: &mut i32) -> Vec<Instr> {
    let mut instrs = vec![];
    let array_instrs = compile_to_instrs(array, &Context {target: Loc::LReg(Reg::RAX), ..*context}, label_count);
    let index_instrs = compile_to_instrs(index, &Context {target: Loc::LReg(Reg::RAX), si: context.si+1, ..*context}, label_count);
    instrs.extend(array_instrs);

    //checking index out of bound
    instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VReg(Reg::RAX)));
    instrs.push(Instr::IAnd(Val::VReg(Reg::RBX), Val::VImm(3)));
    instrs.push(Instr::ICmp(Val::VReg(Reg::RBX), Val::VImm(1)));
    instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_NOT_AN_ARRAY)));
    instrs.push(Instr::IJne("throw_error".to_string()));

    instrs.push(Instr::ISub(Val::VReg(Reg::RAX), Val::VImm(1)));
    instrs.extend(mov_target(&Loc::LStack(context.si), &Val::VReg(Reg::RAX)));
    instrs.extend(index_instrs);
    instrs.extend(mov_target(&Loc::LReg(Reg::RCX), &Val::VStack(context.si)));
    instrs.push(Instr::ICmp(Val::VReg(Reg::RAX), Val::VAddr(Reg::RCX)));
    instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_INDEX_OUT_OF_BOUNDS)));
    instrs.push(Instr::IJge("throw_error".to_string()));
    instrs.push(Instr::ISar(Val::VReg(Reg::RAX), Val::VImm(1)));
    instrs.push(Instr::IAdd(Val::VReg(Reg::RAX), Val::VImm(1)));
    instrs.push(Instr::IMul(Val::VReg(Reg::RAX), Val::VImm(OFFSET_SCALE)));
    instrs.push(Instr::IAdd(Val::VReg(Reg::RCX), Val::VReg(Reg::RAX)));
    instrs.extend(mov_target(&context.target, &Val::VAddr(Reg::RCX)));

    return instrs;
}

fn compile_set_index_instruction(array: &Box<Expr>,
                                  index: &Box<Expr>,
                                  value: &Box<Expr>,
                                  context: &Context,
                                  label_count: &mut i32) -> Vec<Instr> {
    let mut instrs = vec![];
    let array_instrs = compile_to_instrs(array, &Context { target: Loc::LReg(Reg::RAX), ..*context }, label_count);
    let index_instrs = compile_to_instrs(index, &Context { target: Loc::LReg(Reg::RAX), si: context.si + 1, ..*context }, label_count);
    let value_instrs = compile_to_instrs(value, &Context { target: Loc::LReg(Reg::RAX), si: context.si + 2, ..*context }, label_count);
    instrs.extend(array_instrs);

    //checking index out of bound
    instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VReg(Reg::RAX)));
    instrs.push(Instr::IAnd(Val::VReg(Reg::RBX), Val::VImm(3)));
    instrs.push(Instr::ICmp(Val::VReg(Reg::RBX), Val::VImm(1)));
    instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_NOT_AN_ARRAY)));
    instrs.push(Instr::IJne("throw_error".to_string()));

    instrs.push(Instr::ISub(Val::VReg(Reg::RAX), Val::VImm(1)));
    instrs.extend(mov_target(&Loc::LStack(context.si), &Val::VReg(Reg::RAX)));
    instrs.extend(index_instrs);
    instrs.extend(mov_target(&Loc::LReg(Reg::RCX), &Val::VStack(context.si)));
    instrs.push(Instr::ICmp(Val::VReg(Reg::RAX), Val::VAddr(Reg::RCX)));
    instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_INDEX_OUT_OF_BOUNDS)));
    instrs.push(Instr::IJge("throw_error".to_string()));
    instrs.push(Instr::ISar(Val::VReg(Reg::RAX), Val::VImm(1)));
    instrs.push(Instr::IAdd(Val::VReg(Reg::RAX), Val::VImm(1)));
    instrs.push(Instr::IMul(Val::VReg(Reg::RAX), Val::VImm(OFFSET_SCALE)));
    instrs.push(Instr::IAdd(Val::VReg(Reg::RCX), Val::VReg(Reg::RAX)));
    instrs.extend(value_instrs);
    instrs.extend(mov_target(&Loc::LAddr(Reg::RCX), &Val::VReg(Reg::RAX)));
    instrs
}

fn compile_len_instruction(array: &Box<Expr>,
                           context: &Context,
                           label_count: &mut i32) -> Vec<Instr> {
    let mut instrs = vec![];
    let array_instrs = compile_to_instrs(array, &Context { target: Loc::LReg(Reg::RAX), ..*context }, label_count);
    instrs.extend(array_instrs);

    instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VReg(Reg::RAX)));
    instrs.push(Instr::IAnd(Val::VReg(Reg::RBX), Val::VImm(3)));
    instrs.push(Instr::ICmp(Val::VReg(Reg::RBX), Val::VImm(1)));
    instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_NOT_AN_ARRAY)));
    instrs.push(Instr::IJne("throw_error".to_string()));

    instrs.push(Instr::ISub(Val::VReg(Reg::RAX), Val::VImm(1)));
    instrs.extend(mov_target(&context.target, &Val::VAddr(Reg::RAX)));

    return instrs;
}

fn compile_append_instruction(array: &Box<Expr>,
                              value: &Box<Expr>,
                              context: &Context,
                              label_count: &mut i32) -> Vec<Instr> {
    let mut instrs = vec![];
    let array_instrs = compile_to_instrs(array, &Context { target: Loc::LReg(Reg::RAX), ..*context }, label_count);
    let value_instrs = compile_to_instrs(value, &Context { target: Loc::LReg(Reg::RAX), si: context.si + 1, ..*context }, label_count);
    let copy_label_start = new_label(label_count, "copy_start", context.current_function);
    let copy_label_end = new_label(label_count, "copy_end", context.current_function);
    instrs.extend(array_instrs);

    instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VReg(Reg::RAX)));
    instrs.push(Instr::IAnd(Val::VReg(Reg::RBX), Val::VImm(3)));
    instrs.push(Instr::ICmp(Val::VReg(Reg::RBX), Val::VImm(1)));
    instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_NOT_AN_ARRAY)));
    instrs.push(Instr::IJne("throw_error".to_string()));

    instrs.extend(mov_target(&Loc::LStack(context.si), &Val::VReg(Reg::RAX)));
    instrs.extend(value_instrs);
    instrs.extend(mov_target(&Loc::LStack(context.si + 1), &Val::VReg(Reg::RAX)));
    instrs.extend(mov_target(&Loc::LReg(Reg::RCX), &Val::VStack(context.si)));
    instrs.push(Instr::ISub(Val::VReg(Reg::RCX), Val::VImm(1)));
    instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VAddr(Reg::RCX)));
    instrs.extend(mov_target(&Loc::LReg(Reg::RCX), &Val::VReg(Reg::RAX)));
    instrs.push(Instr::IAdd(Val::VReg(Reg::RCX), Val::VImm(2)));
    instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VReg(Reg::R15)));
    instrs.extend(mov_target(&Loc::LAddr(Reg::RBX), &Val::VReg(Reg::RCX)));
    instrs.push(Instr::IAdd(Val::VReg(Reg::RBX), Val::VImm(OFFSET_SCALE)));

    instrs.extend(mov_target(&Loc::LReg(Reg::RCX), &Val::VStack(context.si)));
    instrs.push(Instr::ISub(Val::VReg(Reg::RCX), Val::VImm(1)));


    instrs.push(Instr::ILabel(copy_label_start.clone()));
    instrs.push(Instr::IAdd(Val::VReg(Reg::RCX), Val::VImm(OFFSET_SCALE)));
    instrs.extend(mov_target(&Loc::LAddr(Reg::RBX), &Val::VAddr(Reg::RCX)));
    instrs.push(Instr::IAdd(Val::VReg(Reg::RBX), Val::VImm(OFFSET_SCALE)));
    instrs.push(Instr::ISub(Val::VReg(Reg::RAX), Val::VImm(2)));
    instrs.push(Instr::ICmp(Val::VReg(Reg::RAX), Val::VImm(0)));
    instrs.push(Instr::IJe(copy_label_end.clone()));
    instrs.push(Instr::IJmp(copy_label_start.clone()));
    instrs.push(Instr::ILabel(copy_label_end.clone()));

    instrs.extend(mov_target(&Loc::LAddr(Reg::RBX), &Val::VStack(context.si + 1)));
    instrs.extend(mov_target(&Loc::LReg(Reg::RAX), &Val::VReg(Reg::R15)));
    instrs.push(Instr::IAdd(Val::VReg(Reg::RAX), Val::VImm(1)));
    instrs.extend(mov_target(&context.target, &Val::VReg(Reg::RAX)));
    instrs.extend(mov_target(&Loc::LReg(Reg::R15), &Val::VReg(Reg::RBX)));
    instrs.push(Instr::IAdd(Val::VReg(Reg::R15), Val::VImm(OFFSET_SCALE)));

    return instrs;
}