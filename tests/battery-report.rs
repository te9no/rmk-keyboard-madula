// Compile with rustc --test; exercise the production bridge with a host cache/event sink.
use std::{cell::Cell, sync::Mutex};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChargeState { Unknown }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BatteryStatus {
    Unavailable,
    Available { charge_state: ChargeState, level: Option<u8> },
}
struct Cache(Mutex<Cell<BatteryStatus>>);
impl Cache {
    fn lock<R>(&self, f: impl FnOnce(&Cell<BatteryStatus>) -> R) -> R {
        f(&self.0.lock().unwrap())
    }
}
static BATTERY_STATUS: Cache = Cache(Mutex::new(Cell::new(BatteryStatus::Unavailable)));
type BatteryStatusEvent = BatteryStatus;
static EVENTS: Mutex<Vec<BatteryStatus>> = Mutex::new(Vec::new());
fn publish_event(event: BatteryStatusEvent) { EVENTS.lock().unwrap().push(event); }
#[path = "../vendor/rmk/src/input_device/external_battery.rs"]
mod external_battery;
#[path = "../src/battery_voltage.rs"]
mod voltage;

#[test]
fn measurement_to_host_cache_and_change_notifications() {
    use external_battery::report_external_battery_level as report;
    report(None);
    assert!(EVENTS.lock().unwrap().is_empty());
    let (_, mv) = voltage::millivolts(484).unwrap();
    report(voltage::estimated_percent(mv));
    let expected = BatteryStatus::Available { charge_state: ChargeState::Unknown, level: Some(83) };
    assert_eq!(BATTERY_STATUS.lock(|c| c.get()), expected);
    assert_eq!(*EVENTS.lock().unwrap(), vec![expected]);
    for _ in 0..100 { report(Some(83)); }
    assert_eq!(EVENTS.lock().unwrap().len(), 1);
    // A real empty battery remains representable; missing/invalid data is distinct.
    report(Some(0));
    assert_eq!(BATTERY_STATUS.lock(|c| c.get()),
        BatteryStatus::Available { charge_state: ChargeState::Unknown, level: Some(0) });
    report(None);
    report(Some(101));
    assert_eq!(BATTERY_STATUS.lock(|c| c.get()), BatteryStatus::Unavailable);
    assert_eq!(EVENTS.lock().unwrap().len(), 3);
    report(Some(83));
    assert_eq!(BATTERY_STATUS.lock(|c| c.get()), expected);
    assert_eq!(EVENTS.lock().unwrap().len(), 4);
}
