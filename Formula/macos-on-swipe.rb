class MacosOnSwipe < Formula
  desc "Detect three-finger trackpad swipe gestures and execute a script"
  homepage "https://github.com/foysavas/macos-on-swipe"
  head "https://github.com/foysavas/macos-on-swipe.git", branch: "main"
  license "MIT"

  depends_on "rust" => :build
  depends_on :macos

  def install
    system "cargo", "install", *std_cargo_args
  end

  service do
    run opt_bin/"macos-on-swipe"
    keep_alive true
    log_path var/"log/macos-on-swipe.log"
    error_log_path var/"log/macos-on-swipe.log"
  end

  def caveats
    <<~EOS
      macos-on-swipe requires Accessibility permissions to detect trackpad gestures.

      After first run, grant permission in:
        System Settings → Privacy & Security → Accessibility

      On newer macOS versions, you may also need Input Monitoring permission:
        System Settings → Privacy & Security → Input Monitoring

      Create your handler script at:
        ~/.config/macos-on-swipe/handle-swipe.sh

      Example:
        mkdir -p ~/.config/macos-on-swipe
        echo '#!/bin/bash
        echo "Swipe: $1"' > ~/.config/macos-on-swipe/handle-swipe.sh
        chmod +x ~/.config/macos-on-swipe/handle-swipe.sh

      To start the service (runs at login):
        brew services start macos-on-swipe

      To stop the service:
        brew services stop macos-on-swipe
    EOS
  end

  test do
    assert_match "macos-on-swipe", shell_output("#{bin}/macos-on-swipe --help 2>&1", 1)
  end
end
