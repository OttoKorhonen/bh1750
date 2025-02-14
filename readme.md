
# BH1750 Ambient light sensor

This is a driver written in Rust programming language for BH1759 ambient light sensor. This driver was developed using an ESP32C3 micro controller. This driver should work on both ESP32 devices and Raspberry Pi Pico devices but this has not been tested yet.

## Datasheet

Datasheet for bh1750 can be found at link below:
> https://www.elechouse.com/elechouse/images/product/Digital%20light%20Sensor/bh1750fvi-e.pdf 

## Build instructions

### ESP32-based boards
- cargo build --release --features esp32
- cargo build --release --features esp32s2
- cargo build --release --features esp32s3
- cargo build --release --features esp32c3

### Raspberry Pi
- cargo build --release --features rp-pico  
- cargo build --release --features rpi 

## Supported platforms

| Platform           | Status      |  
|--------------------|------------|  
| ESP32C3           | ✅ Tested  |  
| ESP32 / ESP32S2 / ESP32S3 | ⚠ Not tested |  
| Raspberry Pi Pico | ⚠ Not tested |  
| Raspberry Pi 4/5  | ⚠ Not tested |
