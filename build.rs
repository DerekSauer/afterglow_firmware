// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

fn main() {
    println!("cargo::rustc-link-arg=-Tlinkall.x");
    println!("cargo::rustc-link-arg=-Tdefmt.x");
}
