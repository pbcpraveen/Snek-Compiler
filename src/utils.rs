use crate::constants::*;
use sexp::Atom::*;
use sexp::*;



pub fn new_label(l: &mut i32, s: &str, function_name: &str) -> String {
  let current = *l;
  *l += 1;
  format!("{function_name}_{s}_{current}")
}

pub fn check_dtype_num(operand1: &Val, operand2: &Val) -> Vec<Instr> {
  let mut instrs = Vec::new();
      instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_INVALID_ARGUMENT)));
      instrs.extend(mov_target(&Loc::LReg(Reg::RCX), &operand1.clone()));
      instrs.push(Instr::IAnd(Val::VReg(Reg::RCX), Val::VImm(1)));
      instrs.push(Instr::ITest(Val::VReg(Reg::RCX), Val::VImm(1)));
      instrs.push(Instr::IJne("throw_error".to_string()));
      instrs.extend(mov_target(&Loc::LReg(Reg::RCX), &operand2.clone()));
      instrs.push(Instr::IAnd(Val::VReg(Reg::RCX), Val::VImm(1)));
      instrs.push(Instr::ITest(Val::VReg(Reg::RCX), Val::VImm(1)));
      instrs.push(Instr::IJne("throw_error".to_string()));

      instrs
}

pub fn check_dtype_num_single(operand1: &Val) -> Vec<Instr> {
  let mut instrs = Vec::new();
  instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_INVALID_ARGUMENT)));
  instrs.push(Instr::ITest(operand1.clone(), Val::VImm(1)));
  instrs.push(Instr::IJne("throw_error".to_string()));
  instrs
}

pub fn check_dtype_index(operand1: &Val) -> Vec<Instr> {
  let mut instrs = Vec::new();
  instrs.extend(mov_target(&Loc::LReg(Reg::RBX), &Val::VImm(ERROR_INDEX_NOT_NUMBER)));
  instrs.push(Instr::ITest(operand1.clone(), Val::VImm(1)));
  instrs.push(Instr::IJne("throw_error".to_string()));
  instrs
}

pub fn instr_to_str(i: &Instr) -> String {
    match i {
        Instr::IMov(d, s) => format!("mov {}, {}", loc_to_str(d), val_to_str(s)),
        Instr::IAdd(d, s) => format!("add {}, {}", val_to_str(d), val_to_str(s)),
        Instr::ISub(d, s) => format!("sub {}, {}", val_to_str(d), val_to_str(s)),
        Instr::IMul(d, s) => format!("imul {}, {}", val_to_str(d), val_to_str(s)),
        Instr::ICmp(d, s) => format!("cmp {}, {}", val_to_str(d), val_to_str(s)),
        Instr::IJmp(l) => format!("jmp {}", l),
        Instr::IJe(l) => format!("je {}", l),
        Instr::IJne(l) => format!("jne {}", l),
        Instr::IJg(l) => format!("jg {}", l),
        Instr::IJge(l) => format!("jge {}", l),
        Instr::IJl(l) => format!("jl {}", l),
        Instr::IJle(l) => format!("jle {}", l),
        Instr::ILabel(l) => format!("{}:", l),
        Instr::IPush(v) => format!("push {}", val_to_str(v)),
        Instr::IPop(l) => format!("pop {}", val_to_str(l)),
        Instr::IAnd(d, s) => format!("and {}, {}", val_to_str(d), val_to_str(s)),
        Instr::IXor(d, s) => format!("xor {}, {}", val_to_str(d), val_to_str(s)),
        Instr::ITest(d, s) => format!("test qword {}, {}", val_to_str(d), val_to_str(s)),
        Instr::ISar(d, s) => format!("sar {}, {}", val_to_str(d), val_to_str(s)),
        Instr::IJnz(l) => format!("jnz {}", l),
        Instr::IJz(l) => format!("jz {}", l),
        Instr::ICmove(d, s) => format!("cmove {}, {}", val_to_str(d), val_to_str(s)),
        Instr::ICmovne(d, s) => format!("cmovne {}, {}", val_to_str(d), val_to_str(s)),
        Instr::ICmovg(d, s) => format!("cmovg {}, {}", val_to_str(d), val_to_str(s)),
        Instr::INot(d) => format!("not {}", val_to_str(d)),
        Instr::ISal(d, s) => format!("sal {}, {}", val_to_str(d), val_to_str(s)),
        Instr::IShr(d, s) => format!("shr {}, {}", val_to_str(d), val_to_str(s)),
        Instr::IJo(l) => format!("jo {}", l),
        Instr::IRet => format!("ret"),
        Instr::ICall(l) => format!("call {}", l),
      }
}

