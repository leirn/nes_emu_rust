[![Clippy check](https://github.com/leirn/nes_emu_rust/actions/workflows/clippy.yml/badge.svg)](https://github.com/leirn/nes_emu_rust/actions/workflows/clippy.yml)

# nes_emu_rust
NES Emulator in Rest

This repository intends to port nes_emu_py once it will be working enough.
This a sandbox to learn programming in Rust.

Current status:
- CPU : fully functional and compliant to nestest
- PPU : implementation in progress. Probably 80 to 90% done
- APU : not started
- Inputs : Just started, not pluged yet. 20% done
- Mappers : only mapper 0 hardcoded


# Setup development environment

- Go to latest SDL releases : https://github.com/libsdl-org/SDL/releases/tag/release-2.24.0
- Download SDL2-devel-2.x.y-VC.zip
- Copy lib/x64/*.lib into C:\Users\\{Your Username}\\.rustup\toolchains\\{current toolchain}\lib\rustlib\\{current toolchain}\lib

More information on https://crates.io/crates/sdl2
