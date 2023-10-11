use rs_envflag_macros::*;

/// an example about bool flag
#[envflag]
const BOOL_FLAG: Option<bool>;

/// an example about bool flag with default
#[envflag(default = true)]
const BOOL_FLAG_W_DEFAULT: bool;

fn main() {
    if let Some(x) = BOOL_FLAG.fetch().unwrap() {
        println!("{}", x);
    } else {
        println!("not present.");
    }
    println!("{}", BOOL_FLAG_W_DEFAULT.fetch().unwrap());
}
