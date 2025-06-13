// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

// Needed by embassy-executor's nightly feature to statically allocate tasks at
// compile time without needing a fixed sized task arena.
#![feature(impl_trait_in_assoc_type)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]
#![no_main]
#![no_std]

mod boards;

use {defmt as _, defmt_rtt as _};
#[cfg(feature = "esp")]
use {esp_alloc as _, esp_backtrace as _};

#[cfg(feature = "esp")]
#[esp_hal_embassy::main]
async fn main(_task_spawner: embassy_executor::Spawner) {
    let board = boards::Board::new(boards::ClockSpeed::Low80Mhz);
}
