# Alpakkabadge2024

Alpakkabadge2024 - Created originally for Disobey 2024
![IMG_5230](https://github.com/AlpakkaFarmi/Alpakkabadge2024/assets/1899518/a2a75f0f-662e-4a03-a8c5-fc214acd9281)

Schematic: https://github.com/AlpakkaFarmi/Alpakkabadge2024/blob/main/hw/alpakkabadge_schematic_v6.pdf

# Getting Started

## 1. Remove battery power

CR2032 is not powerful enough to feed the Pico and three RGB-LEDs, so it is wiser to power the badge via USB or the dedicated power input pads.

To ensure that the battery is not charged from the external power supply, it is essential to cut the jumper below the battery holder.

## 2. Remove three resistors
![IMG_5291](https://github.com/AlpakkaFarmi/Alpakkabadge2024/assets/1899518/c1cc8bd7-7634-43b4-92ab-43d06c6a29c4)

There are three resistors located inside the Pico footprint.
The red LEDs are connected to GND via three resistors. These resistors keep the LEDs on when the battery is inserted.

To allow Pico to control these red LEDs, remove these three resistors.

## 3. Solder Pico

Add solder to one corner of the Pico footprint and align Pico with the pads. This way you can still adjust board position by heating the soldered pad.

When Pico is aligned, solder all other pads.

## 4. Program Pico

Connect Pico to computer while holding the BOOTSEL-button.
You should see new mass memory device appearing.

Copy the latest release from this repo to Pico and the badge should reboot with blinking lights.

# Building Firmware

## Based on Minimal rp2040 template

Minimal template for using rust with rp2040.

### Development prerequisites

Install Rust https://www.rust-lang.org/

### How to use this template

First install [cargo-generate](https://crates.io/crates/cargo-generate)

    cargo install cargo-generate

Generate new project based on the template

    cargo generate tanelikaivola/rp2040_minimal_template

### Building and installing template

#### Setup for building the firmware

If you have fresh installation of rust, you need to add architecture as a target

    rustup target add thumbv6m-none-eabi

Then install [elf2uf2-rs](https://crates.io/crates/elf2uf2-rs) for automatic deploy of the build firmware to pico
    
    cargo install elf2uf2-rs

#### Build

Got to your project folder (the name you gave in `cargo generate`)

    cd [your project name]

Connect pico into computer in bootmode and run `run` to build and deploy of the firmware

    cargo run --release

