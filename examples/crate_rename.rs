use rs_envflag as xyz;
use rs_envflag_macros::*;

/// crate is renamed to `xyz`.
#[envflag(crate=xyz)]
const STR_FLAG: Option<String>;

fn main() {
    if let Some(x) = STR_FLAG.fetch().unwrap() {
        println!("{}", x);
    } else {
        println!("not present.");
    }
}
