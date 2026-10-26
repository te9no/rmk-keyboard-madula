# Rynk physical layout inspection

From the repository root:

    cargo +stable run --locked --manifest-path tests/layout-preview/Cargo.toml --target x86_64-unknown-linux-gnu -- keyboard.toml

Uses RMK's actual layout parser and compressed wire format decoder, checks a round trip,
and prints the decoded geometry (64 keys, 2 encoders). Rect coordinates are centers in key units.
The Madula display overlay in keyboard.toml follows the ZMK physical layout, including rotated thumb-key centers. It does not reorder
layout.map key coordinates or change keymap layers. Virtual input keys remain editable below the keyboard.
