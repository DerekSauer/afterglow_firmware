// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

#![feature(impl_trait_in_assoc_type)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]
#![no_main]
#![no_std]

use {defmt as _, defmt_rtt as _, esp_backtrace as _};

#[esp_hal_embassy::main]
async fn main(task_spawner: embassy_executor::Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let timer_group_0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timer_group_0.timer0);

    loop {
        defmt::info!("Hello ESP32!");
        embassy_time::Timer::after_secs(1).await;
    }
}
