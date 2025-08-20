// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

#![feature(impl_trait_in_assoc_type)] // Needed by embassy-executor's statically allocated tasks.
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]
#![no_main]
#![no_std]

mod ble;
mod boards;

use embassy_futures::join::join;
use {defmt as _, defmt_rtt as _};
#[cfg(feature = "esp")]
use {esp_alloc as _, esp_backtrace as _};

use crate::boards::Board;

/// Name of the manufacturer of the device.
static MANUFACTURER_NAME: &str = "Sauerstoff.ca";

/// Model number or name of the device.
static MODEL_NUMBER: &str = "Afterglow-01";

/// The device's serial number.
static SERIAL_NUMBER: &str = "AG-202507-0001";

/// This firmware's version.
static FIRMWARE_REVISION: &str = env!("CARGO_PKG_VERSION");

/// Hardware revision name or number of this device.
#[cfg(feature = "nougat-c3")]
static HARDWARE_REVISION: &str = if cfg!(feature = "nougat-c3") {
    "nougat-c3"
} else {
    "unknown"
};

#[cfg(feature = "esp")]
#[esp_hal_embassy::main]
async fn main(_task_spawner: embassy_executor::Spawner) {
    let mut board = match Board::init() {
        Ok(board) => board,
        Err(error) => defmt::panic!("Could not initialize the board: {}", error),
    };

    let ble_host = ble::BleHost::new(
        board.get_mac_address(),
        board.ble_controller,
        &mut board.rng,
    );

    let (mut peripheral, ble_runner) = ble_host.run();

    let gatt_server = ble::gatt_server::GattServer::start(MODEL_NUMBER);

    let _ = join(ble::ble_task(ble_runner), async {
        loop {
            match ble::advertise::advertise(MODEL_NUMBER, &mut peripheral, &gatt_server).await {
                Ok(connection) => {
                    gatt_server.gatt_event_loop(&connection).await.unwrap();
                }
                Err(err) => {
                    let wrapped_err = defmt::Debug2Format(&err);
                    defmt::panic!("[ble] advertising error: {:?}", wrapped_err)
                }
            }
        }
    })
    .await;
}
