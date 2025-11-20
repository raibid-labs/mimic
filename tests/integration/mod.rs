//! Integration tests module.

mod basic;

#[cfg(feature = "sixel")]
mod sixel;

#[cfg(feature = "bevy")]
mod bevy;
