# macos-on-swipe

A macOS application that detects three-finger trackpad swipe gestures and executes a script with the swipe direction.

## Features

- Detects three-finger swipe gestures globally (system-wide)
- Runs as a background process (no dock icon)
- Executes `~/.config/macos-on-swipe/handle-swipe.sh` with direction argument (`left`, `right`, `up`, `down`)

## Installation

### Homebrew

```bash
brew install --HEAD foysavas/macos-on-swipe/macos-on-swipe
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

case "$direction" in
    left)
        echo "Swiped left!"
        ;;
    right)
        echo "Swiped right!"
        ;;
    up)
        echo "Swiped up!"
        ;;
    down)
        echo "Swiped down!"
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
