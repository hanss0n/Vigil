# Keep Awake: a COSMIC panel applet

Click to toggle. When on, it holds a logind `sleep` **block** inhibitor, so
the machine won't auto-suspend (handy for long remote / Claude Code sessions),
but **idle is not inhibited**, so monitors still switch off and the screen still locks.

The on/off state is saved in `~/.local/state/cosmic-applet-awake/enabled`
and restored when the panel starts, so it survives reboots and updates.

## Build & install

    sudo apt install rustup just libxkbcommon-dev libfontconfig-dev libfreetype-dev libexpat1-dev
    rustup default stable
    just install

Then add **Keep Awake** in Settings → Desktop → Panel → Applets.

Check it's working: `systemd-inhibit --list` shows a `Keep Awake … sleep … block` row.
