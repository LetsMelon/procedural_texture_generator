mod bitmap;
pub mod coordinate;
pub mod generator;
pub mod input_output_value;
pub mod library;
pub mod link;
pub mod node;

use generator::SpaceNode;
use itertools::Itertools;
use once_cell::sync::OnceCell;
use petgraph::stable_graph::NodeIndex;
use rusvid_core::prelude::{Pixel, Plane};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use crate::coordinate::Coordinate;
use crate::generator::Generator;
use crate::input_output_value::InputOutputValue;
use crate::library::map::Map;
use crate::library::noise::Noise;
use crate::library::static_value::StaticValue;
use crate::link::Link;

static mut GENERATOR: OnceCell<Generator> = OnceCell::new();
static mut SELECTED_NODE: Option<NodeIndex> = None;

#[cfg(target_arch = "wasm32")]
#[macro_export]
#[allow(unused)]
macro_rules! println {
    () => {
        web_sys::console::log_0()
    };
    ($($arg:tt)*) => {{
        web_sys::console::log_1(&format!($($arg)*).into());
    }};
}

#[cfg(target_arch = "wasm32")]
#[macro_export]
#[allow(unused)]
macro_rules! dbg {
    () => {
        $crate::println!("[{}:{}]", $crate::file!(), $crate::line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::println!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

fn get_generator() -> &'static Generator {
    unsafe {
        GENERATOR.get_or_init(|| {
            let mut generator = Generator::new();
            let node_noise = generator.add_node_with_space({
                let mut sn = SpaceNode::new({
                    let mut n = Noise::new(1);

                    n.set_scale(Coordinate::new(4.0, 4.0, 1.0));
                    n.set_offset(Coordinate::new(0.0, 0.0, 0.0));

                    n
                });

                sn.name = "Noise".to_string();

                sn
            });
            let node_map = generator.add_node_with_space({
                let mut sn = SpaceNode::new(Map::new(vec![
                    (InputOutputValue::Float(1.0), 0.30),
                    (InputOutputValue::Float(0.0), 0.3001),
                ]));

                sn.name = "Map".to_string();

                sn
            });
            let _node_static = generator.add_node(StaticValue::new(InputOutputValue::Pixel(
                Pixel::new(255, 100, 0, 255),
            )));
            let node_output = generator.output_node();

            println!("Created nodes");

            // generator.add_edge(Link::new(node_noise, node_output));
            generator.add_edge(Link::new(node_noise, node_map));
            generator.add_edge(Link::new(node_map, node_output));
            // generator.add_edge(Link::new(node_static, node_output));

            println!("Added links");

            generator
        })
    }
}

fn get_selected_node() -> Option<NodeIndex> {
    unsafe { SELECTED_NODE }
}

fn set_selected_node(value: Option<NodeIndex>) {
    unsafe {
        SELECTED_NODE = value;
    }
}

pub(crate) fn render_square(
    plane: &mut Plane,
    pos: (u32, u32),
    size: (u32, u32),
    color: Pixel,
) -> anyhow::Result<()> {
    for x in pos.0..(pos.0 + size.0).min(plane.height()) {
        for y in pos.1..(pos.1 + size.1).min(plane.width()) {
            plane.put_pixel(x, y, color)?;
        }
    }

    Ok(())
}

fn draw_circle(
    plane: &mut Plane,
    pos: (u32, u32),
    color: Pixel,
    radius: u32,
) -> anyhow::Result<()> {
    let radius = radius as i32;
    let f_radius = radius as f32;

    let width = plane.width();
    let height = plane.height();

    for (x, y) in ((radius * -1)..radius)
        .cartesian_product((radius * -1)..radius)
        .sorted_by_key(|(x, y)| x * 4157 + y * 6481)
        .dedup()
        .filter(|(delta_x, delta_y)| {
            let xx = *delta_x as f32;
            let yy = *delta_y as f32;

            (xx.powf(2.0) + yy.powf(2.0)).sqrt() < f_radius
        })
        .map(|(delta_x, delta_y)| (pos.0 as i32 + delta_x, pos.1 as i32 + delta_y))
        .filter(|(x, y)| !(*x < 0 || *y < 0 || *x > width as i32 || *y > height as i32))
    {
        plane.put_pixel(x as u32, y as u32, color)?;
    }

    Ok(())
}

fn draw_line(
    plane: &mut Plane,
    pos1: (u32, u32),
    pos2: (u32, u32),
    color: Pixel,
    stroke_weight: u32,
) -> anyhow::Result<()> {
    let mut x1 = pos1.0 as i32;
    let mut y1 = pos1.1 as i32;
    let x2 = pos2.0 as i32;
    let y2 = pos2.1 as i32;

    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    while x1 != x2 || y1 != y2 {
        if x1 > 0 && x1 <= plane.width() as i32 && y1 > 0 && y1 <= plane.height() as i32 {
            // plane.put_pixel(x1 as u32, y1 as u32, color)?;
            draw_circle(plane, (x1 as u32, y1 as u32), color, stroke_weight)?;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x1 += sx;
        }

        if e2 < dx {
            err += dx;
            y1 += sy;
        }
    }

    Ok(())
}

#[wasm_bindgen]
pub fn move_node_to(node: usize, position_x: u32, position_y: u32) -> Result<(), JsValue> {
    println!("Move node {:?} to {:?}", &node, (position_x, position_y));

    let generator = unsafe { GENERATOR.get_mut().unwrap() };

    if let Some(node) = generator
        .internal_graph
        .node_weight_mut(NodeIndex::new(node))
    {
        let n = unsafe { node.as_ptr().as_mut() }.unwrap();
        n.position = (position_x, position_y);
    }

    Ok(())
}

#[wasm_bindgen]
pub fn move_node(node: usize, delta_x: i32, delta_y: i32) -> Result<(), JsValue> {
    println!("Move node: {:?}", &node);

    let generator = get_generator();
    if let Some(n) = generator.internal_graph.node_weight(NodeIndex::new(node)) {
        let n = unsafe { n.as_ptr().as_mut() }.unwrap();
        let position = n.position;

        move_node_to(
            node,
            (position.0 as i32 + delta_x) as u32,
            (position.1 as i32 + delta_y) as u32,
        )?;
    }

    Ok(())
}

#[wasm_bindgen]
pub fn nodes(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    println!("Nodes call:");

    let generator = get_generator();
    let mut plane = Plane::new(width, height).unwrap();

    println!("Draw edges:");

    for edge in generator.internal_graph.raw_edges() {
        let source_index = edge.source();
        let target_index = edge.target();

        let source = unsafe {
            generator
                .internal_graph
                .node_weight(source_index)
                .unwrap()
                .as_ptr()
                .as_ref()
                .unwrap()
        };
        let target = unsafe {
            generator
                .internal_graph
                .node_weight(target_index)
                .unwrap()
                .as_ptr()
                .as_ref()
                .unwrap()
        };

        if source.position == target.position {
            continue;
        }

        println!("{:?} -> {:?}", source, target);

        let source_center = (
            source.position.0 + source.size.0 / 2,
            source.position.1 + source.size.1 / 2,
        );
        let target_center = (
            target.position.0 + target.size.0 / 2,
            target.position.1 + target.size.1 / 2,
        );

        println!("source_center: {:?}", source_center);
        println!("target_center: {:?}", target_center);

        draw_line(
            &mut plane,
            source_center,
            target_center,
            Pixel::new(15, 75, 165, 255),
            5,
        )
        .unwrap();
    }

    for node in generator.connected_nodes_to_output() {
        let node = node.borrow();

        println!("Rendering node: {:?}", &node);
        node.render(&mut plane).unwrap();
    }

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
    println!("Render call:");

    let generator = get_generator();
    let plane = generator.generate(width, height).unwrap();

    println!("Generated plane");

    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(plane.as_data_flatten().as_slice()),
        width,
        height,
    )?;
    ctx.put_image_data(&data, 0.0, 0.0)?;

    Ok(())
}

#[wasm_bindgen]
pub fn canvas_click(
    _ctx: &CanvasRenderingContext2d,
    click_x: u32,
    click_y: u32,
) -> Result<(), JsValue> {
    println!("Canvas click call:");
    println!("Click position: {:?}", (click_x, click_y));

    let generator = get_generator();

    let maybe_node = generator
        .internal_graph
        .node_indices()
        .map(|id| {
            (id, unsafe {
                generator
                    .internal_graph
                    .node_weight(id)
                    .unwrap()
                    .as_ptr()
                    .as_ref()
                    .unwrap()
            })
        })
        .sorted_by(|(_, a), (_, b)| a.z_index.cmp(&b.z_index))
        .fold(None, |acc, (id, item)| {
            if acc.is_some() {
                return acc;
            }

            println!(
                "Check with node {:?} at position: {:?} with size {:?}",
                &id, item.position, item.size
            );

            // TODO not right!
            if click_x >= item.position.0
                && click_x <= (item.position.0 + item.size.0)
                && click_y >= item.position.1
                && click_y <= (item.position.1 + item.size.1)
            {
                return Some((id, item));
            }

            None
        });

    println!("Clicked on: {:?}", &maybe_node);

    match maybe_node {
        Some((id, _)) => set_selected_node(Some(id)),
        None => set_selected_node(None),
    }

    Ok(())
}

#[wasm_bindgen]
pub fn canvas_click_active() -> Option<usize> {
    get_selected_node().map(|item| item.index())
}

#[wasm_bindgen]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let _ = get_generator();
}
