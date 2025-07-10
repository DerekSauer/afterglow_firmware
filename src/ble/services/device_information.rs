// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

use trouble_host::prelude::*;

const SERIAL_NUMBER_LEN: usize = 10;
const SERIAL_NUMBER: [u8; SERIAL_NUMBER_LEN] = *b"202507-001";

#[gatt_service(uuid = service::DEVICE_INFORMATION)]
pub struct DeviceInformation {
    #[characteristic(uuid = characteristic::SERIAL_NUMBER_STRING, read, value = SERIAL_NUMBER)]
    pub serial_number: [u8; SERIAL_NUMBER_LEN],
}
