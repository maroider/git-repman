use std::{ffi::OsString, path::PathBuf};

use clap::Clap;
use duct::cmd;
use url::Url;

/// Clone a repository
///
/// You can pass along extra arguments to git by appending `-- <args>` to the
/// command line.
#[derive(Clap)]
pub struct Clone {
    url: String,
    /// Add an upstream remote by using a username / organization name.
    ///
    /// The user name / organization name where the upstream repo can be found.
    /// The name of the upstream repo must be the same as the repo being cloned.
    #[clap(short, long)]
    upstream: Option<String>,
    /// Add an upstream remote by specifying its full URL.
    #[clap(long, conflicts_with("upstream"))]
    upstream_url: Option<String>,
    #[clap(last(true))]
    args: Vec<OsString>,
}

impl Clone {
    pub fn run(self) -> anyhow::Result<()> {
        let mut args: Vec<OsString> = vec!["clone".into(), self.url.clone().into()];
        let url = Url::parse(&self.url)?;

        let mut clone_path_segments: Vec<&str> = Vec::new();
        if let Some(host) = url.host_str() {
            clone_path_segments.push(host);
            if let Some(path) = url.path_segments() {
                clone_path_segments.extend(path);
            }
        }
        let mut clone_path = basedir()?;
        clone_path.extend(clone_path_segments);
        args.push(clone_path.clone().into_os_string());
        args.extend(self.args);
        cmd("git", args).run()?;

        if let Some(upstream) = self.upstream {
            let mut remote_url = url.clone();
            let mut segments: Vec<String> = url
                .path_segments()
                .map(|segments| segments.map(ToString::to_string).collect())
                .unwrap_or(vec![]);
            if let Ok(mut segments_mut) = remote_url.path_segments_mut() {
                segments_mut.clear();
                segments[0] = upstream;
                segments_mut.extend(segments);
            }
            cmd!("git", "remote", "add", "upstream", remote_url.to_string())
                .dir(&clone_path)
                .run()?;
        } else if let Some(upstream_url) = self.upstream_url {
            cmd!("git", "remote", "add", "upstream", upstream_url)
                .dir(&clone_path)
                .run()?;
        }
        Ok(())
    }
}

fn basedir() -> anyhow::Result<PathBuf> {
    let output = cmd!("git", "config", "--global", "repman.basedir")
        .stdout_capture()
        .unchecked()
        .run()?;
    match output.status.code() {
        Some(-1) => {
            anyhow::bail!(
                "repman.basedir is not set. Please use `git config --gobal` to configure it."
            )
        }
        Some(0) => {}
        Some(_) => {
            anyhow::bail!("Non-zero exit code while reading repman.basedir")
        }
        None => anyhow::bail!("No exit code while reading repman.basedir"),
    }
    let dir = String::from_utf8(output.stdout)?;
    let dir = dir.trim();
    if dir.is_empty() {
        anyhow::bail!("repman.basedir is an empty string")
    }
    let dir = PathBuf::from(dir);
    if !dir.is_absolute() {
        anyhow::bail!("repman.basedir must be an absolute path")
    }
    Ok(dir)
}
