use rs_envflag_macros::*;

/// an example about i64 flag
#[envflag]
const I64_FLAG: Option<i64>;

/// an example about i64 flag with default
#[envflag(default = 42)]
const I64_FLAG_W_DEFAULT: i64;

fn main() {
    if let Some(x) = I64_FLAG.fetch().unwrap() {
        println!("{}", x);
    } else {
        println!("not present.");
    }
    println!("{}", I64_FLAG_W_DEFAULT.fetch().unwrap());
}
