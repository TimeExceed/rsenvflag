use rs_envflag_macros::*;

/// an example about str flag
#[envflag]
const STR_FLAG: Option<String>;

/// an example about str flag with default
#[envflag(default = "abc")]
const STR_FLAG_W_DEFAULT: String;

fn main() {
    if let Some(x) = STR_FLAG.fetch() {
        println!("{}", x);
    } else {
        println!("not present.");
    }
    println!("{}", STR_FLAG_W_DEFAULT.fetch());
}
