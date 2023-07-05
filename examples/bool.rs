use rs_envflag::define_bool_flag;

define_bool_flag!(BOOL_FLAG, "an example about bool flag");
define_bool_flag!(
    BOOL_FLAG_W_DEFAULT,
    "an example about bool flag with default",
    true
);

fn main() {
    if let Some(x) = BOOL_FLAG.get() {
        println!("{}", x);
    } else {
        println!("not present.");
    }
    println!("{}", BOOL_FLAG_W_DEFAULT.get());
}