fn loc_to_str(l: &Loc) -> String {
    match l {
        Loc::LReg(r) => reg_to_str(r),
        Loc::LStack(o) => {
          let offset = o * OFFSET_SCALE;
            if offset >= 0 {
                format!("qword [rsp - {}]", offset)
            } else {
                format!("qword [rsp + {}]", -1 * offset)
            }
        },
        Loc::LHeap(o) => {
            let offset = o * OFFSET_SCALE;
            format!("qword [r15 + {}]", offset)
        },
        Loc::LAddr(a) => format!("qword [{}]", reg_to_str(a)),
    }
}
pub fn mov_target(dest : &Loc, source : &Val) -> Vec<Instr> {
  let mut instrs = Vec::new();
  match (dest, source) {
      (Loc::LStack(n), Val::VStack(_m)) | (Loc::LStack(n), Val::VHeap(_m)) => {
          instrs.push(Instr::IMov(Loc::LReg(Reg::RDX), source.clone()));
          instrs.push(Instr::IMov(Loc::LStack(*n), Val::VReg(Reg::RDX)));
        },
      (Loc::LStack(n), Val::VAddr(_m)) => {
          instrs.push(Instr::IMov(Loc::LReg(Reg::RDX), source.clone()));
          instrs.push(Instr::IMov(Loc::LStack(*n), Val::VReg(Reg::RDX)));
        },
      (Loc::LHeap(n), Val::VHeap(_m)) | (Loc::LHeap(n), Val::VStack(_m)) => {
          instrs.push(Instr::IMov(Loc::LReg(Reg::RDX), source.clone()));
          instrs.push(Instr::IMov(Loc::LHeap(*n), Val::VReg(Reg::RDX)));
        },
      (Loc::LHeap(n), Val::VAddr(_m)) => {
          instrs.push(Instr::IMov(Loc::LReg(Reg::RDX), source.clone()));
          instrs.push(Instr::IMov(Loc::LHeap(*n), Val::VReg(Reg::RDX)));
        },
      (Loc::LAddr(r1), Val::VStack(_m)) | (Loc::LAddr(r1), Val::VHeap(_m))  => {
          instrs.push(Instr::IMov(Loc::LReg(Reg::RDX), source.clone()));
          instrs.push(Instr::IMov(Loc::LAddr(*r1), Val::VReg(Reg::RDX)));
        },
      (Loc::LAddr(r1), Val::VAddr(_m)) => {
          instrs.push(Instr::IMov(Loc::LReg(Reg::RDX), source.clone()));
          instrs.push(Instr::IMov(Loc::LAddr(*r1), Val::VReg(Reg::RDX)));
        },
      (Loc::LReg(r1), _) => {
          instrs.push(Instr::IMov(Loc::LReg(*r1), source.clone()));
        },
      (Loc::LStack(n), _) => {
          instrs.push(Instr::IMov(Loc::LStack(*n), source.clone()));
        },
      (Loc::LHeap(n), _) => {
          instrs.push(Instr::IMov(Loc::LHeap(*n), source.clone()));
        },
      (Loc::LAddr(r1), _) => {
          instrs.push(Instr::IMov(Loc::LAddr(*r1), source.clone()));
        },
  }
  instrs
}

fn val_to_str(v: &Val) -> String {
    match v {
        Val::VImm(n) => n.to_string(),
        Val::VReg(r) => reg_to_str(r),
        Val::VStack(o) => {
            let offset = o * OFFSET_SCALE;
            if offset >= 0 {
                format!("qword [rsp - {}]", offset)
            } else {
                format!("qword [rsp + {}]", -1 * offset)
            }
        },
        Val::VHeap(o) => {
            let offset = o * OFFSET_SCALE;
            format!("qword [r15 + {}]", offset)
        },
        Val::VAddr(l) => {
            format!("qword [{}]", reg_to_str(l))
        }
    }
}

fn reg_to_str(r: &Reg) -> String {
    match r {
        Reg::RAX => "rax".to_string(),
        Reg::RSP => "rsp".to_string(),
        Reg::RBX => "rbx".to_string(),
        Reg::RDI => "rdi".to_string(),
        Reg::RCX => "rcx".to_string(),
        Reg::RDX => "rdx".to_string(),
        Reg::R15 => "r15".to_string(),
    }
}
pub fn is_def(s: &Sexp) -> bool {
    match s {
        Sexp::List(def_vec) => match &def_vec[..] {
            [Sexp::Atom(S(keyword)), Sexp::List(_), _] if keyword == "fun" => true,
            _ => false
        }
        _ => false,
    }
}