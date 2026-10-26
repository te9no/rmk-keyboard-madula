use super::{BATTERY_STATUS, BatteryStatus, BatteryStatusEvent, ChargeState, publish_event};

/// Publish a board-specific percentage without the built-in Li-ion ADC conversion.
/// Use only when the board owns battery measurement (no BatteryProcessor).
/// None means unavailable, never an empty battery. Charging state is unknown.
pub fn report_external_battery_level(level: Option<u8>) {
    let status = match level.filter(|level| *level <= 100) {
        Some(level) => BatteryStatus::Available {
            charge_state: ChargeState::Unknown,
            level: Some(level),
        },
        None => BatteryStatus::Unavailable,
    };
    let changed = BATTERY_STATUS.lock(|cache| {
        let changed = cache.get() != status;
        cache.set(status);
        changed
    });
    if changed {
        publish_event(BatteryStatusEvent::from(status));
    }
}

