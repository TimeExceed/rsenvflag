use rs_envflag::*;

define_f64_flag!(F64_FLAG, "an example about f64 flag");
define_f64_flag!(
    F64_FLAG_W_DEFAULT,
    "an example about f64 flag with default",
    0.5
);

fn main() {
    if let Some(x) = F64_FLAG.get() {
        println!("{}", x);
    } else {
        println!("not present.");
    }
    println!("{}", F64_FLAG_W_DEFAULT.get());
}
