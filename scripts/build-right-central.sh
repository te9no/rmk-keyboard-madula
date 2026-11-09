#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
for pair in "cornix-left:right-central-left:right-central" "cornix-right-central:right-central-tb-rynk:right-central,pmw3610,madula-rynk,dial-smooth" "madula-tb-peripheral:right-central-madula-tb:right-central,pmw3610,battery-audit"; do
  IFS=: read -r binary variant features <<< "$pair"
  cargo "+${RELEASE_TOOLCHAIN:-1.94.1}" build --release --locked --bin "$binary" --features "$features"
  mkdir -p dist/right-central
  elf="target/thumbv7em-none-eabihf/release/$binary"
  "${OBJCOPY:-arm-none-eabi-objcopy}" -O binary "$elf" "target/$variant.bin"
  python3 scripts/make-uf2.py "target/$variant.bin" "dist/right-central/$variant.uf2"
  cp "$elf" "dist/right-central/$variant.elf"
done
python3 scripts/check-right-central.py
