#![doc = include_str!("../README.md")]

pub struct EnvFlag<T> {
    pub key: &'static str,
    pub parser: fn(key: &str, val: &str) -> anyhow::Result<T>,
}

impl<T> EnvFlag<T> {
    pub fn fetch(&'static self) -> anyhow::Result<Option<T>> {
        match std::env::var(self.key) {
            Ok(x) => Some((self.parser)(self.key, &x)).transpose(),
            Err(std::env::VarError::NotPresent) => Ok(None),
            Err(std::env::VarError::NotUnicode(e)) => {
                anyhow::bail!("Fail to fetch ENV {}: {:?}", self.key, e);
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
    pub fn fetch(&'static self) -> anyhow::Result<T> {
        self.env
            .fetch()
            .map(|x| x.unwrap_or(self.default.to_owned()))
    }
}

pub fn str_parser(_key: &str, val: &str) -> anyhow::Result<String> {
    Ok(val.to_string())
}

pub fn bool_parser(key: &str, val: &str) -> anyhow::Result<bool> {
    val.parse()
        .map_err(|e| anyhow::Error::msg(format!("Unrecognized ENV \"{}\": \"{:?}\"", key, e)))
}

pub fn i64_parser(key: &str, x: &str) -> anyhow::Result<i64> {
    x.parse()
        .map_err(|e| anyhow::Error::msg(format!("Unrecognized ENV \"{}\": \"{:?}\"", key, e)))
}

pub fn f64_parser(key: &str, x: &str) -> anyhow::Result<f64> {
    x.parse()
        .map_err(|e| anyhow::Error::msg(format!("Unrecognized ENV \"{}\": \"{:?}\"", key, e)))
}
