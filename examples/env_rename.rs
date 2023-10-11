use rs_envflag_macros::*;

/// env is named as `XYZ` rather `ABC`.
#[envflag(env_name = "XYZ")]
const ABC: Option<String>;

fn main() {
    if let Some(x) = ABC.fetch().unwrap() {
        println!("{}", x);
    } else {
        println!("not present.");
    }
}
