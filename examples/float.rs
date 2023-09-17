use rs_envflag_macros::*;

/// an example about f64 flag
#[envflag]
const F64_FLAG: Option<f64>;

/// an example about f64 flag with default
#[envflag(default = 0.5)]
const F64_FLAG_W_DEFAULT: f64;

fn main() {
    if let Some(x) = F64_FLAG.fetch() {
        println!("{}", x);
    } else {
        println!("not present.");
    }
    println!("{}", F64_FLAG_W_DEFAULT.fetch());
}
