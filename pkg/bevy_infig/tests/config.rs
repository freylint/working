//! Application configuration behavior tests
use std::ffi::OsString;

use cucumber::{World as _, given, then, when};
use mmcore::app_cfg::AppConfig;
use vfs::{MemoryFS, VfsPath};

pub const DBG_CFG: &'static str = "
    [debug]
    active=\"file\"
";

pub const DBG_CFG_PATH: &'static str = "config.toml";

#[derive(Debug, cucumber::World)]
pub struct BDDWorld {
    cfg_file: AppConfig,
    cfg_cli: AppConfig,
    cfg_none: AppConfig,
    cfg_multi: AppConfig,
    file: VfsPath,
    file_none: VfsPath,
    args: Vec<OsString>,
}

impl Default for BDDWorld {
    fn default() -> Self {
        Self {
            cfg_file: AppConfig::default(),
            cfg_cli: AppConfig::default(),
            cfg_none: AppConfig::default(),
            cfg_multi: AppConfig::default(),
            file: MemoryFS::new().into(),
            file_none: MemoryFS::new().into(),
            args: vec![],
        }
    }
}

#[given("Valid config file exists")]
fn cfg_file(ctx: &mut BDDWorld) -> Result<(), Box<dyn std::error::Error>> {
    ctx.file = ctx.file.join(DBG_CFG_PATH)?;
    write!(ctx.file.create_file()?, "{}", DBG_CFG)?;
    Ok(())
}

#[given("Clargs exist")]
fn clargs(ctx: &mut BDDWorld) {
    ctx.args = vec![OsString::from("--dbg-active=clargs")];
}

#[given("No config file")]
fn no_cfg(ctx: &mut BDDWorld) -> Result<(), Box<dyn std::error::Error>> {
    assert!(ctx.file_none.join(DBG_CFG_PATH)?.exists()? != true);
    Ok(())
}

#[when("I load the configs")]
fn load_configs(ctx: &mut BDDWorld) -> Result<(), Box<dyn std::error::Error>> {
    ctx.cfg_file = AppConfig::hierarchal_load(Some(DBG_CFG_PATH.to_string()), None)?;
    ctx.cfg_cli = AppConfig::hierarchal_load(None, Some(ctx.args.clone()))?;
    ctx.cfg_none = AppConfig::hierarchal_load(None, None)?;
    ctx.cfg_multi =
        AppConfig::hierarchal_load(Some(DBG_CFG_PATH.to_string()), Some(ctx.args.clone()))?;
    Ok(())
}

#[then("Config matches config file")]
fn file_match(ctx: &mut BDDWorld) {
    assert_eq!(ctx.cfg_file.dbg_last_cfg_src, Some("file".to_string()))
}

#[then("Config matches clargs")]
fn clargs_match(ctx: &mut BDDWorld) {
    assert_eq!(ctx.cfg_file.dbg_last_cfg_src, Some("clargs".to_string()))
}

#[then("CLArgs overrule config file")]
fn cli_override(ctx: &mut BDDWorld) {
    assert_eq!(ctx.cfg_file.dbg_last_cfg_src, Some("clargs".to_string()))
}

#[then("Default config file created")]
fn default_match(ctx: &mut BDDWorld) -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(ctx.cfg_file.dbg_last_cfg_src, Some("default".to_string()));
    assert!(ctx.file_none.join(DBG_CFG_PATH)?.exists()?);
    Ok(())
}

fn main() {
    futures::executor::block_on(BDDWorld::run("tests/features/config.feature"));
}
