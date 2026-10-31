//! Remembers whether the applet was switched on, so it survives reboots,
//! panel restarts and updates.

use std::path::PathBuf;

fn path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/state")))?;
    Some(base.join("cosmic-applet-awake/enabled"))
}

pub fn load() -> bool {
    path()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .is_some_and(|s| s.trim() == "1")
}

pub fn save(enabled: bool) {
    let Some(p) = path() else { return };
    if let Some(dir) = p.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if let Err(e) = std::fs::write(&p, if enabled { "1\n" } else { "0\n" }) {
        eprintln!("cosmic-applet-awake: failed to save state to {}: {e}", p.display());
    }
}
