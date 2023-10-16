use anyhow::Result;
use itertools::Itertools;
use rusvid_core::pixel::Pixel;
use rusvid_core::plane::Plane;

#[allow(unused)]
pub fn render_square(
    plane: &mut Plane,
    pos: (i64, i64),
    size: (u32, u32),
    color: Pixel,
) -> Result<()> {
    // normalize the 'coordinates'

    let from_x = (pos.0).max(0);
    let to_x = (pos.0 + size.0 as i64).min(plane.height() as i64);

    let from_y = (pos.1).max(0);
    let to_y = (pos.1 + size.1 as i64).min(plane.width() as i64);

    for x in from_x..to_x {
        for y in from_y..to_y {
            plane.put_pixel(x as u32, y as u32, color)?;
        }
    }

    Ok(())
}

#[allow(unused)]
pub fn draw_circle(plane: &mut Plane, pos: (u32, u32), color: Pixel, radius: u32) -> Result<()> {
    let radius = radius as i32;
    let f_radius = radius as f32;

    let width = plane.width();
    let height = plane.height();

    for (x, y) in ((radius * -1)..radius)
        .cartesian_product((radius * -1)..radius)
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

#[allow(unused)]
pub fn draw_line(
    plane: &mut Plane,
    pos1: (i64, i64),
    pos2: (i64, i64),
    color: Pixel,
    stroke_weight: u32,
) -> Result<()> {
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
