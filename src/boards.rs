// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Boards this firmware supports.

#[cfg(feature = "nougat-c3")]
mod nougat_c3;

#[cfg(feature = "nougat-c3")]
pub type Board<'board> = nougat_c3::Board<'board>;
