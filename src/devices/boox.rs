use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    thread::sleep,
    time::{Duration, Instant},
};

use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AbsInfo, AbsoluteAxisType, AttributeSet, BusType, InputId, Key, UinputAbsSetup, InputEvent, EventType,
};

use regex::Regex;

use crate::devices::Device;

const TIMEOUT: u64 = 10;
const NOTE_AIR_2_MAX_X: i32 = 20966;
const NOTE_AIR_2_MAX_Y: i32 = 15725;
const NOTE_AIR_2_MAX_PRESSURE: i32 = 4095;
const NOTE_AIR_2_MAX_DISTANCE: i32 = 255;
const NOTE_AIR_2_MAX_TILT_X: i32 = 63;
const NOTE_AIR_2_MAX_TILT_Y: i32 = 63;
const NOTE_AIR_2_MIN_TILT_X: i32 = -63;
const NOTE_AIR_2_MIN_TILT_Y: i32 = -63;

macro_rules! abs_setup {
    ($axis_type:ident, $curr:expr, $min:expr, $max:expr, $fuzz:expr, $flat:expr, $res:expr) => {
        &UinputAbsSetup::new(
            AbsoluteAxisType::$axis_type,
            AbsInfo::new($curr, $min, $max, $fuzz, $flat, $res),
        )
    };
}

macro_rules! keys {
    () => (
        &AttributeSet::from_iter([])
    );
    ($($x:expr),+ $(,)?) => (
        &AttributeSet::from_iter([$($x),+])
    );
}

pub struct NoteAir2 {
    dev: VirtualDevice,
    udev_path: String,
    connected: bool,
}

impl Device for NoteAir2 {
    fn new() -> Option<Self> {
        return Some(Self {
            dev: VirtualDeviceBuilder::new()
                .unwrap()
                .name("Boox Virtual Tablet")
                .input_id(InputId::new(BusType::BUS_USB, 1, 1, 1))
                .with_absolute_axis(abs_setup!(ABS_X, 0, 0, NOTE_AIR_2_MAX_X, 0, 0, NOTE_AIR_2_MAX_X))
                .unwrap()
                .with_absolute_axis(abs_setup!(ABS_Y, 0, 0, NOTE_AIR_2_MAX_Y, 0, 0, NOTE_AIR_2_MAX_Y))
                .unwrap()
                .with_absolute_axis(abs_setup!(
                    ABS_PRESSURE,
                    0,
                    0,
                    NOTE_AIR_2_MAX_PRESSURE,
                    0,
                    0,
                    0
                ))
                .unwrap()
                .with_absolute_axis(abs_setup!(
                    ABS_DISTANCE,
                    0,
                    0,
                    NOTE_AIR_2_MAX_DISTANCE,
                    0,
                    0,
                    400
                ))
                .unwrap()
                .with_absolute_axis(abs_setup!(
                    ABS_TILT_X,
                    0,
                    NOTE_AIR_2_MIN_TILT_X,
                    NOTE_AIR_2_MAX_TILT_X,
                    0,
                    0,
                    0
                ))
                .unwrap()
                .with_absolute_axis(abs_setup!(
                    ABS_TILT_Y,
                    0,
                    NOTE_AIR_2_MIN_TILT_Y,
                    NOTE_AIR_2_MAX_TILT_Y,
                    0,
                    0,
                    0
                ))
                .unwrap()
                .with_keys(keys![
                    Key::BTN_TOOL_RUBBER,
                    Key::BTN_TOOL_BRUSH,
                    Key::BTN_TOUCH,
                    Key::BTN_STYLUS,
                    Key::BTN_STYLUS2,
                ])
                .unwrap()
                .build()
                .unwrap(),
            udev_path: Self::get_udev_path(),
            connected: false,
        });
    }

    fn try_connect(&mut self) -> Result<(), &'static str> {
        _ = Command::new("adb").args(["reconnect", "offline"]).output();

        println!("--- Press Accept to authorize usb debugging ---");

        let mut binding = Command::new("adb");
        let test_command = binding.args(["shell", "-T", ":"]); // do nothing successfully ':'

        let timeout = Duration::from_secs(TIMEOUT);
        let start = Instant::now();
        while !test_command.output().unwrap().status.success() || start.elapsed() >= timeout {
            sleep(Duration::from_secs(1));
        }

        if !(start.elapsed() >= timeout) {
            // success
            println!("+++ Validated connection +++");
            self.connected = true;
            return Ok(());
        }
        return Err("Failed to connect");
    }

    fn fetch_events(&mut self) {
        let proc = Command::new("adb")
            .args(["shell", "-T", "getevent", &self.udev_path])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Error reading events via getevent");

        let mut raw = String::with_capacity(16);
        if let Some(stdout) = proc.stdout {
            let rdr = BufReader::new(stdout);
            let mut event_buf = Vec::new();
            rdr.lines().flatten().for_each(|ev| {
                raw.extend(ev.split_whitespace().flat_map(|c| c.chars()));
                let typ = EventType(u16::from_str_radix(raw.get(..=3).unwrap(), 16).unwrap());
                let cod = u16::from_str_radix(raw.get(4..=7).unwrap(), 16).unwrap();
                let val = i64::from_str_radix(raw.get(8..).unwrap(), 16).unwrap();
                if typ == EventType::SYNCHRONIZATION {
                    _ = self.dev.emit(event_buf.as_slice()).unwrap();
                    event_buf.clear();
                } else {
                    event_buf.push(InputEvent::new_now(typ, cod, val as i32));
                }
                raw.clear()
            });
        } else {
            eprintln!("Failed to read stdout from getevent over adb");
        }
    }
}

impl NoteAir2 {
    fn get_udev_path() -> String {
        let output = Command::new("adb").args(["shell", "-T", "getevent", "-si"]).output().unwrap().stdout;
        let re = Regex::new(r#"add device [0-9]+: (.*)\n\s*name:\s*"onyx_emp_Wacom I2C Digitizer""#).unwrap();

        let binding = String::from_utf8(output).unwrap();
        let cap = re.captures(&binding).unwrap().get(1).unwrap();

        return cap.as_str().to_string();
    }
}

pub type UltraTabC = NoteAir2;
