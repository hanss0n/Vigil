bin := "cosmic-applet-awake"
id := "io.github.bear.CosmicAppletAwake"
prefix := env_var('HOME') / ".local"

build:
    cargo build --release

# Install for the current user (no sudo)
install: build
    install -Dm755 target/release/{{bin}} {{prefix}}/bin/{{bin}}
    sed "s|^Exec=.*|Exec={{prefix}}/bin/{{bin}}|" res/{{id}}.desktop > /tmp/{{id}}.desktop
    install -Dm644 /tmp/{{id}}.desktop {{prefix}}/share/applications/{{id}}.desktop
    for f in res/icons/*.svg; do install -Dm644 "$f" {{prefix}}/share/icons/hicolor/scalable/apps/$(basename "$f"); done

uninstall:
    rm -f {{prefix}}/bin/{{bin}} {{prefix}}/share/applications/{{id}}.desktop
    rm -f {{prefix}}/share/icons/hicolor/scalable/apps/{{id}}-*.svg
