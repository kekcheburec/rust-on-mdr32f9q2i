#!/usr/bin/env bash

rm app.hex
cargo objcopy --release -- -O ihex app.hex
./tools/mdr32tool -s 115200 -d /dev/ttyUSB0 -b ./tools/mdr32_uart_bootloader.hex -f ./app.hex
