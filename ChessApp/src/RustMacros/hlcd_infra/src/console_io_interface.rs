use enum_iterator::IntoEnumIterator;

#[derive(Debug, IntoEnumIterator, PartialEq, Clone, Copy)]
pub enum ConsoleColor {
    Black        = 0x0,
    Blue         = 0x1,
    Green        = 0x2,
    Cyan         = 0x3,
    Red          = 0x4,
    Magenta      = 0x5,
    Brown        = 0x6,
    Gray         = 0x7,
    LightGray    = 0x8,
    LightBlue    = 0x9,
    LightGreen   = 0xA,
    LightCyan    = 0xB,
    LightRed     = 0xC,
    LightMagenta = 0xD,
    Yellow       = 0xE,
    White        = 0xF
}

hlcd::define! {
    interface ConsoleIO {
        fn set_foreground(&mut self, color: ConsoleColor);
        fn set_background(&mut self, color: ConsoleColor);
        fn write(&self, s: &str);
        fn read_line(&self) -> String;
    }
}

