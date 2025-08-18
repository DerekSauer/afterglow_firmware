// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

// use trouble_host::prelude::*;

use static_cell::StaticCell;
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

type BleResources =
    HostResources<DefaultPacketPool, MAX_CONNECTIONS, MAX_L2CAP_CHANNELS, MAX_ADVERTISING_SETS>;

/// Resource pool supporting communication between the BLE Host and BLE
/// controller.
static RESOURCES: StaticCell<BleResources> = StaticCell::new();

/// Host side of the BLE Host/Controller Interface.
pub struct BleHost<'stack, C: Controller> {
    stack: Stack<'stack, C, DefaultPacketPool>,
}

impl<'stack, C: Controller> BleHost<'stack, C> {
    /// Construct a new BLE Host.
    ///
    /// The host requires a MAC address, BLE controller, and source of
    /// randomness to seed its own random number generator.
    ///
    /// The controller is hardware specific and will need to be intialized
    /// separately.
    pub fn new<R>(mac_addr: [u8; 6], ble_controller: C, random_source: &mut R) -> Self
    where
        C: Controller,
        R: rand_core::RngCore + rand_core::CryptoRng,
    {
        let mac_address = Address::random(mac_addr);
        let resources = RESOURCES.init_with(|| BleResources::new());

        let stack = trouble_host::new(ble_controller, resources)
            .set_random_address(mac_address)
            .set_random_generator_seed(random_source);

        Self { stack }
    }

    /// Run the BLE Host.
    ///
    /// The returned peripheral allows for advertising. The returned runner must
    /// run the `ble_task` so that BLE events are processed.
    pub fn run(
        &'stack self,
    ) -> (
        Peripheral<'stack, C, DefaultPacketPool>,
        Runner<'stack, C, DefaultPacketPool>,
    ) {
        let host = self.stack.build();

        (host.peripheral, host.runner)
    }
}

/// Background task that pumps the BLE stack's event loop.
///
/// This task must be run alongside other BLE tasks. Recommend joining it with
/// the advertising task.
///
/// # Panic
///
/// Any errors that occur in the BLE event loop are likely unrecoverable and
/// will result in a panic.
pub async fn ble_task<C: Controller, P: PacketPool>(mut runner: Runner<'_, C, P>) {
    if let Err(error) = runner.run().await {
        match error {
            BleHostError::Controller(_) => {
                defmt::panic!("[ble_task] error occured in the BLE controller.")
            }
            BleHostError::BleHost(host_error) => {
                defmt::panic!("[ble_task] error occured in the BLE host: {}", host_error)
            }
        }
    }
}
