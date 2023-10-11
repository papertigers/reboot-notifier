use std::process::Command;

use anyhow::{bail, Context, Result};

const SVCPROP: &str = "/usr/bin/svcprop";

pub struct SvcProp;

impl SvcProp {
    pub fn get<'a>(prop: &str, fmri: &str) -> Result<String> {
        let out = Command::new(SVCPROP)
            .args(["-p", prop, fmri])
            .output()
            .context("executing svcprop failed")?;

        if !out.status.success() {
            bail!(
                "failed to get property {prop} for fmri {fmri}: exit code {:?}",
                out.status.code()
            );
        }

        // To make things easier we are returning stdout as a UTF8 string
        // which should be fine since we are controlling the entire program.
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    }
}
