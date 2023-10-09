use anyhow::Result;
use rusvid_core::prelude::{Pixel, Plane};

// Source: https://lpc.opengameart.org/content/8x8-ascii-bitmap-font-with-c-source
// TODO replace 'font' with another license, maybe cc
/*
/************************************************************************
* font.c
* Copyright (C) Lisa Milne 2014 <lisa@ltmnet.com>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful, but
* WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
* See the GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>
************************************************************************/

/* the values in this array are a 8x8 bitmap font for ascii characters */
static uint64_t font[128] = {
    0x7E7E7E7E7E7E0000,	/* NUL */
    0x7E7E7E7E7E7E0000,	/* SOH */
    0x7E7E7E7E7E7E0000,	/* STX */
    0x7E7E7E7E7E7E0000,	/* ETX */
    0x7E7E7E7E7E7E0000,	/* EOT */
    0x7E7E7E7E7E7E0000,	/* ENQ */
    0x7E7E7E7E7E7E0000,	/* ACK */
    0x7E7E7E7E7E7E0000,	/* BEL */
    0x7E7E7E7E7E7E0000,	/* BS */
    0x0,			    /* TAB */
    0x7E7E7E7E7E7E0000,	/* LF */
    0x7E7E7E7E7E7E0000,	/* VT */
    0x7E7E7E7E7E7E0000,	/* FF */
    0x7E7E7E7E7E7E0000,	/* CR */
    0x7E7E7E7E7E7E0000,	/* SO */
    0x7E7E7E7E7E7E0000,	/* SI */
    0x7E7E7E7E7E7E0000,	/* DLE */
    0x7E7E7E7E7E7E0000,	/* DC1 */
    0x7E7E7E7E7E7E0000,	/* DC2 */
    0x7E7E7E7E7E7E0000,	/* DC3 */
    0x7E7E7E7E7E7E0000,	/* DC4 */
    0x7E7E7E7E7E7E0000,	/* NAK */
    0x7E7E7E7E7E7E0000,	/* SYN */
    0x7E7E7E7E7E7E0000,	/* ETB */
    0x7E7E7E7E7E7E0000,	/* CAN */
    0x7E7E7E7E7E7E0000,	/* EM */
    0x7E7E7E7E7E7E0000,	/* SUB */
    0x7E7E7E7E7E7E0000,	/* ESC */
    0x7E7E7E7E7E7E0000,	/* FS */
    0x7E7E7E7E7E7E0000,	/* GS */
    0x7E7E7E7E7E7E0000,	/* RS */
    0x7E7E7E7E7E7E0000,	/* US */
    0x2828000000000000,	/* " */
    0x287C287C280000,	/* # */
    0x81E281C0A3C0800,	/* $ */
    0x6094681629060000,	/* % */
    0x1C20201926190000,	/* & */
    0x808000000000000,	/* ' */
    0x810202010080000,	/* ( */
    0x1008040408100000,	/* ) */
    0x2A1C3E1C2A000000,	/* * */
    0x8083E08080000,	/* + */
    0x81000,		    /* , */
    0x3C00000000,		/* - */
    0x80000,		    /* . */
    0x204081020400000,	/* / */
    0x80000080000,		/* : */
    0x80000081000,		/* ; */
    0x6186018060000,	/* < */
    0x7E007E000000,		/* = */
    0x60180618600000,	/* > */
    0x3844041800100000,	/* ? */
    0x3C449C945C201C,	/* @ */
    0x3820202020380000,	/* [ */
    0x4020100804020000,	/* \ */
    0x3808080808380000,	/* ] */
    0x1028000000000000,	/* ^ */
    0x1008000000000000,	/* ` */
    0x1C103030101C0000,	/* { */
    0x808080808080800,	/* | */
    0x38080C0C08380000,	/* } */
    0x324C000000,		/* ~ */
    0x7E7E7E7E7E7E0000	/* DEL */
};
 */
#[derive(Debug)]
#[repr(u64)]
pub enum BitmapChar {
    Space = 0x0,
    ExclamationMark = 0x808080800080000,
    Unknown = 0x0000542a542a542a,
    Underscore = 0x7E0000,
    Colon = 0x80000080000,

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
            '_' => BitmapChar::Underscore,
            ':' => BitmapChar::Colon,

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

    pub fn calculate_rect<S: AsRef<str>>(input: S, scale: u32) -> (u32, u32) {
        (
            BitmapChar::CHAR_SIZE.0 * scale * input.as_ref().len() as u32,
            BitmapChar::CHAR_SIZE.1 * scale,
        )
    }

    pub fn render_single_with_scale(
        plane: &mut Plane,
        pos: (u32, u32),
        character: char,
        color: Pixel,
        scale: u32,
    ) -> Result<(u32, u32)> {
        let bitmap = BitmapChar::from_char(character) as u64;

        for delta_x in 0..(BitmapChar::CHAR_SIZE.0 * scale) {
            for delta_y in 0..(BitmapChar::CHAR_SIZE.1 * scale) {
                let pixel_x = pos.0 + delta_x;
                let pixel_y = pos.1 + delta_y;

                let char_x = delta_x / scale;
                let char_y = delta_y / scale;

                let bit_index = (BitmapChar::CHAR_SIZE.0 * BitmapChar::CHAR_SIZE.1 - 1)
                    - (char_y * BitmapChar::CHAR_SIZE.0 + char_x);

                let bit = ((bitmap >> bit_index) & 0x01) != 0;
                // TODO implement `Plane::inside -> bool`
                if bit && pixel_x < plane.width() && pixel_y < plane.height() {
                    plane.put_pixel(pixel_x, pixel_y, color)?;
                }
            }
        }

        Ok((
            BitmapChar::CHAR_SIZE.0 * scale,
            BitmapChar::CHAR_SIZE.1 * scale,
        ))
    }

    pub fn render_single(
        plane: &mut Plane,
        pos: (u32, u32),
        character: char,
        color: Pixel,
    ) -> Result<(u32, u32)> {
        BitmapChar::render_single_with_scale(plane, pos, character, color, 1)
    }

    pub fn render_multiple_with_scale<S: AsRef<str>>(
        plane: &mut Plane,
        pos: (u32, u32),
        text: S,
        color: Pixel,
        scale: u32,
    ) -> Result<(u32, u32)> {
        let text: &str = text.as_ref();

        let mut delta_drawn_x = 0;
        let mut delta_drawn_y = 0;

        for item in text.chars() {
            let delta = BitmapChar::render_single_with_scale(
                plane,
                (pos.0 + delta_drawn_x, pos.1),
                item,
                color,
                scale,
            )?;
            delta_drawn_x += delta.0;
            delta_drawn_y = delta.1;
        }

        Ok((delta_drawn_x, delta_drawn_y))
    }
}
