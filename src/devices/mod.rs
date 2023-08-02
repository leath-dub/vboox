use self::boox::{NoteAir2, UltraTabC};

pub mod boox;

pub trait Device {
    fn new() -> Option<Self> where Self: Sized;
    fn try_connect(&mut self) -> Result<(), &'static str>;
    fn fetch_events(&mut self);
}

pub fn device_by_str(s: &str) -> Option<impl Device> {
    return match s {
        "NoteAir2" => NoteAir2::new(),
        "UltraTabC" => UltraTabC::new(),
        _ => None,
    };
}

pub const DEVICE_LIST: [&str; 2] = [
    "NoteAir2",
    "UltraTabC",
];
