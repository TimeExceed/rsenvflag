#![doc = include_str!("../README.md")]

use log::*;

pub struct EnvFlag<T> {
    pub key: &'static str,
    pub parser: fn(key: &str, val: &str) -> T,
}

impl<T> EnvFlag<T> {
    pub fn fetch(&'static self) -> Option<T> {
        match std::env::var(self.key) {
            Ok(x) => {
                debug!("ENV {}={}", self.key, x);
                Some((self.parser)(self.key, &x))
            }
            Err(std::env::VarError::NotPresent) => None,
            Err(std::env::VarError::NotUnicode(e)) => {
                error!("Fail to fetch ENV {}: {:?}", self.key, e);
                panic!("Fail to fetch ENV {}: {:?}", self.key, e);
            }
        }
    }
}

pub struct EnvFlagWithDefault<T, DefaultT>
where
    T: Clone,
    DefaultT: ToOwned<Owned = T> + 'static + ?Sized,
{
    pub env: EnvFlag<T>,
    pub default: &'static DefaultT,
}

impl<T, DefaultT> EnvFlagWithDefault<T, DefaultT>
where
    T: Clone,
    DefaultT: ToOwned<Owned = T> + 'static + ?Sized,
{
    pub fn fetch(&'static self) -> T {
        self.env.fetch().unwrap_or(self.default.to_owned())
    }
}

pub fn str_parser(_key: &str, val: &str) -> String {
    val.to_string()
}

pub fn bool_parser(key: &str, val: &str) -> bool {
    val.parse().unwrap_or_else(|e| {
        error!(
            "Unrecognized ENV \"{}\": \"{}\"\
            \n  error: {:?}",
            key, val, e,
        );
        panic!("Unrecognized ENV \"{}\": \"{}\"", key, val);
    })
}

pub fn i64_parser(key: &str, x: &str) -> i64 {
    x.parse().unwrap_or_else(|e| {
        error!(
            "Unrecognized ENV \"{}\": \"{}\"\
            \n  error: {:?}",
            key, x, e,
        );
        panic!("Unrecognized ENV \"{}\": \"{}\"", key, x);
    })
}

pub fn f64_parser(key: &str, x: &str) -> f64 {
    x.parse().unwrap_or_else(|e| {
        error!(
            "Unrecognized ENV \"{}\": \"{}\"\
            \n  error: {:?}",
            key, x, e,
        );
        panic!("Unrecognized ENV \"{}\": \"{}\"", key, x);
    })
}
