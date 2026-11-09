#![no_main]
#![no_std]
use rmk::macros::rmk_peripheral;
mod pmw3610;
mod battery_audit;
mod status_led;
#[rmk_peripheral(id = 1)]
mod keyboard_peripheral {
    #[Override(usb)]
    fn init_usb() {

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
