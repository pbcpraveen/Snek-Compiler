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
        4 => eprintln!("runtime error: invalid - expected an array"),
        5 => eprintln!("runtime error: invalid - expected number for index"),
        _ => eprintln!("runtime error: invalid - unknown error code {errcode}"),
    }
    std::process::exit(1);
}

fn parse_input(input: &str) -> i64 {

    match input {
        "null" => 1,
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


#[export_name = "\x01snek_structural_equality"]
pub extern "C" fn snek_structural_equality(heap_ptr1: *const u64, heap_ptr2: *const u64,) -> u64 {
    let val1 = heap_ptr1  as i64;
    let val2 = heap_ptr2 as i64;
    let mut seen = Vec::<Vec<i64>>::new();
    let result = snek_compare(val1, val2, &mut seen);
    return result as u64;
}


fn snek_compare(val1: i64, val2: i64, seen: &mut Vec<Vec<i64>>) -> i64 {
    if val1 == val2 {
        return 7;
    } else {
        if val1 == 1 || val2 == 1 {
            return 3;
        } else if val1 & 1 == 1 && val2 & 1 == 1 {
            let entry = vec![val1, val2];
            if seen.contains(&entry) {
                return 7;
            } else {
                seen.push(entry);
                let addr1 = (val1 - 1) as *const i64;
                let addr2 = (val2 - 1) as *const i64;
                let size_array1 = unsafe { *addr1 } >> 1;
                let size_array2 = unsafe { *addr2 } >> 1;
                if size_array1 != size_array2 {
                    return 3;
                } else {
                    let mut index = 1;
                    while index <= size_array1 {
                        let element1 = unsafe { *addr1.offset(index.try_into().unwrap()) };
                        let element2 = unsafe { *addr2.offset(index.try_into().unwrap()) };
                        let result = snek_compare(element1, element2, seen);
                        if result == 3 {
                            return 3;
                        }
                        index += 1;
                    }
                    seen.pop();
                    return 7;
                }
            }
        } else {
            return 3;
        }
    }
}

fn snek_str(val : i64, seen : &mut Vec<i64>) -> String {

    if val == 7 { "true".to_string() }
    else if val == 3 { "false".to_string() }
    else if val % 2 == 0 { format!("{}", val >> 1) }
    else if val == 1 { "null".to_string() }
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
