use rs_envflag::*;

#[test]
fn strflag_unset() {
    define_str_flag!(STRFLAG_UNSET, "");
    assert!(STRFLAG_UNSET.get().is_none());
}

#[test]
fn strflag_set() {
    define_str_flag!(STRFLAG_SET, "");
    std::env::set_var("STRFLAG_SET", "xixi");
    assert_eq!(STRFLAG_SET.get().unwrap(), "xixi");
}

#[test]
fn strflagwd_unset() {
    define_str_flag!(STRFLAGWD_UNSET, "", "xixi");
    assert_eq!(STRFLAGWD_UNSET.get(), "xixi");
}

#[test]
fn strflagwd_set() {
    define_str_flag!(STRFLAGWD_SET, "", "xixi");
    std::env::set_var("STRFLAGWD_SET", "haha");
    assert_eq!(STRFLAGWD_SET.get(), "haha");
}

#[test]
fn i64flag_unset() {
    define_i64_flag!(I64FLAG_UNSET, "");
    assert!(I64FLAG_UNSET.get().is_none());
}

#[test]
fn i64flag_set() {
    define_i64_flag!(I64FLAG_SET, "");
    std::env::set_var("I64FLAG_SET", "42");
    assert_eq!(I64FLAG_SET.get().unwrap(), 42);
}

#[test]
fn i64flagwd_unset() {
    define_i64_flag!(I64FLAGWD_UNSET, "", 42);
    assert_eq!(I64FLAGWD_UNSET.get(), 42);
}

#[test]
fn i64flagwd_set() {
    define_i64_flag!(I64FLAGWD_SET, "", 42);
    std::env::set_var("I64FLAGWD_SET", "2333");
    assert_eq!(I64FLAGWD_SET.get(), 2333);
}

#[test]
fn f64flag_unset() {
    define_f64_flag!(F64FLAG_UNSET, "");
    assert!(F64FLAG_UNSET.get().is_none());
}

#[test]
fn f64flag_set() {
    define_f64_flag!(F64FLAG_SET, "");
    std::env::set_var("F64FLAG_SET", "4.2");
    assert_eq!(F64FLAG_SET.get().unwrap(), 4.2);
}

#[test]
fn f64flagwd_unset() {
    define_f64_flag!(F64FLAGWD_UNSET, "", 4.2);
    assert_eq!(F64FLAGWD_UNSET.get(), 4.2);
}

#[test]
fn f64flagwd_set() {
    define_f64_flag!(F64FLAGWD_SET, "", 4.2);
    std::env::set_var("F64FLAGWD_SET", "23.33");
    assert_eq!(F64FLAGWD_SET.get(), 23.33);
}

#[test]
fn bflag_unset() {
    define_bool_flag!(BFLAG_UNSET, "");
    assert!(BFLAG_UNSET.get().is_none());
}

#[test]
fn bflag_set_false() {
    define_bool_flag!(BFLAG_SET, "");
    std::env::set_var("BFLAG_SET", "false");
    assert!(!BFLAG_SET.get().unwrap());
}

#[test]
fn bflag_set_true() {
    define_bool_flag!(BFLAG_SET, "");
    std::env::set_var("BFLAG_SET", "true");
    assert!(BFLAG_SET.get().unwrap());
}

#[test]
fn bflagwd_unset() {
    define_bool_flag!(BFLAG_UNSET, "", true);
    assert!(BFLAG_UNSET.get());
}

#[test]
fn bflagwd_set_false() {
    define_bool_flag!(BFLAG_SET, "", true);
    std::env::set_var("BFLAG_SET", "false");
    assert!(!BFLAG_SET.get());
}

#[test]
fn bflagwd_set_true() {
    define_bool_flag!(BFLAG_SET, "", false);
    std::env::set_var("BFLAG_SET", "true");
    assert!(BFLAG_SET.get());
}
