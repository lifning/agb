# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Support for sprites that are not square.

## [0.10.0] - 2022/07/31

Version 0.10.0 brings about many new features. As with most `agb` upgrades, you will need to update your `gba.ld` and `gba_mb.ld` files which you can find in the [template repo](https://github.com/agbrs/template). We would also recommend copying the `[profile.dev]` and `[profile.release]` sections from `Cargo.toml` if you don't have these values already.

### Added
- [Hyperspace roll](https://lostimmortal.itch.io/hyperspace-roll), a new game built for the GMTK Game Jam 2022 using `agb`. The source code can be found in the `examples` directory.
- Started using GitHub discussions as a forum
- Many functions previously undocumented are now documented
- Z-Ordering of sprites - you can now change the render order of sprites rather than it just being defined by the order in which they appear in object memory
- 32kHz audio. Probably the best sound quality you'll get out of the hardware, but uses a lot of ROM space
- Transparent sprite support with aseprite
- You can now write tests for projects depending on agb
- Very basic font rendering - looking for feedback, this API is far from stable
- Faster implementation of memcpy and memset thanks to the agbabi crate which provide a big performance boost for any project using agb
- If you wish, you can now optionally do dynamic memory allocation to IWRAM instead of only EWRAM
- You can now use 64x64px sprites
- You can now configure the background size for tiled backgrounds
- It is possible to create 'dynamic tiles' for backgrounds. These are tiles which are defined at runtime
- Random number generator in agb::rng

### Changed
- Audio system optimisations - reduced CPU usage by more than 50%
- Background tiles are now removed from Video RAM during `commit()` if they are no longer used rather than immediately reducing flickering
- Improved the README for both the main agb crate and the template
- The template now builds with optimisations in debug mode and debug symbols in release mode
- Added `#[must_use]` to many of the places it is needed
- All subcrates get released at once, so versions are kept in lockstep
- A few methods accepting `Num<..>` have been changed to accept `impl Into<Num<..>>` to make them easier to use

### Removed
- The ability to use timer0 and timer1 through the `timer` module. This was done in order to fully support 32kHz audio

### Fixed
- Sprite data is now correctly aligned so fast copies will always work
- A few methods which should really be internal have had `pub` removed
- The crate now compiles (but does not run) doctests in CI which pointed out a large number of non-compiling examples