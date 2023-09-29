use anyhow::Result;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::Node;

#[derive(Debug)]
pub struct StaticValue {
    value: InputOutputValue,
}

impl StaticValue {
    pub fn new(value: InputOutputValue) -> Self {
        StaticValue { value }
    }
}

impl Node for StaticValue {
    fn generate(
        &self,
        _position: &Coordinate,
        _size: &(usize, usize),
        _input: InputOutputValue,
    ) -> Result<InputOutputValue> {
        Ok(self.value)
    }
}

#[cfg(test)]
mod tests {
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
                node.generate(&Coordinate::new_x(0.0), &(0, 0), InputOutputValue::Nothing)
                    .unwrap(),
                value_to_test
            );
        }
    }
}
