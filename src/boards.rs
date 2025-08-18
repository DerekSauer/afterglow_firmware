// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Boards this firmware supports.

#[cfg(feature = "nougat-c3")]
mod nougat_c3;

#[cfg(feature = "nougat-c3")]
pub type BleController = nougat_c3::BleControllerImpl;
#[cfg(feature = "nougat-c3")]
pub type BleHostError = nougat_c3::BleHostErrorImpl;
#[cfg(feature = "nougat-c3")]
pub type Board = nougat_c3::Board;
#[cfg(feature = "nougat-c3")]
pub type RngDriver = esp_hal::rng::Trng<'static>;
