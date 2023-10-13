mod bitmap;
pub mod coordinate;
pub mod generator;
pub mod input_output_value;
pub mod library;
pub mod link;
pub mod node;

use std::sync::{Mutex, MutexGuard};

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
use crate::node::Node;

static GENERATOR: OnceCell<Mutex<Generator>> = OnceCell::new();
static SELECTED_NODE: Mutex<Option<NodeIndex>> = Mutex::new(None);

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

trait RenderNode {
    fn render(&self, plane: &mut Plane) -> anyhow::Result<()>;
}

fn generator_mutex() -> &'static Mutex<generator::Generator> {
    GENERATOR.get_or_init(|| {
        let mut generator = Generator::new();
        let node_noise = generator.add_node({
            let mut n = Noise::new(1);

            n.set_scale(Coordinate::new(8.0, 8.0, 1.0));
            n.set_offset(Coordinate::new(0.0, 0.0, 0.0));

            n.space_info_mut().name = "Noise".to_string();

            n
        });
        let node_map = generator.add_node({
            // let mut n = Map::new(vec![
            //     (InputOutputValue::Float(1.0), 0.30),
            //     (InputOutputValue::Float(0.0), 0.3001),
            // ]);
            let mut n = Map::new(vec![
                (InputOutputValue::Pixel(Pixel::new(0, 0, 100, 255)), 0.20),
                (InputOutputValue::Pixel(Pixel::new(100, 0, 0, 255)), 0.30),
                (InputOutputValue::Pixel(Pixel::new(0, 100, 100, 255)), 0.50),
                (InputOutputValue::Pixel(Pixel::new(255, 255, 0, 255)), 1.0),
            ]);

            n.space_info_mut().name = "Map".to_string();

            n
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

        Mutex::new(generator)
    })
}

fn get_generator() -> MutexGuard<'static, generator::Generator> {
    generator_mutex().lock().unwrap()
}

fn get_selected_node() -> MutexGuard<'static, Option<NodeIndex>> {
    SELECTED_NODE.lock().unwrap()
}

fn set_selected_node(value: Option<NodeIndex>) {
    let mut selected_node = SELECTED_NODE.lock().unwrap();

    *selected_node = value;
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
    pos1: (i64, i64),
    pos2: (i64, i64),
    color: Pixel,
    stroke_weight: u32,
) -> anyhow::Result<()> {
    let mut x1 = pos1.0 as i64;
    let mut y1 = pos1.1 as i64;
    let x2 = pos2.0 as i64;
    let y2 = pos2.1 as i64;

    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    while x1 != x2 || y1 != y2 {
        if x1 > 0 && x1 <= plane.width() as i64 && y1 > 0 && y1 <= plane.height() as i64 {
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

    let mut generator = get_generator();

    if let Some(node) = generator
        .internal_graph
        .node_weight_mut(NodeIndex::new(node))
    {
        let mut n = node.borrow_mut();
        n.space_info_mut().position = (position_x, position_y);
    }

    Ok(())
}

#[wasm_bindgen]
pub fn move_node(node: usize, delta_x: i32, delta_y: i32) -> Result<(), JsValue> {
    println!("Move node: {:?}", &node);

    let generator = get_generator();
    if let Some(n) = generator.internal_graph.node_weight(NodeIndex::new(node)) {
        let position = n.borrow().space_info().position;

        drop(generator);

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
    println!("No panic here: 1:");

    let generator = get_generator();
    let mut plane = Plane::new(width, height).unwrap();
    println!("No panic here: 2:");

    println!("Draw edges:");

    for edge in generator.internal_graph.raw_edges() {
        println!("No panic here: 3:");
        println!("edge: {edge:?}");
        let source_index = edge.source();
        let target_index = edge.target();

        let source = generator
            .internal_graph
            .node_weight(source_index)
            .map(|item| item.borrow());
        let target = generator
            .internal_graph
            .node_weight(target_index)
            .map(|item| item.borrow());

        if let (Some(source), Some(target)) = (source, target) {
            if source.space_info().position == target.space_info().position {
                continue;
            }

            println!("{:?}\n->\n{:?}", source, target);

            println!(
                "source: {:?}\ntarget: {:?}",
                source.space_info().position,
                target.space_info().position
            );

            let source_center = (
                ((source.space_info().position.0 as i64) + (source.space_info().size.0 as i64) / 2),
                ((source.space_info().position.1 as i64) + (source.space_info().size.1 as i64) / 2),
            );
            let target_center = (
                ((target.space_info().position.0 as i64) + (target.space_info().size.0 as i64) / 2),
                ((target.space_info().position.1 as i64) + (target.space_info().size.1 as i64) / 2),
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
    }

    println!("No panic here: 4:");
    for node in generator.connected_nodes_to_output() {
        let node = node.borrow();

        println!("Rendering node: {:?}", &node);
        node.render(&mut plane).unwrap();
    }

    println!("No panic here: 5:");
    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(plane.as_data_flatten().as_slice()),
        width,
        height,
    )?;
    ctx.put_image_data(&data, 0.0, 0.0)?;
    println!("No panic here: 6:");

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
        .filter_map(|id| {
            let node = generator.internal_graph.node_weight(id);

            match node {
                Some(node) => Some((id, node.borrow())),
                None => None,
            }
        })
        .sorted_by(|(_, a), (_, b)| a.space_info().z_index.cmp(&b.space_info().z_index))
        .fold(None, |acc, (id, item)| {
            if acc.is_some() {
                return acc;
            }

            println!(
                "Check with node {:?} at position: {:?} with size {:?}",
                &id,
                item.space_info().position,
                item.space_info().size
            );

            // TODO not right!
            if click_x >= item.space_info().position.0
                && click_x <= (item.space_info().position.0 + item.space_info().size.0)
                && click_y >= item.space_info().position.1
                && click_y <= (item.space_info().position.1 + item.space_info().size.1)
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

    drop(get_generator())
}
