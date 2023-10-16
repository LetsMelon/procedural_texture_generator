use anyhow::Result;
use itertools::Itertools;
use rusvid_core::pixel::Pixel;
use rusvid_core::plane::Plane;

use crate::bitmap::BitmapChar;
use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::{Node, SpaceInfo};
use crate::utils::render_square;

#[derive(Debug)]
pub struct Map {
    // TODO maybe use `Range` instead of `f64` and use a separate struct
    steps: Vec<(InputOutputValue, f64)>,

    space_info: SpaceInfo,
}

impl Map {
    pub fn new(steps: Vec<(InputOutputValue, f64)>) -> Self {
        assert!(steps.len() >= 2);

        Map {
            steps,

            space_info: SpaceInfo::default(),
        }
    }
}

impl Node for Map {
    fn generate(
        &self,
        _position: &Coordinate,
        _size: &(u32, u32),
        input: &[InputOutputValue],
    ) -> Result<InputOutputValue> {
        // TODO copy method for the windows of the `self.steps`` from `render`
        let r = input[0].r_percentage()?;
        let g = input[0].g_percentage()?;
        let b = input[0].b_percentage()?;
        let avg = (r + g + b) / 3.0;

        // TODO the following functions should always return at least something because of `len(self.steps) >= 2`
        let mut first = self.steps.first().unwrap();
        let mut last = self.steps.last().unwrap();

        if self.steps.len() != 2 {
            for step in &self.steps {
                if step.1 < avg {
                    first = step;
                } else {
                    last = step;
                    break;
                }
            }
        }

        if avg < first.1 {
            Ok(first.0)
        } else if avg >= last.1 {
            Ok(last.0)
        } else {
            let v1 = [
                first.0.r_percentage()?,
                first.0.g_percentage()?,
                first.0.b_percentage()?,
                first.0.a_percentage()?,
            ];

            let v2 = [
                last.0.r_percentage()?,
                last.0.g_percentage()?,
                last.0.b_percentage()?,
                last.0.a_percentage()?,
            ];

            let span_delta = last.1 - first.1;
            let first_delta = avg - first.1;
            let percentage = first_delta / span_delta;

            let values = v1
                .iter()
                .zip(v2.iter())
                // .map(|(v1, v2)| (((v1 * percentage + v2 * (1.0 - percentage)) * 255.0) as u8))
                .map(|(v1, v2)| (((v1 * (1.0 - percentage) + v2 * percentage) * 255.0) as u8))
                .collect::<Vec<_>>();

            Ok(InputOutputValue::Pixel(Pixel::new(
                values[0], values[1], values[2], values[3],
            )))
        }
    }

    fn render(&self, plane: &mut Plane) -> Result<()> {
        let space_info = self.space_info();

        render_square(
            plane,
            space_info.position,
            space_info.size,
            space_info.color,
        )?;
        let (_, delta_y) = BitmapChar::render_multiple_with_scale(
            plane,
            (space_info.position.0, space_info.position.1 + 5),
            &space_info.name,
            Pixel::new(0, 0, 0, 255),
            2,
        )?;

        let (_, node_width) = space_info.size;
        let margin = 3;
        let box_width = node_width as i32 - 2 * margin;
        let box_height = delta_y;
        let box_pos = (
            space_info.position.0,
            space_info.position.1 + (delta_y as i64) * 3 / 2,
        );
        if box_width > 0 {
            let steps = self
                .steps
                .iter()
                .enumerate()
                .filter_map(|(i, (value, percentage))| {
                    let color = value.to_common_ground();

                    match color {
                        Ok(c) => Some((c, *percentage, i == 0, i == (self.steps.len() - 1))),
                        Err(_) => None,
                    }
                })
                .collect::<Vec<_>>();

            let windows = steps
                .iter()
                .circular_tuple_windows::<(&_, &_)>()
                .enumerate();

            for (i, (left, right)) in windows {
                if i == steps.len() - 1 {
                    break;
                }

                let r1 = left.0.get_r() as f64;
                let g1 = left.0.get_g() as f64;
                let b1 = left.0.get_b() as f64;

                let r2 = right.0.get_r() as f64;
                let g2 = right.0.get_g() as f64;
                let b2 = right.0.get_b() as f64;

                let x_start =
                    (if left.2 { 0.0 } else { left.1 } * (box_width as f64)).floor() as u32;

                let box_width = box_width as u32;
                for delta_x in x_start..box_width {
                    let x = box_pos.0 + margin as i64 + delta_x as i64;

                    let p = delta_x as f64 / (box_width - 1) as f64;

                    let color = match p {
                        _ if p <= left.1 && left.2 => left.0,
                        _ if p >= right.1 && right.3 => right.0,
                        _ => {
                            // the result from the subtraction _should_ be positive but to be safe we take the absolute
                            let delta_start = (p - left.1).abs();
                            let delta_end = (right.1 - p).abs();
                            // the delta_span _should_ never be zero because of the other clauses in the match statement,
                            // but to be safe we add the smallest fraction that `f64` can represent
                            let delta_span = delta_start + delta_end + f64::MIN_POSITIVE;

                            let delta_start_p = delta_end / delta_span;
                            let delta_end_p = 1.0 - delta_start_p;

                            let r = (r1 * delta_start_p + r2 * delta_end_p) as u8;
                            let g = (g1 * delta_start_p + g2 * delta_end_p) as u8;
                            let b = (b1 * delta_start_p + b2 * delta_end_p) as u8;

                            //TODO alpha channel

                            Pixel::new(r, g, b, 255)
                        }
                    };

                    if x >= 0 && x < plane.width() as i64 {
                        for delta_y in 0..box_height {
                            let y = box_pos.1 + delta_y as i64;

                            if y >= 0 && y < plane.width() as i64 {
                                let err = plane.put_pixel(x as u32, y as u32, color)?;
                                // if let Err(err) = err {
                                //     let is_out_of_bound_error =
                                //         matches!(err, PlaneError::OutOfBound2d(_, _));
                                //     if !is_out_of_bound_error {
                                //         return Err(err.into());
                                //     }
                                // }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn space_info(&self) -> &SpaceInfo {
        &self.space_info
    }

    fn space_info_mut(&mut self) -> &mut SpaceInfo {
        &mut self.space_info
    }
}
