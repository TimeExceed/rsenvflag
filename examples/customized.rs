use rs_envflag_macros::*;

#[envflag(parser=v_parser)]
const X: Option<V>;

#[envflag(parser=v_parser, default=&V::DEFAULT)]
const X_W_DEFAULT: V;

fn main() {
    if let Some(x) = X.fetch().unwrap() {
        println!("{}", x.0);
    } else {
        println!("not present.");
    }
    println!("{:?}", X_W_DEFAULT.fetch());
}

#[derive(Debug, Clone)]
struct V(String);

impl V {
    const DEFAULT: V = V(String::new());
}

fn v_parser(_key: &str, value: &str) -> anyhow::Result<V> {
    Ok(V(value.to_string()))
}
