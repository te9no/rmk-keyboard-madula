#![no_main]
#![no_std]
use rmk::macros::rmk_central;
mod radial_controller;
mod right_central_led;
#[cfg(feature = "iqs9151")]
mod iqs9151_tune;
#[cfg(not(any(feature = "pmw3610", feature = "lpps", feature = "iqs9151")))]
compile_error!("Select a Madula input module");
#[rmk_central]
mod keyboard_central {
    #[Override(entry)]
    fn run() {
        #[cfg(feature = "iqs9151")]
        crate::iqs9151_tune::send_split();
        let mut pointer = rmk::input_device::pointing::PointingProcessor::new(
            &keymap,
            rmk::input_device::pointing::PointingProcessorConfig {
                device_id: 0,
                invert_x: !cfg!(feature = "iqs9151"),
                invert_y: !cfg!(feature = "iqs9151"),
                swap_xy: cfg!(feature = "pmw3610"),
            },
        );
        let mut dial = crate::radial_controller::RadialController;
        let mut usb_transport = rmk::usb::UsbTransport::new(driver, rmk_config.device_config);
        #[cfg(any(feature = "madula-vial", feature = "madula-rynk"))]
        let mut usb_transport = usb_transport.with_host_service(&host_service);
        let mut ble_transport = rmk::ble::BleTransport::new(
            ble_controller, ble_addr, rmk_config,
            [
                rmk::split::PeripheralMatrixConfig { rows: 4, cols: 7, row_offset: 0, col_offset: 0 },
                rmk::split::PeripheralMatrixConfig { rows: 1, cols: 14, row_offset: 4, col_offset: 0 },
            ],
        );
        #[cfg(any(feature = "madula-vial", feature = "madula-rynk"))]
        let mut ble_transport = ble_transport.with_host_service(&host_service);
        let mut wpm = rmk::processor::builtin::wpm::WpmProcessor::new();
        rmk::run_all!(matrix, storage, keyboard, usb_transport, ble_transport,
                      wpm, watchdog_runner, pointer, dial, encoder_0).await;
    }
    #[Override(usb)]
    fn init_usb() {
        use embassy_nrf::gpio::{Level, Output, OutputDrive};
        use embassy_nrf::pwm::{Config, Prescaler, SequenceLoad, SequencePwm};
        let mut config = Config::default();
        config.prescaler = Prescaler::Div1;
        config.max_duty = 20;
        config.sequence_load = SequenceLoad::Common;
        let pwm = SequencePwm::new_1ch(p.PWM0, p.P0_13, config).unwrap();
        spawner.spawn(crate::right_central_led::run(
            pwm, Output::new(p.P0_24, Level::Low, OutputDrive::Standard),
        ).unwrap());
        embassy_nrf::usb::Driver::new(
            p.USBD, Irqs,
            embassy_nrf::usb::vbus_detect::HardwareVbusDetect::new(Irqs),
        )
    }
}
