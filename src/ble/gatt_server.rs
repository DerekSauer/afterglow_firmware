// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

use trouble_host::prelude::*;

use super::services::device_information::DeviceInformation;

#[gatt_server]
pub struct GattServer {
    pub device_information: DeviceInformation,
}

impl<'values> GattServer<'values> {
    /// Start the Gatt server.
    ///
    /// # Panic
    ///
    /// This function will panic if the Gatt Server cannot be started.
    pub fn start(device_name: &'values str) -> Self {
        let gap_config = GapConfig::Peripheral(PeripheralConfig {
            name:       &device_name,
            appearance: &appearance::light_fixtures::LIGHT_CONTROLLER,
        });

        let gatt_server = match GattServer::new_with_config(gap_config) {
            Ok(gatt_server) => gatt_server,
            Err(err) => {
                defmt::panic!("error while starting the Gatt server: {}", err)
            }
        };

        gatt_server
    }

    pub async fn gatt_event_loop<'gatt_server>(
        &self,
        connection: &GattConnection<'values, 'gatt_server, DefaultPacketPool>,
    ) -> Result<(), trouble_host::Error> {
        let _disconnect_reason = loop {
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
