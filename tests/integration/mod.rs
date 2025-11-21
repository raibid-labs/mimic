//! Integration tests module.

mod basic;
mod errors;
mod events;
mod process;
mod wait_conditions;

#[cfg(feature = "sixel")]
mod sixel;

#[cfg(feature = "bevy")]
mod bevy;
