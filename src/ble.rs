// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

use trouble_host::prelude::*;

use crate::boards::BleController;

mod gatt_server;
mod services;

/// This device can service only one connection.
const MAX_CONNECTIONS: usize = 1;

/// This device will advertise the same data each advertising window, so
/// multiple advertising sets are not needed.
const MAX_ADVERTISING_SETS: usize = 1;

/// Two channels will be required for L2CAP transfers (Signal + ATT).
const MAX_L2CAP_CHANNELS: usize = 2;

/// The ESP32C3's WIFI module is hardcoded to an MTU of 251.
///
/// As of this writing an issue is open in esp_hal, requesting the ability to
/// configure this value: https://github.com/esp-rs/esp-hal/issues/2984
///
/// An MTU of 251 is sufficient for our purposes and may be used as is on
/// other boards as well.
const L2CAP_MTU: usize = 251;

/// Resource pool supporting communication between the BLE Host and BLE
/// controller.
type BleResources =
    HostResources<DefaultPacketPool, MAX_CONNECTIONS, MAX_L2CAP_CHANNELS, MAX_ADVERTISING_SETS>;

/// Background task that pumps the BLE stack's event loop.
///
/// # Panic
///
/// Any errors that occur in the BLE event loop are likely unrecoverable and
/// will result in a panic.
#[embassy_executor::task]
async fn ble_task(mut runner: Runner<'static, BleController, DefaultPacketPool>) {
    if let Err(error) = runner.run().await {
        match error {
            // The controller does not provide an error type.
            BleHostError::Controller(_) => {
                defmt::panic!("[ble_task] error occured in the BLE controller.")
            }
            BleHostError::BleHost(host_error) => {
                defmt::panic!("[ble_task] error occured in the BLE host: {}", host_error)
            }
        }
    }
}
