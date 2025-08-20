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
    pub fn start(device_name: &'values str) -> Result<Self, &'static str> {
        let gap_config = GapConfig::Peripheral(PeripheralConfig {
            name:       &device_name,
            appearance: &appearance::light_fixtures::LIGHT_CONTROLLER,
        });

        Ok(GattServer::new_with_config(gap_config)?)
    }

    /// Handle GATT events whenever a connection is made.
    pub async fn gatt_event_loop<'gatt_server>(
        &self,
        connection: &GattConnection<'values, 'gatt_server, DefaultPacketPool>,
    ) -> Result<(), trouble_host::Error> {
        loop {
            match connection.next().await {
                GattConnectionEvent::Disconnected { reason } => {
                    defmt::info!("[gatt] client disconnected, ATT code: {}", reason);
                    break;
                }
                GattConnectionEvent::Gatt { event } => match event.accept() {
                    Ok(reply) => reply.send().await,
                    Err(gatt_error) => {
                        defmt::warn!("[gatt] error processing a request: {:?}", gatt_error)
                    }
                },
                _ => {}
            }
        }

        Ok(())
    }
}
