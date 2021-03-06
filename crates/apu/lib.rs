//! This crate provides GameBoy's APU emulation. It includes APU controlling API via *registers* and sound processing emulation layer. The emulation layer has no dependency about platform-specific sound I/O API e.g. ALSA for GNU/Linux.
//!
//! References:
//! - <https://gbdev.gg8.se/wiki/articles/Gameboy_sound_hardware>
//! - <https://aselker.github.io/gameboy-sound-chip/>

pub mod apu;
pub mod types;
mod util;
