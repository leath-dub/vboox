⚠️  This project is experimental and currently only supports the following devices
* Boox Note Air 2

# vboox

Virtual user space driver for boox devices

## Usage

### Pre-Requisites

If your are on x11 you need to install the xorg-evdev driver (`xf86-input-evdev` on arch).

If you are on wayland it should work out of the box !, if you have
issues open an issue.

You also need android platform tools for adb

**TODO:** add more details about this

Build the project
```
cargo build --release
```
and run it
```
sudo ./target/release/vboox
```
**NOTE:** Currently this works (hopefully) only for the `Boox Note Air 2`

## FAQ

### Support for other tablets/devices ?

Currently only supports Boox Note Air 2 over usb however I would absolutely would love to expand on this project feel free to open any issues or pr's.

If you want your device to have support you can help ! even if you don't write the driver yourself opening an issue and being willing to gather device specifications.

### How does this work ?

This uses adb to fetch kernel input events via the builtin `getevent` tool on android - then it simply creates an appropriate virtual device via evdev and emits any events it reads from `getevent`.
