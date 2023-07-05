use rs_envflag::define_i64_flag;

define_i64_flag!(I64_FLAG, "an example about i64 flag");
define_i64_flag!(
    I64_FLAG_W_DEFAULT,
    "an example about i64 flag with default",
    42
);

fn main() {
    if let Some(x) = I64_FLAG.get() {
        println!("{}", x);
    } else {
        println!("not present.");
    }
    println!("{}", I64_FLAG_W_DEFAULT.get());
}
