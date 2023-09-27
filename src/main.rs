use procedural_texture_generator::library::blank_color::BlankColor;
use procedural_texture_generator::library::pattern::Pattern;
use procedural_texture_generator::Generator;
use rusvid_core::pixel::Pixel;

fn main() {
    let generator = Generator {
        nodes: vec![
            Box::new(BlankColor {
                color: Pixel::new(255, 0, 100, 255),
            }),
            Box::new(Pattern),
        ],
    };

    let plane = generator.generate().unwrap();
    plane.save_as_bmp("out.bmp").unwrap();
}
