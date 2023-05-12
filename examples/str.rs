use rs_envflag::*;

define_str_flag!(STR_FLAG, "an example about str flag");
define_str_flag!(
    STR_FLAG_W_DEFAULT,
    "an example about str flag with default",
    "abc"
);

fn main() {
    if let Some(x) = STR_FLAG.get() {
        println!("{}", x);
    } else {
        println!("not present.");
    }
    println!("{}", STR_FLAG_W_DEFAULT.get());
}
