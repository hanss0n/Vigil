//! Holds a logind "sleep" block inhibitor.
//!
//! Only `sleep` is inhibited, never `idle`, so the screen can still blank
//! and lock while the machine stays reachable over the network.
//! The lock lasts as long as the returned file descriptor is open; dropping it
//! releases the lock (and so does the process exiting or crashing).

use std::os::fd::OwnedFd;

pub fn acquire() -> zbus::Result<OwnedFd> {
    let conn = zbus::blocking::Connection::system()?;
    let reply = conn.call_method(
        Some("org.freedesktop.login1"),
        "/org/freedesktop/login1",
        Some("org.freedesktop.login1.Manager"),
        "Inhibit",
        &("sleep", "Keep Awake", "Keeping the system reachable", "block"),
    )?;
    let fd: zbus::zvariant::OwnedFd = reply.body().deserialize()?;
    Ok(fd.into())
}
