# rs_envflag

This crate provides an easy to define flags by environment variables.
It is a rust counterpart of https://github.com/TimeExceed/envflag.

```rust
use rs_envflag::*;

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
```

One can find more examples in `examples/`.

