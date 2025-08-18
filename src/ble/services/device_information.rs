// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

use bt_hci::uuid::{BluetoothUuid16, characteristic, service};
use trouble_host::attribute::{AttributeTable, Characteristic, Service};

/// The Device Information Service exposes manufacturer and/or vendor
/// information about a device.
///
/// # Remarks
///
/// Some characteristics of the Device Information service are not relevant to
/// our device and are omitted.
pub struct DeviceInformation {
    /// The Manufacturer Name String characteristic shall represent the name of
    /// the manufacturer of the device.
    pub manufacturer_name: Characteristic<&'static str>,

    /// The Model Number String characteristic shall represent the model number
    /// that is assigned by the device vendor.
    pub model_number: Characteristic<&'static str>,

    /// The Serial Number String characteristic shall represent the serial
    /// number for a particular instance of the device.
    pub serial_number: Characteristic<&'static str>,

    /// The Hardware Revision String characteristic shall represent the hardware
    /// revision for the hardware within the device.
    pub hardware_revision: Characteristic<&'static str>,

    /// The Firmware Revision String characteristic shall represent the firmware
    /// revision for the firmware within the device.
    pub firmware_revision: Characteristic<&'static str>,

    handle: u16,
}

impl DeviceInformation {
    /// Each read only characteristic adds two attributes to the attribute
    /// table. The service itself also adds one attribute.
    pub const ATTRIBUTE_COUNT: usize = 5 * 2 + 1;
    /// BLE 16-bit UUID assigned to the Device Information service.
    pub const BLE_UUID16: BluetoothUuid16 = bt_hci::uuid::service::DEVICE_INFORMATION;
    /// Read only attributes do not require Client Characteristic Configuration
    /// Descriptors (CCCD).
    pub const CCCD_COUNT: usize = 0;

    pub fn new<MUTEX, const MAX_ATTRIBUTES: usize>(
        attributes_table: &mut AttributeTable<'_, MUTEX, MAX_ATTRIBUTES>,
    ) -> Self
    where
        MUTEX: embassy_sync::blocking_mutex::raw::RawMutex,
    {
        let mut service = attributes_table.add_service(Service::new(service::DEVICE_INFORMATION));

        let manufacturer_name = service
            .add_characteristic_ro(
                characteristic::MANUFACTURER_NAME_STRING,
                &crate::MANUFACTURER_NAME,
            )
            .build();

        let model_number = service
            .add_characteristic_ro(characteristic::MODEL_NUMBER_STRING, &crate::MODEL_NUMBER)
            .build();

        let serial_number = service
            .add_characteristic_ro(characteristic::SERIAL_NUMBER_STRING, &crate::SERIAL_NUMBER)
            .build();

        let hardware_revision = service
            .add_characteristic_ro(
                characteristic::HARDWARE_REVISION_STRING,
                &crate::HARDWARE_REVISION,
            )
            .build();

        let firmware_revision = service
            .add_characteristic_ro(
                characteristic::FIRMWARE_REVISION_STRING,
                &crate::FIRMWARE_REVISION,
            )
            .build();

        Self {
            handle: service.build(),
            manufacturer_name,
            model_number,
            serial_number,
            hardware_revision,
            firmware_revision,
        }
    }
}
