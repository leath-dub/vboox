#!/bin/sh

set -e

: ${busybox:=busybox}
: ${adb:=adb}
: ${check_period_in_seconds:=10}
: ${vboox_bin_path:=$HOME/.local/bin/vboox}
: ${vboox_bin:=vboox}

set_device_name() {
    if [ -z "$1" ]; then
        echo "usage: vbooxd.sh <device name>" > /dev/stderr
        exit 1
    fi

    case "$1" in
        NoteAir2|UltraTabC);;
        *) echo "invalid device" > /dev/stderr && exit 1;;
    esac

    device_name="$1"
}

boox_device_connected() {
    [ -n "$($adb devices -l | grep BOOX)" ]
}

set_device_name $@

vboox_running() {
    [ -n "$($busybox pgrep -x "$vboox_bin")" ]
}

while :; do
    if ! vboox_running && boox_device_connected; then
        $busybox setsid sh -c "exec $vboox_bin_path $device_name 2>&1 > /dev/null"
    elif vboox_running && ! boox_device_connected; then
        for vboox in $(pidof $vboox_bin $device_name); do
            kill $vboox
        done
    fi
    sleep $check_period_in_seconds
done
