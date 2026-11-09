#![no_main]
#![no_std]
use rmk::macros::rmk_peripheral;
#[cfg(feature = "pmw3610")]
mod pmw3610;
#[cfg(feature = "lpps")]
mod lpps;
#[cfg(feature = "iqs9151")]
mod iqs9151;
#[cfg(feature = "iqs9151")]
mod iqs9151_config;
#[cfg(feature = "iqs9151")]
mod iqs9151_tune;
#[cfg(not(any(feature = "pmw3610", feature = "lpps", feature = "iqs9151")))]
compile_error!("Select a Madula input module");
mod battery_audit;
mod status_led;
#[rmk_peripheral(id = 1)]
mod keyboard_peripheral {
    #[Override(usb)]
    fn init_usb() {
        #[cfg(feature = "iqs9151")]
        spawner.spawn(crate::iqs9151_tune::receive_split().unwrap());
        #[cfg(feature = "lpps")]
        {
            let mut config = embassy_nrf::spim::Config::default();
            config.frequency = embassy_nrf::spim::Frequency::M1;
            config.mode = embassy_nrf::spim::MODE_1;
            let spi = embassy_nrf::spim::Spim::new(
                p.TWISPI0, crate::lpps::Irqs, p.P1_15, p.P1_13, p.P1_12, config,
            );
            spawner.spawn(crate::lpps::run(spi).unwrap());
        }

        #[cfg(feature = "iqs9151")]
        {
            let mut config = embassy_nrf::twim::Config::default();
            config.frequency = embassy_nrf::twim::Frequency::K400;
            config.sda_pullup = true;
            config.scl_pullup = true;
            static TX: static_cell::StaticCell<[u8; 32]> = static_cell::StaticCell::new();
            let i2c = embassy_nrf::twim::Twim::new(
                p.TWISPI0, crate::iqs9151::Irqs, p.P1_14, p.P1_13, config,
                TX.init([0; 32]),
            );
            spawner.spawn(crate::iqs9151::run(i2c,
                embassy_nrf::gpio::Input::new(p.P1_12, embassy_nrf::gpio::Pull::Up),
            ).unwrap());
        }


        #[cfg(feature = "pmw3610")]
        {
            use embassy_nrf::gpio::{Output, Input, Flex, Level, OutputDrive, Pull};
            spawner.spawn(crate::pmw3610::run(
                Output::new(p.P1_13, Level::High, OutputDrive::Standard),
                Flex::new(p.P1_15),
                Output::new(p.P1_12, Level::High, OutputDrive::Standard),
                Input::new(p.P1_14, Pull::Up),
            ).unwrap());
        }
        #[cfg(feature = "battery-audit")]
        {
            let mut config = embassy_nrf::saadc::Config::default();
            config.oversample = embassy_nrf::saadc::Oversample::Over16x;
            let mut channel = embassy_nrf::saadc::ChannelConfig::single_ended(p.P0_02);
            channel.time = embassy_nrf::saadc::Time::_40US;
            let adc = embassy_nrf::saadc::Saadc::new(
                p.SAADC, crate::battery_audit::Irqs, config, [channel],
            );
            spawner.spawn(crate::battery_audit::run(adc).unwrap());
        }
        {
            use embassy_nrf::gpio::{Output, Level, OutputDrive};
            use embassy_nrf::pwm::{Config, Prescaler, SequenceLoad, SequencePwm};
            let mut config = Config::default();
            config.prescaler = Prescaler::Div1;
            config.max_duty = 20;
            config.sequence_load = SequenceLoad::Common;
            let pwm = SequencePwm::new_1ch(p.PWM0, p.P0_28, config).unwrap();
            spawner.spawn(crate::status_led::run(
                pwm,
                Output::new(p.P0_03, Level::High, OutputDrive::Standard),
                Output::new(p.P0_26, Level::High, OutputDrive::Standard),
                Output::new(p.P0_30, Level::High, OutputDrive::Standard),
                Output::new(p.P0_06, Level::High, OutputDrive::Standard),
            ).unwrap());
        }
        embassy_nrf::usb::Driver::new(
            p.USBD,
            Irqs,
            embassy_nrf::usb::vbus_detect::HardwareVbusDetect::new(Irqs),
        )
    }
}
