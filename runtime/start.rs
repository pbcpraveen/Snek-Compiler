use std::env;
use std::convert::TryInto;

const MIN : i64 = -i64::pow(2, 62);
const MAX : i64 = i64::pow(2, 62) - 1;
#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input : i64, memory : *mut i64) -> i64;
}


#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    // TODO: print error message according to writeup
    match errcode {
        1 => eprintln!("runtime error: invalid argument to a binary operation"),
        2 => eprintln!("runtime error: invalid - memory overflow during allocation"),
        3 => eprintln!("runtime error: invalid - index out of bounds"),
        4 => eprintln!("runtime error: invalid - expected an array to index into"),
        _ => eprintln!("runtime error: invalid - unknown error code {errcode}"),
    }
    std::process::exit(1);
}

fn parse_input(input: &str) -> i64 {

    match input {
        "true" => 7,
        "false" => 3,
        _ => {
            match input.parse::<i64>() {
                Ok(n) => {
                    if n < MIN || n > MAX {
                        eprintln!("invalid argument overflow value: {input}");
                        std::process::exit(1);
                    }
                    (n << 1) as i64
                },
                Err(_) => {
                    eprintln!("invalid argument : {input}");
                    std::process::exit(1);
                }
            }
        }
    }
}
#[no_mangle]
#[export_name = "\x01snek_print"]
fn snek_print(val : i64) -> i64 {
  let mut seen = Vec::<i64>::new();
  println!("{}", snek_str(val, &mut seen));
  return val;
}


fn snek_str(val : i64, seen : &mut Vec<i64>) -> String {

    if val == 7 { "true".to_string() }
    else if val == 3 { "false".to_string() }
    else if val % 2 == 0 { format!("{}", val >> 1) }
    else if val == 1 { "nil".to_string() }
    else if val & 1 == 1 {
        if seen.contains(&val)  { return "[...]".to_string() }
        seen.push(val);
        let addr = (val - 1) as *const i64;
        let size_array = unsafe{ *addr } >> 1;
        let mut index = 1;
        let mut builder = vec![];
        while index <= size_array {
            let element = unsafe{ *addr.offset(index.try_into().unwrap()) };
            let stringified_element = snek_str(element, seen);
            builder.push(stringified_element);
            index += 1;
        }
        seen.pop();
        let stringified_array = "[Array: ".to_owned() + &builder.join(", ") + "]";
        return stringified_array;
    } else {
        format!("Unknown value: {}", val)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);
    let mut memory = Vec::<i64>::with_capacity(1000000);
    let buffer :*mut i64 = memory.as_mut_ptr();

    let i: i64 = unsafe { our_code_starts_here(input, buffer) };
    //println!("{}" , unsafe{*((i - 1) as *const i64)} >> 1);
    snek_print(i as i64);
}
