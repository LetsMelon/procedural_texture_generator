use anyhow::Result;
use rusvid_core::prelude::Pixel;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::{Node, SpaceInfo};

#[derive(Debug)]
pub struct Mix {
    space_info: SpaceInfo,
}

impl Mix {
    pub fn new() -> Self {
        Mix {
            space_info: SpaceInfo::default(),
        }
    }
}

impl Node for Mix {
    fn generate(
        &self,
        _position: &Coordinate,
        _size: &(u32, u32),
        input: &[InputOutputValue],
    ) -> Result<InputOutputValue> {
        assert_eq!(input.len(), 3);

        // `delta` must be from type `InputOutputValue::Float`
        let deltas = match input[2] {
            InputOutputValue::Float(value) => [value, value, value, value],
            InputOutputValue::Nothing
            | InputOutputValue::Pixel(_)
            | InputOutputValue::U8X3Array(_)
            | InputOutputValue::U8X4Array(_) => todo!(),
            InputOutputValue::F64X3Array(values) => [values[0], values[1], values[2], 1.0],
            InputOutputValue::F64X4Array(values) => [values[0], values[1], values[2], values[3]],
        };

        let values = input[0]
            .to_common_ground()?
            .to_raw_float()
            .map(|item| item as f64)
            .zip(
                input[1]
                    .to_common_ground()?
                    .to_raw_float()
                    .map(|item| item as f64),
            )
            .zip(deltas)
            .map(|((first, second), delta)| (first * delta + second * (1.0 - delta)) as u8)
            .collect::<Vec<_>>();

        Ok(InputOutputValue::Pixel(Pixel::new(
            values[0], values[1], values[2], values[3],
        )))
    }

    fn space_info(&self) -> &SpaceInfo {
        &self.space_info
    }

    fn space_info_mut(&mut self) -> &mut SpaceInfo {
        &mut self.space_info
    }
}

#[cfg(test)]
mod tests {
    use rusvid_core::prelude::Pixel;

    use super::Mix;
    use crate::generator::Generator;
    use crate::input_output_value::InputOutputValue;
    use crate::library::noise::Noise;
    use crate::library::static_value::StaticValue;
    use crate::link::Link;

    #[test]
    fn just_works() {
        let mut generator = Generator::new();

        let id_mix = generator.add_node(Mix::new());
        let id_static_mix_factor =
            generator.add_node(StaticValue::new(InputOutputValue::Float(0.5)));
        let id_static_color = generator.add_node(StaticValue::new(InputOutputValue::Pixel(
            Pixel::new(255, 0, 100, 255),
        )));
        let id_noise = generator.add_node(Noise::new(1));
        let id_output = generator.output_node();

        generator.add_edge_named(Link::new(id_static_mix_factor, id_mix), "value");
        generator.add_edge_named(Link::new(id_static_color, id_mix), "input1");
        generator.add_edge_named(Link::new(id_noise, id_mix), "input2");
        generator.add_edge(Link::new(id_mix, id_output));

        let _ = generator.generate(100, 100).unwrap();
    }
}
