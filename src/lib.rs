use log::*;

pub struct EnvFlag<T> {
    pub key: &'static str,
    pub parser: fn(key: &str, val: &str) -> T,
}

impl<T> EnvFlag<T> {
    pub fn get(&self) -> Option<T> {
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

pub struct EnvFlagWithDefault<T: Clone> {
    pub key: &'static str,
    pub parser: fn(key: &str, val: &str) -> T,
    pub default: T,
}

impl<T: Clone> EnvFlagWithDefault<T> {
    pub fn get(&self) -> T {
        match std::env::var(self.key) {
            Ok(x) => {
                debug!("ENV {}={}", self.key, x);
                (self.parser)(self.key, &x)
            }
            Err(std::env::VarError::NotPresent) => self.default.clone(),
            Err(std::env::VarError::NotUnicode(e)) => {
                error!("Fail to fetch ENV {}: {:?}", self.key, e);
                panic!("Fail to fetch ENV {}: {:?}", self.key, e);
            }
        }
    }
}

pub struct StrFlagWithDefault {
    flag: EnvFlag<String>,
    default: &'static str,
}

impl StrFlagWithDefault {
    pub const fn new(key: &'static str, default: &'static str) -> Self {
        Self {
            flag: EnvFlag::<String> {
                key,
                parser: |_key: &str, val: &str| -> String { val.to_string() },
            },
            default,
        }
    }

    pub fn get(&self) -> String {
        if let Some(r) = self.flag.get() {
            r
        } else {
            self.default.to_string()
        }
    }
}

#[macro_export]
macro_rules! define_str_flag {
    ($key:ident, $help:literal) => {
        const $key: $crate::EnvFlag<String> = $crate::EnvFlag::<String> {
            key: stringify!($key),
            parser: |_key: &str, val: &str| -> String { val.to_string() },
        };
    };
    ($key:ident, $help:literal, $default:expr) => {
        const $key: $crate::StrFlagWithDefault = $crate::StrFlagWithDefault::new(stringify!($key), $default);
    };
}

pub struct BoolFlag {
    flag: EnvFlag<bool>,
}

impl BoolFlag {
    pub const fn new(key: &'static str) -> Self {
        Self {
            flag: EnvFlag::<bool> {
                key,
                parser: parse_bool,
            },
        }
    }

    pub fn get(&self) -> Option<bool> {
        self.flag.get()
    }
}

pub struct BoolFlagWithDefault {
    flag: EnvFlagWithDefault<bool>,
}

impl BoolFlagWithDefault {
    pub const fn new(key: &'static str, default: bool) -> Self {
        Self {
            flag: EnvFlagWithDefault::<bool> {
                key,
                parser: parse_bool,
                default,
            },
        }
    }

    pub fn get(&self) -> bool {
        self.flag.get()
    }
}

fn parse_bool(key: &str, val: &str) -> bool {
    val.parse().unwrap_or_else(|e| {
        error!(
            "Unrecognized ENV \"{}\": \"{}\"\
            \n  error: {:?}",
            key, val, e,
        );
        panic!("Unrecognized ENV \"{}\": \"{}\"", key, val);
    })
}

#[macro_export]
macro_rules! define_bool_flag {
    ($key:ident, $help:literal) => {
        const $key: $crate::BoolFlag = $crate::BoolFlag::new(stringify!($key));
    };
    ($key:ident, $help:literal, $default:expr) => {
        const $key: $crate::BoolFlagWithDefault = $crate::BoolFlagWithDefault::new(stringify!($key), $default);
    };
}

pub struct I64Flag {
    flag: EnvFlag<i64>,
}

impl I64Flag {
    pub const fn new(key: &'static str) -> Self {
        Self {
            flag: EnvFlag::<i64> {
                key,
                parser: parse_int,
            },
        }
    }

    pub fn get(&self) -> Option<i64> {
        self.flag.get()
    }
}

pub struct I64FlagWithDefault {
    flag: EnvFlagWithDefault<i64>,
}

impl I64FlagWithDefault {
    pub const fn new(key: &'static str, default: i64) -> Self {
        Self {
            flag: EnvFlagWithDefault {
                key,
                parser: parse_int,
                default,
            },
        }
    }

    pub fn get(&self) -> i64 {
        self.flag.get()
    }
}

fn parse_int(key: &str, x: &str) -> i64 {
    x.parse().unwrap_or_else(|e| {
        error!(
            "Unrecognized ENV \"{}\": \"{}\"\
            \n  error: {:?}",
            key, x, e,
        );
        panic!("Unrecognized ENV \"{}\": \"{}\"", key, x);
    })
}

#[macro_export]
macro_rules! define_i64_flag {
    ($key:ident, $help:literal) => {
        const $key: $crate::I64Flag = $crate::I64Flag::new(stringify!($key));
    };
    ($key:ident, $help:literal, $default:expr) => {
        const $key: $crate::I64FlagWithDefault = $crate::I64FlagWithDefault::new(stringify!($key), $default);
    };
}

pub struct F64Flag {
    flag: EnvFlag<f64>,
}

impl F64Flag {
    pub const fn new(key: &'static str) -> Self {
        Self {
            flag: EnvFlag::<f64> {
                key,
                parser: parse_f64,
            },
        }
    }

    pub fn get(&self) -> Option<f64> {
        self.flag.get()
    }
}

pub struct F64FlagWithDefault {
    flag: EnvFlagWithDefault<f64>,
}

impl F64FlagWithDefault {
    pub const fn new(key: &'static str, default: f64) -> Self {
        Self {
            flag: EnvFlagWithDefault::<f64> {
                key,
                parser: parse_f64,
                default,
            },
        }
    }

    pub fn get(&self) -> f64 {
        self.flag.get()
    }
}

fn parse_f64(key: &str, x: &str) -> f64 {
    x.parse().unwrap_or_else(|e| {
        error!(
            "Unrecognized ENV \"{}\": \"{}\"\
            \n  error: {:?}",
            key, x, e,
        );
        panic!("Unrecognized ENV \"{}\": \"{}\"", key, x);
    })
}

#[macro_export]
macro_rules! define_f64_flag {
    ($key:ident, $help:literal) => {
        const $key: $crate::F64Flag = $crate::F64Flag::new(stringify!($key));
    };
    ($key:ident, $help:literal, $default:expr) => {
        const $key: $crate::F64FlagWithDefault = $crate::F64FlagWithDefault::new(stringify!($key), $default);
    };
}
