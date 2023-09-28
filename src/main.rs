use procedural_texture_generator::coordinate::Coordinate;
use procedural_texture_generator::generator::Generator;
use procedural_texture_generator::library::map::Map;
use procedural_texture_generator::library::noise::Noise;
use rusvid_core::prelude::Pixel;

fn main() {
    let generator = Generator {
        nodes: vec![
            // Box::new(BlankColor {
            //     color: Pixel::new(255, 0, 100, 255),
            // }),
            // Box::new(Pattern),
            Box::new({
                let mut n = Noise::new(1);

                n.set_scale(Coordinate::new(4.0, 4.0, 1.0));
                n.set_offset(Coordinate::new(0.0, 0.0, 0.0));

                n
            }),
            // Box::new(Normalize::new(0.75)),
            // Box::new({
            //     let colors = [
            //         Pixel::new(166, 99, 204, 255),
            //         Pixel::new(0, 168, 232, 255),
            //         Pixel::new(255, 77, 109, 255),
            //         Pixel::new(157, 78, 221, 255),
            //         Pixel::new(255, 133, 0, 255),
            //     ];
            //
            //     let steps = 0.05;
            //
            //     let colors = (0..50)
            //         .map(|item| (item as f64) * steps)
            //         .filter(|item| *item <= 1.0)
            //         .enumerate()
            //         .map(|(i, item)| (colors[i % colors.len()], item))
            //         .collect::<Vec<_>>();
            //
            //     Map::new(colors)
            // }),
        ],
    };

    let plane = generator.generate().unwrap();
    plane.save_as_bmp("out.bmp").unwrap();
}
