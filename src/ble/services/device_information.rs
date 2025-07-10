// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

use trouble_host::prelude::*;

const CARGO_PKG_VERSION_BYTES: &[u8] = env!("CARGO_PKG_VERSION").as_bytes();
const FIRMWARE_REVISION_LEN: usize = CARGO_PKG_VERSION_BYTES.len();
const FIRMWARE_REVISION: [u8; FIRMWARE_REVISION_LEN] = const {
    let mut owned_bytes: [u8; FIRMWARE_REVISION_LEN] = [0; FIRMWARE_REVISION_LEN];
    let mut index = 0;

    while index < FIRMWARE_REVISION_LEN {
        owned_bytes[index] = CARGO_PKG_VERSION_BYTES[index];
        index += 1;
    }

    owned_bytes
};

const MANUFACTURER_NAME_LEN: usize = 13;
const MANUFACTURER_NAME: [u8; MANUFACTURER_NAME_LEN] = *b"Sauerstoff.ca";

const MODEL_NUMBER_LEN: usize = 12;
const MODEL_NUMBER: [u8; MODEL_NUMBER_LEN] = *b"Afterglow-01";

// TODO: Add serial number stamping automation to Build script.
const SERIAL_NUMBER_LEN: usize = 10;
const SERIAL_NUMBER: [u8; SERIAL_NUMBER_LEN] = *b"202507-001";

#[gatt_service(uuid = service::DEVICE_INFORMATION)]
pub struct DeviceInformation {
    #[characteristic(uuid = characteristic::FIRMWARE_REVISION_STRING, read, value =
    FIRMWARE_REVISION)]
    pub firmware_revision: [u8; FIRMWARE_REVISION_LEN],

    #[characteristic(uuid = characteristic::MANUFACTURER_NAME_STRING, read, value =
    MANUFACTURER_NAME)]
    pub manufacturer_name: [u8; MANUFACTURER_NAME_LEN],

    #[characteristic(uuid = characteristic::MODEL_NUMBER_STRING, read, value = MODEL_NUMBER)]
    pub model_number:  [u8; MODEL_NUMBER_LEN],
    #[characteristic(uuid = characteristic::SERIAL_NUMBER_STRING, read, value = SERIAL_NUMBER)]
    pub serial_number: [u8; SERIAL_NUMBER_LEN],
}
