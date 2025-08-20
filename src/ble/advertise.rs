// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

use trouble_host::prelude::*;

use super::services::device_information::DeviceInformation;

pub async fn advertise<'values, 'server, C: Controller>(
    device_name: &'values str,
    peripheral_role: &mut Peripheral<'values, C, DefaultPacketPool>,
    gatt_server: &'server super::gatt_server::GattServer<'values>,
) -> Result<GattConnection<'values, 'server, DefaultPacketPool>, BleHostError<C::Error>> {
    let mut advertise_data = [0; 31];

    AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ServiceUuids16(&[DeviceInformation::BLE_UUID16.to_le_bytes()]),
            AdStructure::CompleteLocalName(device_name.as_bytes()),
        ],
        &mut advertise_data[..],
    )?;

    let advertiser = peripheral_role
        .advertise(
            &AdvertisementParameters::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data:  &advertise_data[..],
                scan_data: &[],
            },
        )
        .await?;

    let connection = advertiser
        .accept()
        .await?
        .with_attribute_server(gatt_server)?;

    Ok(connection)
}
