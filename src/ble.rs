// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

use trouble_host::prelude::*;

pub mod advertise;
pub mod gatt_server;
mod services;

/// This device can service only one connection.
const MAX_CONNECTIONS: usize = 1;

/// This device will advertise the same data each advertising window, so
/// multiple advertising sets are not needed.
const MAX_ADVERTISING_SETS: usize = 1;

/// Two channels will be required for L2CAP transfers (Signal + ATT).
const MAX_L2CAP_CHANNELS: usize = 2;

/// Maximum size of packets sent over the L2CAP channels.
const L2CAP_MTU: usize = 251;

/// Resource pool supporting communication between the BLE Host and BLE
/// controller.
type BleResources =
    HostResources<DefaultPacketPool, MAX_CONNECTIONS, MAX_L2CAP_CHANNELS, MAX_ADVERTISING_SETS>;

/// Controller part of the BLE host controller interface.
/// The controller is specific to a board's particular radio hardware and is
/// provided by the board's support module.
pub type BleController = super::boards::BleController;

/// Error type for the BLE host controller interface.
/// This error type is specific to the board's particular controller and is
/// provided by the board's support module.
type BleHostError = super::boards::BleHostError;

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
            trouble_host::BleHostError::Controller(_) => {
                defmt::panic!("[ble_task] error occured in the BLE controller.")
            }
            trouble_host::BleHostError::BleHost(host_error) => {
                defmt::panic!("[ble_task] error occured in the BLE host: {}", host_error)
            }
        }
    }
}
