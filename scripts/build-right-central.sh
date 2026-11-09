#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
for pair in "cornix-left:right-central-left:right-central" \
  "cornix-right-central:right-central-tb-rynk:right-central,pmw3610,madula-rynk,dial-smooth" \
  "madula-peripheral:right-central-madula-tb:right-central,pmw3610,battery-audit" \
  "cornix-right-central:right-central-lpps-rynk:right-central,lpps,madula-rynk,dial-smooth" \
  "madula-peripheral:right-central-madula-lpps:right-central,lpps,battery-audit" \
  "cornix-right-central:right-central-iqs-rynk:right-central,iqs9151,madula-rynk,dial-smooth" \
  "madula-peripheral:right-central-madula-iqs:right-central,iqs9151,battery-audit"; do
  IFS=: read -r binary variant features <<< "$pair"
  cargo "+${RELEASE_TOOLCHAIN:-1.94.1}" build --release --locked --bin "$binary" --features "$features"
  mkdir -p dist/right-central
  elf="target/thumbv7em-none-eabihf/release/$binary"
  "${OBJCOPY:-arm-none-eabi-objcopy}" -O binary "$elf" "target/$variant.bin"
  python3 scripts/make-uf2.py "target/$variant.bin" "dist/right-central/$variant.uf2"
  cp "$elf" "dist/right-central/$variant.elf"
done
python3 scripts/check-right-central.py
