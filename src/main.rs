use std::env;
use std::fs::File;
use std::io::prelude::*;
use sexp::*;

use snek::utils::*;
use snek::parser::*;
use snek::compiler::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name =  &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;
    let prog = "(".to_owned() + &in_contents + ")";

    let sexp = match parse(&prog) {
      Ok(sexp) => sexp,
      Err(e) => panic!("Invalid: {}", e),
    };
    let prog = parse_program(&sexp);
    println!("{:?}", prog);

    let instrs = compile_program(&prog);

    let result = instrs
        .iter()
        .map(|i| instr_to_str(i))
        .collect::<Vec<String>>()
        .join("\n");

    let asm_program = format!(
        "
section .text
global our_code_starts_here
extern snek_error
extern snek_print
extern snek_structural_equality
throw_error:
  mov rdi, rbx
  push rsp
  call snek_error
  ret
our_code_starts_here:
mov r15, rsi
{}
",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}