use std::collections::HashMap;

use anyhow::Result;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::{Node, SpaceInfo};

#[derive(Debug)]
pub struct StaticValue {
    value: InputOutputValue,

    space_info: SpaceInfo,
}

impl StaticValue {
    pub fn new(value: InputOutputValue) -> Self {
        StaticValue {
            value,

            space_info: SpaceInfo::default(),
        }
    }
}

impl Node for StaticValue {
    fn generate(
        &self,
        _position: &Coordinate,
        _size: &(u32, u32),
        _input: HashMap<String, InputOutputValue>,
    ) -> Result<InputOutputValue> {
        Ok(self.value)
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
    use std::collections::HashMap;

    use rusvid_core::pixel::Pixel;

    use super::StaticValue;
    use crate::coordinate::Coordinate;
    use crate::input_output_value::InputOutputValue;
    use crate::node::Node;

    #[test]
    fn always_returns_the_same_value() {
        let values_to_test = [
            InputOutputValue::Float(1.0),
            InputOutputValue::Nothing,
            InputOutputValue::Pixel(Pixel::new(255, 0, 0, 255)),
        ];

        for value_to_test in values_to_test {
            let node = StaticValue::new(value_to_test);
            assert_eq!(
                node.generate(&Coordinate::new_x(0.0), &(0, 0), HashMap::new())
                    .unwrap(),
                value_to_test
            );
        }
    }
}
