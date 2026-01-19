# macos-on-swipe

A macOS application that detects three-finger trackpad swipe gestures and executes a script with the swipe direction.

## Features

- Detects three-finger swipe gestures globally (system-wide)
- Runs as a background process (no dock icon)
- Executes `~/.config/macos-on-swipe/handle-swipe.sh` with two arguments: direction (`left`, `right`, `up`, `down`) and finger count

## Installation

### Homebrew

```bash
brew install --HEAD foysavas/tap/macos-on-swipe
```

To start automatically at login:
```bash
brew services start macos-on-swipe
```

### From Source

Requires Rust toolchain (install via [rustup](https://rustup.rs/)):

```bash
git clone https://github.com/foysavas/macos-on-swipe.git
cd macos-on-swipe
cargo build --release
```

The binary will be at `target/release/macos-on-swipe`.

## Setup

1. Create your handler script at `~/.config/macos-on-swipe/handle-swipe.sh`:

```bash
mkdir -p ~/.config/macos-on-swipe

cat > ~/.config/macos-on-swipe/handle-swipe.sh << 'EOF'
#!/bin/bash
direction="$1"
fingers="$2"

echo "Swiped $direction with $fingers fingers"

case "$direction" in
    left)
        # Handle left swipe
        ;;
    right)
        # Handle right swipe
        ;;
    up)
        # Handle up swipe
        ;;
    down)
        # Handle down swipe
        ;;
esac
EOF

chmod +x ~/.config/macos-on-swipe/handle-swipe.sh
```

2. Run the app:

```bash
macos-on-swipe
```

3. Grant **Accessibility** permission when prompted (System Settings → Privacy & Security → Accessibility)

4. On newer macOS versions, you may also need **Input Monitoring** permission (System Settings → Privacy & Security → Input Monitoring)

## Configuration

Optionally create a config file at `~/.config/macos-on-swipe/config.toml`:

```toml
# Minimum fingers required for swipe (default: 3)
min_fingers = 3

# Ignore touches within this margin of trackpad edges (0.0 - 0.5)
# Higher = larger ignore zone (helps with palm rejection)
# Default: 0.0
edge_margin = 0.1

# Minimum touch pressure to count as a finger (0.0 - 1.0)
# Higher = ignore lighter touches (helps with palm rejection)
# Default: 0.0
min_pressure = 0.0

# Cooldown after two-finger gestures (milliseconds)
# Ignores multi-finger gestures for this duration after a two-finger gesture ends
# Helps prevent accidental swipes when lifting fingers from scroll
# Default: 0 (disabled)
two_finger_cooldown_ms = 200

# Swipe sensitivity thresholds per direction (0.0 - 1.0)
# Lower = more sensitive, Higher = less sensitive
# Default: 0.10 (10% of trackpad)
left = 0.10
right = 0.10
up = 0.10
down = 0.10
```

All settings are optional and will use defaults if not specified.

## Trackpad Settings

By default, macOS uses three-finger swipes for system gestures. To avoid conflicts:

1. Open **System Settings** → **Trackpad** → **More Gestures**
2. Set "Swipe between full-screen apps" to **Off** or **Four Fingers**
3. Set "Mission Control" to **Off** or **Four Fingers**

## Running at Login

### With Homebrew

```bash
brew services start macos-on-swipe
```

To stop:
```bash
brew services stop macos-on-swipe
```

### Manual Setup

If you built from source, create `~/Library/LaunchAgents/com.user.macos-on-swipe.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.user.macos-on-swipe</string>
    <key>ProgramArguments</key>
    <array>
        <string>/path/to/macos-on-swipe</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

Load with: `launchctl load ~/Library/LaunchAgents/com.user.macos-on-swipe.plist`

## License

MIT
