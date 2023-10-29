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
use crate::library::mix::Mix;
use crate::library::noise::Noise;
use crate::library::output::Output;
use crate::library::static_value::StaticValue;
use crate::link::Link;
use crate::node::Node;
use crate::utils::{draw_circle, draw_line, render_square};

static GENERATOR: OnceCell<Mutex<Generator>> = OnceCell::new();

static CACHED_GENERATOR_OUTPUT: OnceCell<Plane> = OnceCell::new();

static SELECTED_NODE: Mutex<Option<NodeIndex>> = Mutex::new(None);

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

fn generator_mutex() -> &'static Mutex<Generator> {
    GENERATOR.get_or_init(|| {
        let mut generator = Generator::new();

        let node_mix = generator.add_node({
            let mut m = Mix::new();
            m.space_info_mut().name = "Mix".to_string();
            m
        });
        let node_noise = generator.add_node({
            let mut n = Noise::new(1);

            let scale = 10.0;
            n.set_scale(Coordinate::new_xy(scale, scale));

            n.space_info_mut().name = "Noise".to_string();

            n
        });
        let node_input1 = generator.add_node(StaticValue::new(InputOutputValue::Pixel(
            Pixel::new(255, 0, 100, 255),
        )));
        let node_input2 = generator.add_node(StaticValue::new(InputOutputValue::Pixel(
            Pixel::new(0, 255, 150, 255),
        )));
        let node_output = generator.output_node();

        println!("Created nodes");

        generator.add_edge_named(Link::new(node_noise, node_mix), "value");
        generator.add_edge_named(Link::new(node_input1, node_mix), "input1");
        generator.add_edge_named(Link::new(node_input2, node_mix), "input2");
        generator.add_edge(Link::new(node_mix, node_output));

        println!("Added links");

        Mutex::new(generator)
    })
}

fn get_generator() -> MutexGuard<'static, Generator> {
    generator_mutex().lock().unwrap()
}

fn get_selected_node() -> MutexGuard<'static, Option<NodeIndex>> {
    SELECTED_NODE.lock().unwrap()
}

fn set_selected_node(value: Option<NodeIndex>) {
    let mut selected_node = SELECTED_NODE.lock().unwrap();

    *selected_node = value;
}

#[wasm_bindgen]
pub fn move_node_to(node: usize, position_x: i64, position_y: i64) -> Result<(), JsValue> {
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
pub fn move_node(node: usize, delta_x: i64, delta_y: i64) -> Result<(), JsValue> {
    println!("Move node: {:?}, {:?}", &node, (delta_x, delta_y));

    let generator = get_generator();
    if let Some(n) = generator.internal_graph.node_weight(NodeIndex::new(node)) {
        let position = n.borrow().space_info().position;

        drop(generator);

        move_node_to(
            node,
            position.0 as i64 + delta_x,
            position.1 as i64 + delta_y,
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

            let source_center = (
                ((source.space_info().position.0) + (source.space_info().size.0 as i64) / 2),
                ((source.space_info().position.1) + (source.space_info().size.1 as i64) / 2),
            );
            let target_center = (
                ((target.space_info().position.0) + (target.space_info().size.0 as i64) / 2),
                ((target.space_info().position.1) + (target.space_info().size.1 as i64) / 2),
            );

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

    for node in generator.connected_nodes_to_output() {
        let node_borrowed = node.borrow();

        println!("Rendering node: {:?}", &node_borrowed,);
        node_borrowed.render(&mut plane).unwrap();

        if node_borrowed.is_output() {
            // TODO There must be a better way to cast a 'dyn Node' to a '&Output'.
            if let Some(output_ref) = unsafe { (node.as_ref().as_ptr() as *mut Output).as_ref() } {
                output_ref
                    .draw_generated_output_into_node(
                        &mut plane,
                        CACHED_GENERATOR_OUTPUT.get().unwrap(),
                    )
                    .unwrap()
            }
        }
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
            let click_x = click_x as i64;
            let click_y = click_y as i64;

            if click_x >= item.space_info().position.0
                && click_x <= (item.space_info().position.0 + (item.space_info().size.0 as i64))
                && click_y >= item.space_info().position.1
                && click_y <= (item.space_info().position.1 + (item.space_info().size.1 as i64))
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

    let generator = get_generator();

    CACHED_GENERATOR_OUTPUT
        .set(generator.generate(200, 200).unwrap())
        .unwrap();
}
