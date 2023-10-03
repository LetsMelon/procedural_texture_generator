// Source: https://lpc.opengameart.org/content/8x8-ascii-bitmap-font-with-c-source
#[derive(Debug)]
#[repr(u64)]
pub enum BitmapChar {
    Space = 0x0,
    ExclamationMark = 0x808080800080000,
    Unknown = 0x0000542a542a542a,
    Number0 = 0x1824424224180000,
    Number1 = 0x8180808081C0000,
    Number2 = 0x3C420418207E0000,
    Number3 = 0x3C420418423C0000,
    Number4 = 0x81828487C080000,
    Number5 = 0x7E407C02423C0000,
    Number6 = 0x3C407C42423C0000,
    Number7 = 0x7E04081020400000,
    Number8 = 0x3C423C42423C0000,
    Number9 = 0x3C42423E023C0000,
    UpperA = 0x1818243C42420000,
    UpperB = 0x7844784444780000,
    UpperC = 0x3844808044380000,
    UpperD = 0x7844444444780000,
    UpperE = 0x7C407840407C0000,
    UpperF = 0x7C40784040400000,
    UpperG = 0x3844809C44380000,
    UpperH = 0x42427E4242420000,
    UpperI = 0x3E080808083E0000,
    UpperJ = 0x1C04040444380000,
    UpperK = 0x4448507048440000,
    UpperL = 0x40404040407E0000,
    UpperM = 0x4163554941410000,
    UpperN = 0x4262524A46420000,
    UpperO = 0x1C222222221C0000,
    UpperP = 0x7844784040400000,
    UpperQ = 0x1C222222221C0200,
    UpperR = 0x7844785048440000,
    UpperS = 0x1C22100C221C0000,
    UpperT = 0x7F08080808080000,
    UpperU = 0x42424242423C0000,
    UpperV = 0x8142422424180000,
    UpperW = 0x4141495563410000,
    UpperX = 0x4224181824420000,
    UpperY = 0x4122140808080000,
    UpperZ = 0x7E040810207E0000,
    LowerA = 0x3C023E463A0000,
    LowerB = 0x40407C42625C0000,
    LowerC = 0x1C20201C0000,
    LowerD = 0x2023E42463A0000,
    LowerE = 0x3C427E403C0000,
    LowerF = 0x18103810100000,
    LowerG = 0x344C44340438,
    LowerH = 0x2020382424240000,
    LowerI = 0x800080808080000,
    LowerJ = 0x800180808080870,
    LowerK = 0x20202428302C0000,
    LowerL = 0x1010101010180000,
    LowerM = 0x665A42420000,
    LowerN = 0x2E3222220000,
    LowerO = 0x3C42423C0000,
    LowerP = 0x5C62427C4040,
    LowerQ = 0x3A46423E0202,
    LowerR = 0x2C3220200000,
    LowerS = 0x1C201804380000,
    LowerT = 0x103C1010180000,
    LowerU = 0x2222261A0000,
    LowerV = 0x424224180000,
    LowerW = 0x81815A660000,
    LowerX = 0x422418660000,
    LowerY = 0x422214081060,
    LowerZ = 0x3C08103C0000,
}

impl BitmapChar {
    pub const CHAR_SIZE: (u32, u32) = (8, 8);

    pub fn from_char(input: char) -> BitmapChar {
        match input {
            ' ' => BitmapChar::Space,
            '!' => BitmapChar::ExclamationMark,
            '0' => BitmapChar::Number0,
            '1' => BitmapChar::Number1,
            '2' => BitmapChar::Number2,
            '3' => BitmapChar::Number3,
            '4' => BitmapChar::Number4,
            '5' => BitmapChar::Number5,
            '6' => BitmapChar::Number6,
            '7' => BitmapChar::Number7,
            '8' => BitmapChar::Number8,
            '9' => BitmapChar::Number9,
            'A' => BitmapChar::UpperA,
            'B' => BitmapChar::UpperB,
            'C' => BitmapChar::UpperC,
            'D' => BitmapChar::UpperD,
            'E' => BitmapChar::UpperE,
            'F' => BitmapChar::UpperF,
            'G' => BitmapChar::UpperG,
            'H' => BitmapChar::UpperH,
            'I' => BitmapChar::UpperI,
            'J' => BitmapChar::UpperJ,
            'K' => BitmapChar::UpperK,
            'L' => BitmapChar::UpperL,
            'M' => BitmapChar::UpperM,
            'N' => BitmapChar::UpperN,
            'O' => BitmapChar::UpperO,
            'P' => BitmapChar::UpperP,
            'Q' => BitmapChar::UpperQ,
            'R' => BitmapChar::UpperR,
            'S' => BitmapChar::UpperS,
            'T' => BitmapChar::UpperT,
            'U' => BitmapChar::UpperU,
            'V' => BitmapChar::UpperV,
            'W' => BitmapChar::UpperW,
            'X' => BitmapChar::UpperX,
            'Y' => BitmapChar::UpperY,
            'Z' => BitmapChar::UpperZ,
            'a' => BitmapChar::LowerA,
            'b' => BitmapChar::LowerB,
            'c' => BitmapChar::LowerC,
            'd' => BitmapChar::LowerD,
            'e' => BitmapChar::LowerE,
            'f' => BitmapChar::LowerF,
            'g' => BitmapChar::LowerG,
            'h' => BitmapChar::LowerH,
            'i' => BitmapChar::LowerI,
            'j' => BitmapChar::LowerJ,
            'k' => BitmapChar::LowerK,
            'l' => BitmapChar::LowerL,
            'm' => BitmapChar::LowerM,
            'n' => BitmapChar::LowerN,
            'o' => BitmapChar::LowerO,
            'p' => BitmapChar::LowerP,
            'q' => BitmapChar::LowerQ,
            'r' => BitmapChar::LowerR,
            's' => BitmapChar::LowerS,
            't' => BitmapChar::LowerT,
            'u' => BitmapChar::LowerU,
            'v' => BitmapChar::LowerV,
            'w' => BitmapChar::LowerW,
            'x' => BitmapChar::LowerX,
            'y' => BitmapChar::LowerY,
            'z' => BitmapChar::LowerZ,
            _ => BitmapChar::Unknown,
        }
    }
}
