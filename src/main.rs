use procedural_texture_generator::coordinate::Coordinate;
use procedural_texture_generator::generator::Generator;
use procedural_texture_generator::input_output_value::InputOutputValue;
use procedural_texture_generator::library::map::Map;
use procedural_texture_generator::library::noise::Noise;
use procedural_texture_generator::library::static_value::StaticValue;
use procedural_texture_generator::link::Link;
use rusvid_core::prelude::Pixel;

fn main() {
    // let generator = Generator {
    //     nodes: vec![
    //         noise_node.clone(),
    //         // Box::new({
    //         //     let colors = [
    //         //         Pixel::new(166, 99, 204, 255),
    //         //         Pixel::new(0, 168, 232, 255),
    //         //         Pixel::new(255, 77, 109, 255),
    //         //         Pixel::new(157, 78, 221, 255),
    //         //         Pixel::new(255, 133, 0, 255),
    //         //     ];
    //         //
    //         //     let steps = 0.05;
    //         //
    //         //     let colors = (0..50)
    //         //         .map(|item| (item as f64) * steps)
    //         //         .filter(|item| *item <= 1.0)
    //         //         .enumerate()
    //         //         .map(|(i, item)| (colors[i % colors.len()], item))
    //         //         .collect::<Vec<_>>();
    //         //
    //         //     Map::new(colors)
    //         // }),
    //     ],
    //     links: vec![Link::new(noise_node.clone(), NodeOrOutput::Output)],
    // };
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
    let _node_static = generator.add_node(StaticValue::new(InputOutputValue::Pixel(Pixel::new(
        255, 100, 0, 255,
    ))));
    let node_output = generator.output_node();

    // generator.add_edge(Link::new(node_noise, node_output));
    generator.add_edge(Link::new(node_noise, node_map));
    generator.add_edge(Link::new(node_map, node_output));
    // generator.add_edge(Link::new(node_static, node_output));

    let plane = generator.generate().unwrap();
    plane.save_as_bmp("out.bmp").unwrap();
}
