# Vigil

**Keep your COSMIC desktop from going to sleep, with one click.** Vigil adds a "Keep Awake" button to the panel. While it's on, the computer won't suspend by itself, but the screen still turns off and locks as usual.

It's handy when something needs to keep running while you're away: a long download, a build, or a remote session you connect to from another device.

## Why this exists

COSMIC doesn't have a "keep awake" or caffeine switch yet. The usual trick of asking the system to block sleep doesn't help either: COSMIC's own idle timer suspends the computer as *you*, and the system lets you override your own block.

So Vigil does both. It blocks sleep the usual way, which stops suspends from other sources, and it temporarily turns off COSMIC's own "suspend when idle" setting. When you switch it off, your normal setting comes back.

## How it behaves

- **Click the icon to switch it on or off.** The icon shows which state it's in.
- **The screen still turns off and locks.** Only suspend is blocked, so your screen isn't left on and unlocked.
- **It stays on until you switch it off**, even across reboots and logouts.
- **Your settings are safe.** Vigil remembers your original suspend setting and puts it back when you switch it off, even if the panel crashed in between. If you change the suspend time in Settings while Vigil is on, your new value is kept.

## Installing

1. Install the build tools:

   ```
   sudo apt install just libxkbcommon-dev libfontconfig-dev libfreetype-dev libexpat1-dev
   ```

   You also need the Rust toolchain ([rustup.rs](https://rustup.rs)).

2. Build and install. Everything goes into your home folder, so no `sudo` is needed:

   ```
   just install
   ```

3. Open **Settings → Desktop → Panel → Configure panel applets**, click **Add applet**, and pick **Keep Awake**. If it isn't in the list, log out and back in.

## Uninstalling

Switch Vigil off first, so your normal suspend setting is restored. Then remove it from the panel in **Settings → Desktop → Panel**, and run:

```
just uninstall
```

## Checking that it works

With Vigil switched on, both of these should agree:

```
systemd-inhibit --list    # shows a "Keep Awake … sleep … block" row
cat ~/.config/cosmic/com.system76.CosmicIdle/v1/suspend_on_ac_time    # says None
```

## For developers

Vigil is a small Rust panel applet built on [libcosmic](https://github.com/pop-os/libcosmic).

```
src/
  main.rs      the applet: icon, click to toggle
  inhibit.rs   the system sleep block (logind)
  idle.rs      pauses and restores COSMIC's idle suspend setting
  state.rs     remembers whether it's on
res/           panel entry and icons
```

It keeps its files in `~/.local/state/cosmic-applet-awake/`: `enabled` holds the on/off state, and `idle-backup/` holds your original suspend setting while Vigil is on.

```
cargo test --release    # run the tests
just install            # build and install for your user
```
