#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color { //custom color enum
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode { //contains the full background and foreground color bytes
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar { //struct to hold character definitions
    asciiCharacter: u8,
    colorCode: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer { //define buffer for the character
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer { //define the writer to write to the last line and shift up when full
    columnPosition: usize,
    colorCode: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer { //implement writing functions for Writer
    pub fn writeByte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newLine(),
            byte => {
                if self.columnPosition >= BUFFER_WIDTH {
                    self.newLine()
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.columnPosition;

                let colorCode = self.colorCode;
                self.buffer.chars[row][col] = ScreenChar {
                    asciiCharacter: byte,
                    colorCode,
                };
                self.columnPosition += 1;
            }
        }
    }

    fn newLine(&mut self) {
        
    }

    pub fn writeString(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                //printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.writeByte(byte),
                _ => self.writeByte(0xfe), //not in printable ASCII range
            }
        }
    }
}

//prints to a line
pub fn printr() {
    let mut writer = Writer {
        columnPosition: 0,
        colorCode: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.writeByte(b'H');
    writer.writeString("ello ");
    writer.writeString("World!");
}