// SPDX-FileCopyrightText: 2025 Derek Sauer
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Board support for Breadstick Innovation's Nougat C3-Mini LED
//! controller: https://shop.breadstick.ca/products/nougat-c3-mini

use esp_hal::clock::CpuClock;
use esp_hal::config::{WatchdogConfig, WatchdogStatus};
use esp_hal::rmt::Rmt;
use esp_hal::rng::{Rng, Trng};
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{Blocking, Config, peripherals};

// This board has a single output for LED light strips.
// The user must choose one LED type or the other.
#[cfg(all(feature = "clockless_leds", feature = "clocked_leds"))]
compile_error!("feature `clockless_leds` and `clocked_leds` cannot be enabled a the same time");

/// Breadstick Innovation's Nougat C3-Mini LED control board.
pub struct Board<'board> {
    /// GPIO pin connected to the "BOOT" button.
    pub boot_button: peripherals::GPIO9<'static>,

    /// GPIO pin connected to the LED strip's data line.
    pub led_data_pin: peripherals::GPIO0<'static>,

    /// GPIO pin connected to the LED strip's clock line.
    #[cfg(feature = "clocked_leds")]
    pub led_clock_pin: peripherals::GPIO1<'static>,

    /// Remote control transceiver.
    /// This peripheral provides hardware accelerated transmission of timing
    /// sensitive packets to a GPIO pin.
    #[cfg(feature = "clockless_leds")]
    pub remote_control: esp_hal::rmt::Rmt<'board, Blocking>,

    /// Timer dedicated to the WIFI module.
    wifi_timer: TimerGroup<'board, peripherals::TIMG0<'static>>,

    /// Random number generator dedicate to the WIFI module and BLE host.
    rng: Trng<'board>,

    /// The WIFI module will take control of radio clocks when enabled.
    radio_clock: peripherals::RADIO_CLK<'static>,
}

impl Board<'_> {
    /// Begin constructing a new interface to our Nougat C3-Mini. Processor,
    /// clocks, and peripherals are intialized.
    pub fn new<'board>(clock_speed: ClockSpeed) -> Board<'board> {
        // The WIFI controller requires a heap for its own allocations.
        // The bootloader reserves 64kb of RAM that we can reuse once the bootloader has
        // jumped to our code. Our own code will not touch this heap.
        // SAFETY: esp_hal's linker script for the ESP32C3 defines a 64kb memory segment
        // called `dram2_uninit` which comprises the memory region used by the
        // bootloader.
        esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);

        // Watchdog timers will not be needed for this application.
        let peripherals = esp_hal::init(
            Config::default()
                .with_cpu_clock(clock_speed.into())
                .with_watchdog(
                    WatchdogConfig::default()
                        .with_swd(false)
                        .with_rwdt(WatchdogStatus::Disabled)
                        .with_timg0(WatchdogStatus::Disabled)
                        .with_timg1(WatchdogStatus::Disabled),
                ),
        );

        defmt::info!("[board] mcu and peripherals initialized.");

        let boot_button = peripherals.GPIO9;
        let led_data_pin = peripherals.GPIO0;

        #[cfg(feature = "clocked_leds")]
        let led_clock_pin = peripherals.GPIO1;

        #[cfg(feature = "clockless_leds")]
        let remote_control = {
            let clock_rate = clock_speed.half_clock_mhz();
            let rmt_rate = Rate::from_mhz(clock_rate);
            match Rmt::new(peripherals.RMT, rmt_rate) {
                Ok(rmt) => {
                    defmt::info!(
                        "[board] remote control peripheral configured at {}Mhz clock rate",
                        clock_rate
                    );
                    rmt
                }
                Err(error) => {
                    defmt::panic!("[board] could not initialize RMT peripheral: {}", error)
                }
            }
        };

        // Peripherals reserved for the wifi/ble stack.
        let wifi_timer = TimerGroup::new(peripherals.TIMG0);
        let rng = Trng::new(peripherals.RNG, peripherals.ADC1);
        let radio_clock = peripherals.RADIO_CLK;

        // Embassy can use the second timer group.
        let embassy_timer = TimerGroup::new(peripherals.TIMG1);
        esp_hal_embassy::init(embassy_timer.timer0);

        Board {
            boot_button,
            led_data_pin,
            #[cfg(feature = "clocked_leds")]
            led_clock_pin,
            #[cfg(feature = "clockless_leds")]
            remote_control,
            wifi_timer,
            rng,
            radio_clock,
        }
    }

    /// Retrieve a clone of the random number generator.
    pub fn clone_rng(&self) -> Rng {
        self.rng.rng.clone()
    }
}

/// The ESP32C3 processor on this board offers two clock speeds.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ClockSpeed {
    /// 80Mhz.
    #[default]
    Low80Mhz,

    /// 160Mhz.
    High160Mhz,
}

impl ClockSpeed {
    /// Get half the clock speed in Mhz.
    pub fn half_clock_mhz(&self) -> u32 {
        match &self {
            ClockSpeed::Low80Mhz => 40,
            ClockSpeed::High160Mhz => 80,
        }
    }
}

impl From<ClockSpeed> for CpuClock {
    fn from(value: ClockSpeed) -> Self {
        match value {
            ClockSpeed::Low80Mhz => CpuClock::_80MHz,
            ClockSpeed::High160Mhz => CpuClock::_160MHz,
        }
    }
}
