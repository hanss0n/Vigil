//! Pauses COSMIC's idle suspend while Keep Awake is on.
//!
//! The logind inhibitor alone does not stop it: cosmic-idle suspends by
//! running `systemctl suspend` as the user without a tty, so systemctl skips
//! its inhibitor check, and logind lets a user override block inhibitors
//! they hold themselves. So while on, we set cosmic-idle's suspend timeouts
//! to `None` (it watches its config live) and put the old values back when
//! switched off.
//!
//! The original values are saved under `~/.local/state`, which survives a
//! reboot, so if the applet dies while on they are still there to restore
//! when it is switched off (or starts in the off state).

use std::io;
use std::path::{Path, PathBuf};

const KEYS: [&str; 2] = ["suspend_on_ac_time", "suspend_on_battery_time"];
const PAUSED: &str = "None";

fn home_dir(var: &str, fallback: &str) -> Option<PathBuf> {
    std::env::var_os(var)
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(fallback)))
}

fn config_dir() -> Option<PathBuf> {
    Some(home_dir("XDG_CONFIG_HOME", ".config")?.join("cosmic/com.system76.CosmicIdle/v1"))
}

fn backup_dir() -> Option<PathBuf> {
    Some(home_dir("XDG_STATE_HOME", ".local/state")?.join("cosmic-applet-awake/idle-backup"))
}

/// Replace `path` atomically, so cosmic-idle never reads a half-written value.
fn write_atomic(path: &Path, contents: &str) -> io::Result<()> {
    let dir = path.parent().expect("config path has a parent");
    std::fs::create_dir_all(dir)?;
    let tmp = dir.join(format!(".{}.awake-tmp", path.file_name().unwrap().to_string_lossy()));
    std::fs::write(&tmp, contents)?;
    std::fs::rename(&tmp, path)
}

fn read_opt(path: &Path) -> io::Result<Option<String>> {
    match std::fs::read_to_string(path) {
        Ok(s) => Ok(Some(s)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

/// Turn off idle suspend, saving the current settings first.
pub fn pause() -> io::Result<()> {
    let (Some(cfg), Some(bak)) = (config_dir(), backup_dir()) else {
        return Err(io::Error::other("HOME is not set"));
    };
    std::fs::create_dir_all(&bak)?;
    for key in KEYS {
        let saved = bak.join(key);
        // A backup left over from an earlier run already holds the user's
        // real value; the config may now say None because of us.
        if !saved.exists() {
            // An empty backup means "the key was unset" (an empty config
            // file is never a valid value).
            let current = read_opt(&cfg.join(key))?.unwrap_or_default();
            write_atomic(&saved, &current)?;
        }
        write_atomic(&cfg.join(key), PAUSED)?;
    }
    Ok(())
}

/// Put back the settings saved by `pause`. Does nothing if none are saved.
/// A key the user changed meanwhile (no longer `None`) is left alone.
pub fn resume() -> io::Result<()> {
    let (Some(cfg), Some(bak)) = (config_dir(), backup_dir()) else {
        return Ok(());
    };
    for key in KEYS {
        let saved = bak.join(key);
        let Some(original) = read_opt(&saved)? else { continue };
        let path = cfg.join(key);
        if read_opt(&path)?.is_some_and(|s| s.trim() == PAUSED) {
            if original.is_empty() {
                std::fs::remove_file(&path)?;
            } else {
                write_atomic(&path, &original)?;
            }
        }
        std::fs::remove_file(&saved)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pause_then_resume_restores_settings() {
        let root = std::env::temp_dir().join(format!("awake-idle-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        // SAFETY: this is the only test that touches the environment.
        unsafe {
            std::env::set_var("XDG_CONFIG_HOME", root.join("config"));
            std::env::set_var("XDG_STATE_HOME", root.join("state"));
        }
        let cfg = config_dir().unwrap();
        std::fs::create_dir_all(&cfg).unwrap();
        std::fs::write(cfg.join("suspend_on_ac_time"), "Some(1800000)").unwrap();
        // suspend_on_battery_time left unset (COSMIC default).

        pause().unwrap();
        pause().unwrap(); // a second pause (restart while on) must keep the real backup
        assert_eq!(std::fs::read_to_string(cfg.join("suspend_on_ac_time")).unwrap(), "None");
        assert_eq!(std::fs::read_to_string(cfg.join("suspend_on_battery_time")).unwrap(), "None");

        resume().unwrap();
        assert_eq!(std::fs::read_to_string(cfg.join("suspend_on_ac_time")).unwrap(), "Some(1800000)");
        assert!(!cfg.join("suspend_on_battery_time").exists());
        resume().unwrap(); // no backups left: no-op

        // A value the user changes while paused is kept.
        pause().unwrap();
        std::fs::write(cfg.join("suspend_on_ac_time"), "Some(600000)").unwrap();
        resume().unwrap();
        assert_eq!(std::fs::read_to_string(cfg.join("suspend_on_ac_time")).unwrap(), "Some(600000)");

        std::fs::remove_dir_all(&root).unwrap();
    }
}
