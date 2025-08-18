// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

use static_cell::StaticCell;
use trouble_host::prelude::*;

use super::services::device_information::DeviceInformation;

#[gatt_server]
pub struct GattServer {
    pub device_information: DeviceInformation,
}

impl<'stack> GattServer<'stack> {
    /// Start the Gatt server.
    ///
    /// # Panic
    ///
    /// This function will panic if the Gatt Server cannot be started.
    pub fn start_server(
        task_spawner: &embassy_executor::Spawner,
        mac_address: [u8; 6],
        ble_controller: super::BleController,
        rng: &mut crate::boards::RngDriver,
    ) -> (
        &'static Self,
        Peripheral<'stack, super::BleController, DefaultPacketPool>,
    ) {
        let mac_address = Address::random(mac_address);

        let hci_resources = {
            static HCI_RESOURCES: StaticCell<super::BleResources> = StaticCell::new();
            HCI_RESOURCES.init_with(|| super::BleResources::new())
        };

        let ble_stack = {
            static BLE_STACK: StaticCell<Stack<'_, super::BleController, DefaultPacketPool>> =
                StaticCell::new();
            BLE_STACK.init_with(|| {
                trouble_host::new(ble_controller, hci_resources)
                    .set_random_address(mac_address)
                    .set_random_generator_seed(rng)
            })
        };

        let host = ble_stack.build();

        let gatt_server = {
            static GATT_SERVER: StaticCell<GattServer<'_>> = StaticCell::new();
            GATT_SERVER.init_with(|| {
                match GattServer::new_with_config(GapConfig::Peripheral(PeripheralConfig {
                    name:       crate::MODEL_NUMBER,
                    appearance: &appearance::light_fixtures::LIGHT_CONTROLLER,
                })) {
                    Ok(gatt_server) => gatt_server,
                    Err(err) => {
                        defmt::panic!("error while starting the Gatt server: {}", err)
                    }
                }
            })
        };

        task_spawner.must_spawn(super::ble_task(host.runner));

        (gatt_server, host.peripheral)
    }

    pub async fn gatt_event_loop<'gatt_server>(
        &self,
        connection: &GattConnection<'stack, 'gatt_server, DefaultPacketPool>,
    ) -> Result<(), trouble_host::Error> {
        let disconnect_reason = loop {
            match connection.next().await {
                GattConnectionEvent::Disconnected { reason } => break reason,
                GattConnectionEvent::Gatt { event } => match event.accept() {
                    Ok(reply) => reply.send().await,
                    Err(gatt_error) => {
                        defmt::warn!("[gatt] error processing a request: {:?}", gatt_error)
                    }
                },
                _ => {}
            }
        };

        Ok(())
    }
}
