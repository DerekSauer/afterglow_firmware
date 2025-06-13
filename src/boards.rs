// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Boards this firmware is intended to support.

#[cfg(feature = "nougat-c3")]
mod nougat_c3;

#[cfg(feature = "nougat-c3")]
pub use nougat_c3::Board;
#[cfg(feature = "nougat-c3")]
pub use nougat_c3::ClockSpeed;
