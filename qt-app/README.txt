
1. brew reinstall qt@5
2. echo 'export PATH="/opt/homebrew/opt/qt@5/bin:$PATH"' >> ~/.zshrc

If you need to have qt@5 first in your PATH, run:
  echo 'export PATH="/opt/homebrew/opt/qt@5/bin:$PATH"' >> ~/.zshrc

For compilers to find qt@5 you may need to set:
  export LDFLAGS="-L/opt/homebrew/opt/qt@5/lib"
  export CPPFLAGS="-I/opt/homebrew/opt/qt@5/include"

For pkg-config to find qt@5 you may need to set:
  export PKG_CONFIG_PATH="/opt/homebrew/opt/qt@5/lib/pkgconfig"
==> Summary
🍺  /opt/homebrew/Cellar/qt@5/5.15.8_2: 10,840 files, 344.4MB
==> Running `brew cleanup qt@5`...
Disable this behaviour by setting HOMEBREW_NO_INSTALL_CLEANUP.
Hide these hints with HOMEBREW_NO_ENV_HINTS (see `man brew`).
