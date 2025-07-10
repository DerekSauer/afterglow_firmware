// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

use trouble_host::prelude::*;

use super::services::device_information::DeviceInformation;

#[gatt_server]
pub struct GattServer {
    pub device_information: DeviceInformation,
}
