# Keep Awake: a COSMIC panel applet

Click to toggle. When on, the machine won't auto-suspend (handy for long
remote / Claude Code sessions), but **idle is not inhibited**, so monitors
still switch off and the screen still locks.

It does two things while on:

- holds a logind `sleep` **block** inhibitor (stops suspends requested by
  other users or root), and
- sets COSMIC's `suspend_on_ac_time` / `suspend_on_battery_time`
  (`~/.config/cosmic/com.system76.CosmicIdle/v1/`) to `None`. This part is
  needed because cosmic-idle suspends with `systemctl suspend` as *you*, and
  logind lets a user override their own block inhibitors.

The original timeouts are saved in `~/.local/state/cosmic-applet-awake/idle-backup/`
and put back when you switch it off, or on the next start if the applet died
while on. If you change the suspend time in Settings while it's on, your new
value is kept.

The on/off state is saved in `~/.local/state/cosmic-applet-awake/enabled`,
so it survives reboots, logouts, panel restarts and updates.

## Build & install

    sudo apt install rustup just libxkbcommon-dev libfontconfig-dev libfreetype-dev libexpat1-dev
    rustup default stable
    just install

Then add **Keep Awake** in Settings → Desktop → Panel → Applets.

Check it's working: `systemd-inhibit --list` shows a `Keep Awake … sleep … block` row,
and `cat ~/.config/cosmic/com.system76.CosmicIdle/v1/suspend_on_ac_time` says `None`.
