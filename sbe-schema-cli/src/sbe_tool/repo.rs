use anyhow::{bail, Result};
use git2::{build::{CheckoutBuilder, RepoBuilder}, FetchOptions, Progress, RemoteCallbacks};
use std::{cell::RefCell, io::{self, Write}, path::{Path, PathBuf}, process::Command};

struct State {
    progress: Option<Progress<'static>>,
    total: usize,
    current: usize,
    path: Option<PathBuf>,
    newline: bool,
}

const GIT_URL: &str = "https://github.com/real-logic/simple-binary-encoding.git";
const CHECKOUT_DIR: &str = ".simple-binary-encoding";
const GRADLEW_CMD: &str = "./gradlew";
const SBE_VERSION_FILE: &str = "version.txt";

pub fn clean() -> Result<()> {
    rm_repo_folder()?;

    let version_file = Path::new(super::SBE_VERSION_FILE);
    let version = std::fs::read_to_string(version_file)?;
    let jar = super::SBE_JAR_FORMAT.replace("{version}", &version.trim());

    std::fs::remove_file(version_file)?;
    std::fs::remove_file(Path::new(jar.as_str()))?;
    
    Ok(())
}

/// Clean the SBE tool directory
fn rm_repo_folder() -> Result<()> {
    let dir = Path::new(CHECKOUT_DIR);
    if dir.exists() {
        std::fs::remove_dir_all(dir)?;
    } else {
        bail!("Directory does not exist");
    }
    Ok(())
}

/// Build the SBE tool
pub fn build() -> Result<()> {
    let output = Command::new(GRADLEW_CMD)
        .current_dir(Path::new(CHECKOUT_DIR))
        .spawn()
        .expect("Unable to spawn sbe build")
        .wait_with_output()
        .expect("Unable to execute sbe build");

    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr).unwrap();
        bail!("SBE build failed\n{}", stderr);
    }

    Ok(())
}

/// Copy the SBE tool jar to the current directory
pub fn copy_sbe_jar() -> Result<()> {

    let version_file = Path::new(CHECKOUT_DIR).join(SBE_VERSION_FILE);
    let version = std::fs::read_to_string(version_file.clone())?;
    let jar = super::SBE_JAR_FORMAT.replace("{version}", &version.trim());
    let src = Path::new(CHECKOUT_DIR)
        .join("sbe-all").join("build").join("libs").join(&jar);

    let dst = Path::new(&jar);
    std::fs::copy(src, dst)?;
    std::fs::copy(version_file, super::SBE_VERSION_FILE)?;

    Ok(())
}

/// Clone the SBE repository
pub fn clone() -> Result<()> {

    let state = RefCell::new(State {
        progress: None,
        total: 0,
        current: 0,
        path: None,
        newline: false,
    });
    let mut cb = RemoteCallbacks::new();
    cb.transfer_progress(|stats| {
        let mut state = state.borrow_mut();
        state.progress = Some(stats.to_owned());
        print(&mut *state);
        true
    });
    
    let mut co = CheckoutBuilder::new();
    co.progress(|path, cur, total| {
        let mut state = state.borrow_mut();
        state.path = path.map(|p| p.to_path_buf());
        state.current = cur;
        state.total = total;
        print(&mut *state);
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    RepoBuilder::new()
        .fetch_options(fo)
        .with_checkout(co)
        .clone(GIT_URL, Path::new(CHECKOUT_DIR))?;
    println!();

    Ok(())
}

/// Print progress of the git clone
fn print(state: &mut State) {
    let stats = state.progress.as_ref().unwrap();
    let network_pct = (100 * stats.received_objects()) / stats.total_objects();
    let index_pct = (100 * stats.indexed_objects()) / stats.total_objects();
    let co_pct = if state.total > 0 {
        (100 * state.current) / state.total
    } else {
        0
    };
    let kbytes = stats.received_bytes() / 1024;
    if stats.received_objects() == stats.total_objects() {
        if !state.newline {
            println!();
            state.newline = true;
        }
        print!(
            "Resolving deltas {}/{}\r",
            stats.indexed_deltas(),
            stats.total_deltas()
        );
    } else {
        print!(
            "net {:3}% ({:4} kb, {:5}/{:5})  /  idx {:3}% ({:5}/{:5})  \
             /  chk {:3}% ({:4}/{:4}) {}\r",
            network_pct,
            kbytes,
            stats.received_objects(),
            stats.total_objects(),
            index_pct,
            stats.indexed_objects(),
            stats.total_objects(),
            co_pct,
            state.current,
            state.total,
            state
                .path
                .as_ref()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default()
        )
    }
    io::stdout().flush().unwrap();
}