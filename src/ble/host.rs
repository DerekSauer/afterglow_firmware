// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

use static_cell::make_static;
use trouble_host::prelude::*;

use super::BleResources;

/// Bluetooth Low Energy host.
/// Provides access to the BLE peripheral allowing advertising and connections
/// to occur.
pub struct Host<'stack, C: Controller> {
    /// BLE stack comprised of a host, controller, and shared resource pool.
    stack: Stack<'stack, C, DefaultPacketPool>,
}

impl<'stack, C: Controller> Host<'stack, C> {
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
        let resources = make_static!(BleResources::new());

        let stack = trouble_host::new(ble_controller, resources)
            .set_random_address(mac_address)
            .set_random_generator_seed(random_source);

        Self { stack }
    }

    /// Run the BLE Host.
    ///
    /// The returned peripheral allows for advertising. The returned runner must
    /// run the [ble_background_task](super::ble_background_task) so that BLE
    /// events are processed.
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
