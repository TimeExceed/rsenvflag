use rs_envflag_macros::*;

#[test]
fn strflag_unset() {
    #[envflag]
    const STRFLAG_UNSET: Option<String>;

    assert!(STRFLAG_UNSET.fetch().unwrap().is_none());
}

#[test]
fn strflag_set() {
    #[envflag]
    const STRFLAG_SET: Option<String>;

    std::env::set_var("STRFLAG_SET", "xixi");
    assert_eq!(STRFLAG_SET.fetch().unwrap(), Some("xixi".to_owned()));
}

#[test]
fn strflagwd_unset() {
    #[envflag(default = "xixi")]
    const STRFLAGWD_UNSET: String;

    assert_eq!(STRFLAGWD_UNSET.fetch().unwrap(), "xixi");
}

#[test]
fn strflagwd_set() {
    #[envflag(default = "xixi")]
    const STRFLAGWD_SET: String;

    std::env::set_var("STRFLAGWD_SET", "haha");
    assert_eq!(STRFLAGWD_SET.fetch().unwrap(), "haha");
}

#[test]
fn crate_rename() {
    use rs_envflag as xyz;
    #[envflag(default="xixi", crate=xyz)]
    const CRATE_RENAME: String;

    std::env::set_var("CRATE_RENAME", "haha");
    assert_eq!(CRATE_RENAME.fetch().unwrap(), "haha");
}

#[test]
fn env_rename() {
    #[envflag(default = "xixi", env_name = "XYZ")]
    const ABC: String;

    std::env::set_var("XYZ", "haha");
    assert_eq!(ABC.fetch().unwrap(), "haha");
}

#[test]
fn i64flag_unset() {
    #[envflag]
    const I64FLAG_UNSET: Option<i64>;

    assert!(I64FLAG_UNSET.fetch().unwrap().is_none());
}

#[test]
fn i64flag_set() {
    #[envflag]
    const I64FLAG_SET: Option<i64>;
    std::env::set_var("I64FLAG_SET", "42");
    assert_eq!(I64FLAG_SET.fetch().unwrap(), Some(42));
}

#[test]
fn i64flagwd_unset() {
    #[envflag(default = 42)]
    const I64FLAGWD_UNSET: i64;

    assert_eq!(I64FLAGWD_UNSET.fetch().unwrap(), 42);
}

#[test]
fn i64flagwd_set() {
    #[envflag(default = 42)]
    const I64FLAGWD_SET: i64;

    std::env::set_var("I64FLAGWD_SET", "2333");
    assert_eq!(I64FLAGWD_SET.fetch().unwrap(), 2333);
}

#[test]
fn f64flag_unset() {
    #[envflag]
    const F64FLAG_UNSET: Option<f64>;

    assert!(F64FLAG_UNSET.fetch().unwrap().is_none());
}

#[test]
fn f64flag_set() {
    #[envflag]
    const F64FLAG_SET: Option<f64>;

    std::env::set_var("F64FLAG_SET", "4.2");
    assert_eq!(F64FLAG_SET.fetch().unwrap(), Some(4.2));
}

#[test]
fn f64flagwd_unset() {
    #[envflag(default = 4.2)]
    const F64FLAGWD_UNSET: f64;

    assert_eq!(F64FLAGWD_UNSET.fetch().unwrap(), 4.2);
}

#[test]
fn f64flagwd_set() {
    #[envflag(default = 4.2)]
    const F64FLAGWD_SET: f64;

    std::env::set_var("F64FLAGWD_SET", "23.33");
    assert_eq!(F64FLAGWD_SET.fetch().unwrap(), 23.33);
}

#[test]
fn bflag_unset() {
    #[envflag]
    const BFLAG_UNSET: Option<bool>;

    assert!(BFLAG_UNSET.fetch().unwrap().is_none());
}

#[test]
fn bflag_set_false() {
    #[envflag]
    const BFLAG_SET_FALSE: Option<bool>;

    std::env::set_var("BFLAG_SET_FALSE", "false");
    assert!(!BFLAG_SET_FALSE.fetch().unwrap().unwrap());
}

#[test]
fn bflag_set_true() {
    #[envflag]
    const BFLAG_SET_TRUE: Option<bool>;

    std::env::set_var("BFLAG_SET_TRUE", "true");
    assert_eq!(BFLAG_SET_TRUE.fetch().unwrap(), Some(true));
}

#[test]
fn bflagwd_unset() {
    #[envflag(default = true)]
    const BFLAGWD_UNSET: bool;

    assert!(BFLAGWD_UNSET.fetch().unwrap());
}

#[test]
fn bflagwd_set_false() {
    #[envflag(default = true)]
    const BFLAGWD_SET_FALSE: bool;

    std::env::set_var("BFLAGWD_SET_FALSE", "false");
    assert!(!BFLAGWD_SET_FALSE.fetch().unwrap());
}

#[test]
fn bflagwd_set_true() {
    #[envflag(default = false)]
    const BFLAGWD_SET_TRUE: bool;

    std::env::set_var("BFLAGWD_SET_TRUE", "true");
    assert!(BFLAGWD_SET_TRUE.fetch().unwrap());
}

#[test]
fn customized_parser_unset() {
    #[envflag(parser=v_parser)]
    const CSTM_FLAG_UNSET: Option<V>;

    assert!(CSTM_FLAG_UNSET.fetch().unwrap().is_none());
}

#[test]
fn customized_parser_set() {
    #[envflag(parser=v_parser)]
    const CSTM_FLAG_SET: Option<V>;

    std::env::set_var("CSTM_FLAG_SET", "xixi");
    assert_eq!(CSTM_FLAG_SET.fetch().unwrap(), Some(V("xixi".to_string())));
}

#[test]
fn customized_parser_wd_unset() {
    #[envflag(parser=v_parser, default=&V::DEFAULT)]
    const CSTM_FLAG_WD_UNSET: V;

    assert_eq!(CSTM_FLAG_WD_UNSET.fetch().unwrap(), V::DEFAULT);
}

#[test]
fn customized_parser_wd_set() {
    #[envflag(parser=v_parser, default=&V::DEFAULT)]
    const CSTM_FLAG_WD_SET: V;

    std::env::set_var("CSTM_FLAG_WD_SET", "xixi");
    assert_eq!(CSTM_FLAG_WD_SET.fetch().unwrap(), V("xixi".to_string()));
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct V(String);

impl V {
    const DEFAULT: V = V(String::new());
}

fn v_parser(_key: &str, value: &str) -> anyhow::Result<V> {
    Ok(V(value.to_string()))
}
