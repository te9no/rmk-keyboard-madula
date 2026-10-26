// HID layout adapted from sekigon-gonnoc/zmk-driver-hires-dial
// src/radial_controller/hid.c (353a21964a2f6df1de128ca6e71ec63518729ab7).
// Copyright (c) 2026 sekigon-gonnoc
// SPDX-License-Identifier: MIT
// Separate USB interface: no Report ID; button bit + signed 15-bit 0.1 degree delta.
#[cfg(test)]
pub const DESCRIPTOR: &[u8] = &[
    0x05, 0x01, 0x09, 0x0e, 0xa1, 0x01,
    0x05, 0x0d, 0x09, 0x21, 0xa1, 0x00,
    0x05, 0x09, 0x09, 0x01, 0x95, 0x01, 0x75, 0x01,
    0x15, 0x00, 0x25, 0x01, 0x81, 0x02,
    0x05, 0x01, 0x09, 0x37, 0x95, 0x01, 0x75, 0x0f,
    0x55, 0x0f, 0x65, 0x14, 0x36, 0xf0, 0xf1,
    0x46, 0x10, 0x0e, 0x16, 0xf0, 0xf1, 0x26, 0x10, 0x0e,
    0x81, 0x06, 0xc0, 0xc0,
];

pub fn pack(button: bool, angle: i16) -> [u8; 2] {
    let angle = angle.clamp(-3600, 3600);
    (((angle as u16 & 0x7fff) << 1) | u16::from(button)).to_le_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn button_and_signed_angle() {
        assert_eq!(pack(false, 0), [0, 0]);
        assert_eq!(pack(true, 0), [1, 0]);
        for angle in [-3600, -180, 0, 180, 3600] {
            for button in [false, true] {
                let v = u16::from_le_bytes(pack(button, angle));
                assert_eq!(v & 1, u16::from(button));
                assert_eq!((v as i16) >> 1, angle);
            }
        }
        assert_eq!(pack(false, i16::MAX), pack(false, 3600));
        assert_eq!(pack(false, i16::MIN), pack(false, -3600));
    }
    #[test]
    fn radial_collection_and_two_byte_report() {
        assert_eq!(&DESCRIPTOR[..6], &[5, 1, 9, 14, 161, 1]);
        assert!(DESCRIPTOR.windows(2).any(|v| v == [0x75, 1]));
        assert!(DESCRIPTOR.windows(2).any(|v| v == [0x75, 15]));
    }
}


// Experimental cadence: preserve the angle, direction and button in every report.
// 6 degrees x 3 reports = the original 18-degree encoder detent.
#[cfg(any(test, feature = "dial-smooth"))]
pub fn split_rotation(angle: i16) -> [i16; 3] {
    let first = angle / 3;
    [first, first, angle - first * 2]
}

#[cfg(test)]
mod pacing_tests {
    use super::*;
    #[test]
    fn splitting_conserves_signed_rotation_and_button() {
        for angle in -3600i16..=3600 {
            let steps = split_rotation(angle);
            assert_eq!(steps.iter().map(|&v| i32::from(v)).sum::<i32>(), i32::from(angle));
            for button in [false, true] {
                for step in steps {
                    let word = u16::from_le_bytes(pack(button, step));
                    assert_eq!(word & 1, u16::from(button));
                    assert_eq!((word as i16) >> 1, step);
                }
            }
        }
        assert_eq!(split_rotation(180), [60, 60, 60]);
        assert_eq!(split_rotation(-180), [-60, -60, -60]);
    }
}
