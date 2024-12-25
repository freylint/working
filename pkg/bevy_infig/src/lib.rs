use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{any::Any, ffi::OsString, path::Path};

#[derive(Debug, Parser, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntegratedConfig {
    name: Option<String>,
    pub dbg_last_cfg_src: Option<String>,
}

impl IntegratedConfig {
    fn write_default<T: AsRef<Path>>(path: T) -> Result<Self, std::io::Error> {
        unimplemented!()
    }

    pub fn cli_load(argv: Option<Vec<OsString>>) -> Result<Self, Box<dyn std::error::Error>> {
        Err("unimplemented".into())
    }

    pub fn file_load(path: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        Err("unimplemented".into())
    }

    pub fn hierarchal_load(
        path: Option<String>,
        argv: Option<Vec<OsString>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self::default()
            .merge(Self::file_load(path)?)
            .merge(Self::cli_load(argv)?))
    }

    pub fn merge(self, above: Self) -> Self {
        unimplemented!()
    }

    fn merge_select<T: Any>(maybe_left: Option<T>, maybe_right: Option<T>) -> Option<T> {
        match (&maybe_left, &maybe_right) {
            (None, None) => None,
            (Some(_), None) => maybe_left,
            (None, Some(_)) => maybe_right,
            (Some(_), Some(_)) => maybe_right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test merge_select with `None` `None` input
    fn msel_nn() {
        let left: Option<bool> = None;
        let right: Option<bool> = None;

        assert!(IntegratedConfig::merge_select(left, right).is_none())
    }

    #[test]
    fn msel_sn() {
        let left: Option<bool> = Some(false);
        let right: Option<bool> = None;

        let merged = IntegratedConfig::merge_select(left, right);

        assert!(merged.is_some());
        assert_eq!(merged.unwrap(), left.unwrap());
    }

    #[test]
    fn msel_ns() {
        let left: Option<bool> = None;
        let right: Option<bool> = Some(false);

        let merged = IntegratedConfig::merge_select(left, right);

        assert!(merged.is_some());
        assert_eq!(merged.unwrap(), right.unwrap());
    }

    #[test]
    fn msel_ss() {
        let left: Option<bool> = Some(true);
        let right: Option<bool> = Some(false);

        let merged = IntegratedConfig::merge_select(left, right);

        assert!(merged.is_some());
        assert_eq!(merged.unwrap(), right.unwrap());
    }

    #[test]
    fn merge_overwrite() {
        let left = IntegratedConfig {
            name: None,
            dbg_last_cfg_src: None,
        };

        // Test is invalid of left eq to default
        assert_ne!(left, IntegratedConfig::default());
        assert_eq!(
            left.merge(IntegratedConfig::default()),
            IntegratedConfig::default()
        )
    }
}

impl Default for IntegratedConfig {
    fn default() -> Self {
        Self {
            name: Some(env!("CARGO_CRATE_NAME").to_string()),
            dbg_last_cfg_src: Some(String::from("default")),
        }
    }
}
