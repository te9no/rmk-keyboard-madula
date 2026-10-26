//! USB/BLE Radial Controller actions routed by RMK's active transport.
use rmk::core_traits::Runnable;
use rmk::event::{ActionEvent, EventSubscriber, SubscribableEvent};
use rmk::types::action::Action;
#[path = "radial_report.rs"]
mod report;
pub struct RadialController;
impl Runnable for RadialController {
    async fn run(&mut self) -> ! {
        let mut actions = ActionEvent::subscriber();
        let mut button = false;
        loop {
            let event = actions.next_event().await;
            let angle = match event.action {
                Action::User(20) if event.keyboard_event.pressed => 180,
                Action::User(21) if event.keyboard_event.pressed => -180,
                Action::User(22) => { button = event.keyboard_event.pressed; 0 }
                #[cfg(feature = "iqs9151")]
                Action::User(id) if event.keyboard_event.pressed => {
                    if let Some(pulses) = crate::iqs9151_tune::adjust(id) { crate::status_led::pulse(pulses); }
                    continue;
                }
                _ => continue,
            };
            #[cfg(feature = "dial-smooth")]
            if angle != 0 {
                // Experimental: keep 18 degrees per detent. Do not fabricate
                // extra rotation to exceed Windows' tool-dependent threshold.
                for (index, step) in report::split_rotation(angle).into_iter().enumerate() {
                    if index != 0 {
                        embassy_time::Timer::after_millis(8).await;
                    }
                    rmk::channel::send_hid_report(
                        rmk::hid::Report::RadialControllerReport(report::pack(button, step))
                    ).await;
                }
                continue;
            }
            rmk::channel::send_hid_report(
                rmk::hid::Report::RadialControllerReport(report::pack(button, angle))
            ).await;
        }
    }
}
