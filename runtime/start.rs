use std::env;


const MIN : i64 = -i64::pow(2, 62);
const MAX : i64 = i64::pow(2, 62) - 1;
#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64) -> u64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    // TODO: print error message according to writeup
    match errcode {
        1 => eprintln!("runtime error: invalid argument to a binary operation"),
        2 => eprintln!("runtime error: invalid - memory overflow during allocation"),
        _ => eprintln!("runtime error: invalid - unknown error code {errcode}"),
    }
    std::process::exit(1);
}

fn parse_input(input: &str) -> u64 {

    match input {
        "true" => 3,
        "false" => 1,
        _ => {
            match input.parse::<i64>() {
                Ok(n) => {
                    if n < MIN || n > MAX {
                        eprintln!("invalid argument overflow value: {input}");
                        std::process::exit(1);
                    }
                    (n << 1) as u64
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
  if val == 3 { println!("true"); }
  else if val == 1 { println!("false"); }
  else if val % 2 == 0 { println!("{}", val >> 1); }
  else {
    println!("Invalid : Unknown value: {}", val);
  }
  return val;
}

fn print_return_val(val: i64) -> String {
    match val {
        1 => "false".to_string(),
        3 => "true".to_string(),
        n if n % 2 == 0 => (n >> 1).to_string(),
        n => panic!("Invalid 63 bit representation: {}", n),
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let i: u64 = unsafe { our_code_starts_here(input) };
    println!("{}", print_return_val(i as i64));
}
