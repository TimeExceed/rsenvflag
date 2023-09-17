use rs_envflag_macros::*;

// #[envflag(parser=v_parser)]
// const X: Option<V>;

#[envflag(parser=v_parser, default=&V::DEFAULT)]
const X_W_DEFAULT: V;

fn main() {
    // if let Some(x) = X.fetch() {
    //     println!("{:?}", x);
    // } else {
    //     println!("not present.");
    // }
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
