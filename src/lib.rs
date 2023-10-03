mod bitmap;
pub mod coordinate;
pub mod generator;
pub mod input_output_value;
pub mod library;
pub mod link;
pub mod node;

use bitmap::BitmapChar;
use once_cell::sync::OnceCell;
use rusvid_core::prelude::{Pixel, Plane};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{console, CanvasRenderingContext2d, ImageData};

use crate::coordinate::Coordinate;
use crate::generator::Generator;
use crate::input_output_value::InputOutputValue;
use crate::library::map::Map;
use crate::library::noise::Noise;
use crate::library::static_value::StaticValue;
use crate::link::Link;

static GENERATOR: OnceCell<Generator> = OnceCell::new();

fn get_generator() -> &'static Generator {
    GENERATOR.get_or_init(|| {
        let mut generator = Generator::new();
        let node_noise = generator.add_node({
            let mut n = Noise::new(1);

            n.set_scale(Coordinate::new(4.0, 4.0, 1.0));
            n.set_offset(Coordinate::new(0.0, 0.0, 0.0));

            n
        });
        let node_map = generator.add_node(Map::new(vec![
            (InputOutputValue::Float(1.0), 0.30),
            (InputOutputValue::Float(0.0), 0.3001),
        ]));
        let _node_static = generator.add_node(StaticValue::new(InputOutputValue::Pixel(
            Pixel::new(255, 100, 0, 255),
        )));
        let node_output = generator.output_node();

        console::log_1(&"Created nodes".into());

        // generator.add_edge(Link::new(node_noise, node_output));
        generator.add_edge(Link::new(node_noise, node_map));
        generator.add_edge(Link::new(node_map, node_output));
        // generator.add_edge(Link::new(node_static, node_output));

        console::log_1(&"Added links".into());

        generator
    })
}

fn render_square(plane: &mut Plane, pos: (u32, u32), size: (u32, u32)) -> anyhow::Result<()> {
    for x in pos.0..(pos.0 + size.0).min(plane.height()) {
        for y in pos.1..(pos.1 + size.1).min(plane.width()) {
            plane.put_pixel(x, y, Pixel::new(0, 255, 100, 255))?;
        }
    }

    Ok(())
}

fn render_char(
    plane: &mut Plane,
    pos: (u32, u32),
    character: char,
    color: Pixel,
    scale: u32,
) -> anyhow::Result<()> {
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

    Ok(())
}

#[wasm_bindgen]
pub fn nodes(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    console::log_1(&"Nodes call:".into());

    let _generator = get_generator();
    let mut plane = Plane::new_with_fill(width, height, Pixel::new(255, 100, 0, 255)).unwrap();

    render_square(&mut plane, (100, 100), (100, 100)).unwrap();

    // Unknown char
    render_char(&mut plane, (220, 250), '@', Pixel::new(100, 0, 255, 255), 1).unwrap();
    render_char(&mut plane, (250, 250), 'A', Pixel::new(100, 0, 255, 255), 1).unwrap();
    render_char(
        &mut plane,
        (250 + 8, 250 - 8 / 4 * 3),
        'B',
        Pixel::new(100, 0, 255, 255),
        2,
    )
    .unwrap();
    render_char(
        &mut plane,
        (250 + 8 + 16, 250 - 8 / 4 * 3 - 16 / 4 * 3),
        'C',
        Pixel::new(100, 0, 255, 255),
        4,
    )
    .unwrap();
    render_char(
        &mut plane,
        (250 + 8 + 16 + 32, 250 - 8 / 4 * 3 - 16 / 4 * 3 - 32 / 4 * 3),
        'D',
        Pixel::new(100, 0, 255, 255),
        8,
    )
    .unwrap();
    render_char(
        &mut plane,
        (
            250 + 8 + 16 + 32 + 64,
            250 - 8 / 4 * 3 - 16 / 4 * 3 - 32 / 4 * 3 - 64 / 4 * 3,
        ),
        'E',
        Pixel::new(100, 0, 255, 255),
        16,
    )
    .unwrap();

    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(plane.as_data_flatten().as_slice()),
        width,
        height,
    )?;
    ctx.put_image_data(&data, 0.0, 0.0)?;

    Ok(())
}

#[wasm_bindgen]
pub fn render(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    console::log_1(&"Render call:".into());

    let generator = get_generator();
    let plane = generator.generate(width, height).unwrap();

    console::log_1(&"Generated plane".into());

    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(plane.as_data_flatten().as_slice()),
        width,
        height,
    )?;
    ctx.put_image_data(&data, 0.0, 0.0)?;

    Ok(())
}
