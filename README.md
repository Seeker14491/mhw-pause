# Monster Hunter: World pause tool

Monster Hunter: World has no pause option in-game; this is a small tool that will suspend the game process, effectively pausing it. Just run `mhw-pause.exe` to pause, and to resume press enter in the console window that opens.

## Building from source

You need Rust installed: https://rustup.rs/. Then just run

```
cargo build --release
```

in this directory, and the built executable should be at `target\release\mhw-pause.exe`.
