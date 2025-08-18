// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Board support for Breadstick Innovation's Nougat C3-Mini LED
//! controller: https://shop.breadstick.ca/products/nougat-c3-mini

use bt_hci::controller::ExternalController;
use esp_hal::clock::CpuClock;
use esp_hal::config::{WatchdogConfig, WatchdogStatus};
use esp_hal::efuse::Efuse;
use esp_hal::gpio::{Input, InputConfig, Pull};
use esp_hal::rmt::Rmt;
use esp_hal::rng::Trng;
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{Blocking, Config, peripherals};
use esp_wifi::ble::controller::{BleConnector, BleConnectorError};
use static_cell::StaticCell;

pub type BleControllerImpl = bt_hci::controller::ExternalController<BleConnector<'static>, 20>;

pub type BleHostErrorImpl = BleConnectorError;

// This board has a single output for LED light strips.
// The user must choose one LED type or the other.
#[cfg(all(feature = "clockless_leds", feature = "clocked_leds"))]
compile_error!("feature `clockless_leds` and `clocked_leds` cannot be enabled a the same time");

/// Breadstick Innovation's Nougat C3-Mini LED control board.
pub struct Board {
    /// BLE controller.
    pub ble_controller: BleControllerImpl,

    /// Push button connected to GPIO9.
    button: esp_hal::gpio::Input<'static>,

    /// GPIO pin connected to the LED strip's clock line.
    #[cfg(feature = "clocked_leds")]
    led_clock_pin: peripherals::GPIO1<'board>,

    /// GPIO pin connected to the LED strip's data line.
    led_data_pin: peripherals::GPIO0<'static>,

    /// Remote control transceiver.
    /// This peripheral provides hardware accelerated transmission of timing
    /// sensitive packets to a GPIO pin.
    #[cfg(feature = "clockless_leds")]
    remote_control: esp_hal::rmt::Rmt<'static, Blocking>,

    /// True random number generator.
    pub rng: Trng<'static>,
}

impl Board {
    /// Begin constructing a new interface to our Nougat C3-Mini. Processor,
    /// clocks, and peripherals are intialized.
    ///
    /// # Panic
    ///
    /// Some peripheral drivers have potentionally fallible initialization
    /// processes but they are required for the correct functioning of the
    /// device. This function will panic if these peripherals cannot be
    /// initialized.
    pub fn init() -> Board {
        // Watchdog timers will not be needed for this application.
        let peripherals = esp_hal::init(
            Config::default()
                .with_cpu_clock(CpuClock::max())
                .with_watchdog(
                    WatchdogConfig::default()
                        .with_swd(false)
                        .with_rwdt(WatchdogStatus::Disabled)
                        .with_timg0(WatchdogStatus::Disabled)
                        .with_timg1(WatchdogStatus::Disabled),
                ),
        );

        // The WIFI controller requires a heap for its own allocations.
        // The bootloader reserves 64kb of RAM that we can reuse once the bootloader has
        // jumped to our code. Our own code will not touch this heap.
        // SAFETY: esp_hal's linker script for the ESP32C3 defines a 64kb memory segment
        // called `dram2_uninit` which comprises the memory region used by the
        // bootloader.
        esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);

        let embassy_timer = SystemTimer::new(peripherals.SYSTIMER);
        esp_hal_embassy::init(embassy_timer.alarm0);

        let pull_up_input = InputConfig::default().with_pull(Pull::Up);
        let button = Input::new(peripherals.GPIO9, pull_up_input);

        let led_data_pin = peripherals.GPIO0;

        #[cfg(feature = "clocked_leds")]
        let led_clock_pin = peripherals.GPIO1;

        #[cfg(feature = "clockless_leds")]
        let remote_control = {
            let rmt_rate = Rate::from_mhz(80);
            Rmt::new(peripherals.RMT, rmt_rate)
                .map_err(|err| defmt::panic!("failed to initialize the RMT peripheral: {}", err))
                .unwrap()
        };

        // The true random number generator uses one of the ADC peripherals as a source
        // of thermal noise to seed the RNG peripheral.
        let rng = Trng::new(peripherals.RNG, peripherals.ADC1);

        let ble_controller = {
            let timer_group = TimerGroup::new(peripherals.TIMG0);

            let wifi_controller = {
                static WIFI_CONTROLLER: StaticCell<esp_wifi::EspWifiController<'static>> =
                    StaticCell::new();
                WIFI_CONTROLLER.init_with(|| {
                    match esp_wifi::init(timer_group.timer0, rng.rng.clone()) {
                        Ok(wifi_controller) => wifi_controller,
                        Err(err) => defmt::panic!("failed to initialize wifi controller: {}", err),
                    }
                })
            };

            let hci_transport =
                esp_wifi::ble::controller::BleConnector::new(wifi_controller, peripherals.BT);

            ExternalController::new(hci_transport)
        };

        Board {
            ble_controller,
            button,
            #[cfg(feature = "clocked_leds")]
            led_clock_pin,
            led_data_pin,
            #[cfg(feature = "clockless_leds")]
            remote_control,
            rng,
        }
    }

    /// Retrieve this board's MAC address.
    ///
    /// The manufacturer of the board's MCU has conveniently written a unique
    /// MAC address to ROM which can be used as the device's BLE public address.
    pub fn get_mac_address(&self) -> [u8; 6] {
        Efuse::mac_address()
    }
}
