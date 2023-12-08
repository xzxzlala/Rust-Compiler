use std::convert::TryInto;
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone)]
struct SnakeVal(u64);
static TAG_MASK: u64 = 0x00_00_00_00_00_00_00_01;
static SNAKE_TRU: SnakeVal = SnakeVal(0xFF_FF_FF_FF_FF_FF_FF_FF);
static SNAKE_FLS: SnakeVal = SnakeVal(0x7F_FF_FF_FF_FF_FF_FF_FF);

type ErrorCode = u64;
static ARI_EXP_NUM: ErrorCode = 1;
static COM_EXP_NUM: ErrorCode = 2;
static OVERFLOW: ErrorCode = 3;
static IF_EXP_BOOL: ErrorCode = 4;
static LOGIC_EXP_BOOL: ErrorCode = 5;
static INDEX_NOT_NUMBER: ErrorCode = 6;
static INDEX_OUT_OF_BOUND: ErrorCode = 7;
static INDEX_INTO_NONARRAY: ErrorCode = 8;
static LENGTH_INTO_NONARRAY: ErrorCode = 9;
static Not_CLOSURE: ErrorCode = 10;
static ARITY_MISMATCH: ErrorCode = 11;
static FARI_EXP_FLOAT: ErrorCode = 12;
static FCOM_EXP_FLOAT: ErrorCode = 13;
static FLOAT_OVERFLOW: ErrorCode = 14;

#[repr(C)]
struct SnakeArray {
    size: u64,
    elts: *const SnakeVal,
}

/* You can use this function to cast a pointer to an array on the heap
 * into something more convenient to access
 *
 */
fn load_snake_array(p: *const u64) -> SnakeArray {
    unsafe {
        let size = *p;
        SnakeArray {
            size,
            elts: std::mem::transmute(p.add(1)),
        }
    }
}
static mut HEAP: [u64; 100000] = [0; 100000];
#[link(name = "compiled_code", kind = "static")]
extern "sysv64" {

    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    #[link_name = "\x01start_here"]
    fn start_here() -> SnakeVal;
}
fn unsigned_to_signed(x: u64) -> i64 {
    i64::from_le_bytes(x.to_le_bytes())
}
fn is_array(x: SnakeVal) -> bool {
    (x.0 & TAG_MASK == 1) && (x.0 & 0x02 == 0) && (x.0 & 0x04 == 0)
}
fn sprint_snake_val(x: SnakeVal) -> String {
    if x.0 & TAG_MASK == 0 { // it's a number
        format!("{}", unsigned_to_signed(x.0) >> 1)
    } else if x == SNAKE_TRU {
        String::from("true")
    } else if x == SNAKE_FLS {
        String::from("false")
    } else if x.0 & TAG_MASK == 1 && x.0 & 0x02 == 2 && x.0 & 0x04 == 0 { // it's a closure
        format!("<closure>")
    } else if (x.0 & TAG_MASK == 1) && (x.0 & 0x02 == 0) && (x.0 & 0x04 == 0) { // it's an array
        let arr = load_snake_array((x.0-1) as *const u64);
        let mut s = String::from("[");
        for i in 0..arr.size {
            if i != 0 {
                s.push_str(", ");
            }
            let x = unsafe { *arr.elts.add(i as usize) };
            if is_array(x) && load_snake_array((x.0-1) as *const u64).size == arr.size &&
            load_snake_array((x.0-1) as *const u64).elts == arr.elts{
                s.push_str("<loop>");
                break;
            }else {
                s.push_str(&sprint_snake_val(unsafe { *arr.elts.add(i as usize) }));
            }
        }
        s.push(']');
        s
    } else if (x.0 & TAG_MASK == 1) && (x.0 & 0x02 == 0) { // it's a float
        let a: u32 = (x.0 >> 32 as u32).try_into().unwrap();
        let f: f32 = unsafe { std::mem::transmute(a) };
        let formatted = if f.fract() == 0.0 {
            format!("{}.0", f)
        } else {
            format!("{}", f)
        };
        format!("{}", formatted)
    } else{
        format!("Invalid snake value 0x{:x}", x.0)
    }
}
#[export_name = "\x01print_snake_val"]
extern "sysv64" fn print_snake_val(v: SnakeVal) -> SnakeVal {
    println!("{}", sprint_snake_val(v));
    v
}

/* Implement the following error function. You are free to change the
 * input and output types as needed for your design.
 *
**/
#[export_name = "\x01snake_error"]
extern "sysv64" fn snake_error(err_code: u64, v: SnakeVal) {
    /* */
    if err_code == ARI_EXP_NUM{
        eprintln!("arithmetic expected a number, but got {}", sprint_snake_val(v));
    } else if err_code == COM_EXP_NUM{
        eprintln!("comparison expected a number, but got {}", sprint_snake_val(v));
    } else if err_code == OVERFLOW {
        eprintln!("overflow, {}", sprint_snake_val(v));
    } else if err_code == IF_EXP_BOOL {
        eprintln!("if expected a boolean, but got {}", sprint_snake_val(v));
    } else if err_code == LOGIC_EXP_BOOL{
        eprintln!("logic expected a boolean, but got {}", sprint_snake_val(v));
    } else if err_code == INDEX_NOT_NUMBER{
        eprintln!("index not a number, but got {}", sprint_snake_val(v));
    } else if err_code == INDEX_OUT_OF_BOUND{
        eprintln!("index out of bounds, but got {}", sprint_snake_val(v));
    } else if err_code == INDEX_INTO_NONARRAY{
        eprintln!("indexed into non-array, but got {}", sprint_snake_val(v));
    } else if err_code == LENGTH_INTO_NONARRAY{
        eprintln!("length called with non-array, but got {}", sprint_snake_val(v));        
    } else if err_code == Not_CLOSURE{
        eprintln!("called a non-function, but got {}", sprint_snake_val(v));
    } else if err_code == ARITY_MISMATCH{
        eprintln!("wrong number of arguments, but got {}", sprint_snake_val(v));
    } else if err_code == FARI_EXP_FLOAT{
        eprintln!("arithmetic expected a float, but got {}", sprint_snake_val(v));
    } else if err_code == FCOM_EXP_FLOAT{
        eprintln!("comparison expected a float, but got {}", sprint_snake_val(v));
    } else if err_code == FLOAT_OVERFLOW{
        eprintln!("overflow");
    } else
    {
        eprintln!("unknown error code {}", err_code);
    }
    std::process::exit(1);
}

fn main() {
    let output = unsafe { 
        start_here()
    };
    println!("{}", sprint_snake_val(output));
}
