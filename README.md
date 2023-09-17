# rs_envflag

This crate provides an easy to define flags controlled by environment variables.
It is a rust, and of course much more rustacean, reimplementation of <https://github.com/TimeExceed/envflag>.

## a str flag without default values

Here is an example about defining a str flag.

```rust
use rs_envflag_macros::*;

/// an example about str flag
#[envflag]
const STR_FLAG: Option<String>;

fn main() {
    if let Some(x) = STR_FLAG.fetch() {
        println!("{}", x);
    } else {
        println!("not present.");
    }
}
```

When we run it directly, `STR_FLAG` will be `None`.

```text
$ cargo build --examples && target/debug/examples/str
not present.
```

But once `STR_FLAG` is set, it looks like

```text
$ cargo build --examples && STR_FLAG=abc target/debug/examples/str
abc
```

## a str flag with default values

Also we can define default values to flags.

```rust
use rs_envflag_macros::*;

/// an example about str flag with default
#[envflag(default="abc")]
const STR_FLAG_W_DEFAULT: String;

fn main() {
    println!("{}", STR_FLAG_W_DEFAULT.fetch());
}
```

Then we will compile and run it.

```text
$ cargo build --examples && target/debug/examples/str && STR_FLAG_W_DEFAULT=xyz target/debug/examples/str
xyz
```

### i64/f64/bool flags with/without default values

We can also define i64, f64 and bool flags, either with or without, default values.
Please refer to [examples/](https://github.com/TimeExceed/rsenvflag/tree/main/examples) for details.

### customized types and customized parsers

Now we will show how to define flags with customized types.

```rust
use rs_envflag_macros::*;

#[envflag(parser=v_parser)]
const X: Option<V>;

fn main() {
    if let Some(x) = X.fetch() {
        println!("{:?}", x);
    } else {
        println!("not present.");
    }
}

#[derive(Debug)]
struct V(String);

fn v_parser(_key: &str, value: &str) -> V {
    V(value.to_string())
}
```

*   Just a parser is required.
    It must accept 2 arguments.
    The 2nd argument is the value to parse.
    The 1st is the name of the env variable.
    This is not necessary for parsing values.
    But it is convenient to log something about parsing errors and warnings.

To define default values of env flags of customized types, more things are required.

```rust
use rs_envflag_macros::*;

#[envflag(parser=v_parser, default=&V::DEFAULT)]
const X_W_DEFAULT: V;

fn main() {
    println!("{:?}", X_W_DEFAULT.fetch());
}

#[derive(Debug)]
struct V(String);

impl V {
    const DEFAULT: V = V(String::new());
}

fn v_parser(_key: &str, value: &str) -> V {
    V(value.to_string())
}

impl From<&V> for V {
    fn from(value: &V) -> Self {
        V(value.0.clone())
    }
}
```

1.  Besides parsers, _const_ default values are required.
    And they must be refered by references, e.g., in this example `default=&V::DEFAULT`.
2.  A conversion from `&V` to `V`.
    Why so bothering?
    The answer is that the default values can have different types.
    Just they, actually their references, must be convertiable to types of flags.

### flag renaming

Names of env variables and those in rust can be different.
We support it by `env_name` attribute.

```rust
use rs_envflag_macros::*;

/// env is named as `XYZ` rather `ABC`.
#[envflag(env_name="XYZ")]
const ABC: Option<String>;

fn main() {
    if let Some(x) = ABC.fetch() {
        println!("{}", x);
    } else {
        println!("not present.");
    }
}
```

Now, this program will response what env variable `XYZ` is.

```text
$ cargo build --examples && target/debug/examples/env_rename && XYZ=xyz target/debug/examples/env_rename
not present.
xyz
```

### crate renaming

Occasionally, crate `rs_envflag` have to be imported as a different name.
We also support this case by `crate` attribute.
Please refer to [examples/crate_rename.rs](https://github.com/TimeExceed/rsenvflag/tree/main/examples/crate_rename.rs) for details.
