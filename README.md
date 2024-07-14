# PN532 Rust Library

[![License](https://img.shields.io/crates/l/pn532.svg)](https://github.com/italicmew/pn532/blob/main/LICENSE)

## Introduction

`pn532` is a platform-agnostic driver for the PN532 NFC/RFID controller, written in Rust and based on the [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits. This library allows you to easily interface with the PN532 controller using any microcontroller or platform that supports Rust and `embedded-hal`.

## Features

- Platform-agnostic and portable across different hardware.
- Easy-to-use interface for sending commands to the PN532.
- Support for I2C/SPI communication.
- Extensible design to add more frame types and functionalities.

## Development goals
[ ] i2c interface working
[ ] spi interface working
[ ] esp32 simple example (get firmware version)
[ ] raspiberry pi pico simple example (get firmware version)
[ ] read mifare
[ ] ISO14443A tag
[ ] NTAG203 or NTAG213 tag reader/writer
[ ] NFC example
[ ] RFID example
[ ] HSU interface


## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
embedded-hal = "0.2"  # or the latest version available
pn532 = "0.1"  # Replace with the actual version

```

## Usage

TODO
