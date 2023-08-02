⚠️  This project is experimental and currently only supports the following devices
* Boox Note Air 2
* Boox Ultra Tab C

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
**NOTE:** Currently this works (hopefully)

## Wireless

As of android 11 you can use adb wirelessly by enabling wireless debugging and pairing your device.

## FAQ

### Support for other tablets/devices ?

Currently works with usb and wifi connections, basically once you have an adb connection regardless of how, this program should work

If you want your device to have support you can help ! even if you don't write the driver yourself opening an issue and being willing to gather device specifications.

### How does this work ?

This uses adb to fetch kernel input events via the builtin `getevent` tool on android - then it simply creates an appropriate virtual device via evdev and emits any events it reads from `getevent`.
