// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

#![feature(impl_trait_in_assoc_type)] // Needed by embassy-executor's statically allocated tasks.
#![feature(type_alias_impl_trait)] // Needed by static_cell::make_static!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]
#![no_main]
#![no_std]

mod ble;
mod boards;

use {defmt as _, defmt_rtt as _};
#[cfg(feature = "esp")]
use {esp_alloc as _, esp_backtrace as _};

use crate::boards::Board;

#[cfg(feature = "esp")]
#[esp_hal_embassy::main]
async fn main(_task_spawner: embassy_executor::Spawner) {
    let board = Board::init();
}
