use csscolorparser::Color;
use plotters::prelude::*;
use rand::Rng;

fn hex2rgb(hex_color: &str) -> RGBColor {
    // 解析十六进制颜色
    let color = hex_color.parse::<Color>().unwrap();
    let rgba = color.to_rgba8();
    // 将解析出的 RGB 值用于 plotters 的颜色结构
    RGBColor(rgba[0], rgba[1], rgba[2])
}
pub fn generate(
    dimension: usize,
    num_lines: i32,
    output_width: u32,
    output_height: u32,
    fat_color: String,
    thin_color: String,
    edge_color: String,
    output_filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Generate random shifts for each grid direction
    let mut rng = rand::rng();
    let shifts: Vec<f64> = (0..dimension).map(|_| rng.random::<f64>()).collect();
    println!("Shifts: {:?}", shifts);

    // Compute angles and unit vectors for grid directions
    let theta: Vec<f64> = if dimension % 2 == 0 {
        (0..dimension)
            .map(|i| i as f64 * std::f64::consts::PI / dimension as f64)
            .collect()
    } else {
        (0..dimension)
            .map(|i| i as f64 * 2.0 * std::f64::consts::PI / dimension as f64)
            .collect()
    };

    let uv: Vec<(f64, f64)> = theta.iter().map(|&t| (t.cos(), t.sin())).collect();

    // Set up the drawing area
    let root = SVGBackend::new(output_filename, (output_width, output_height)).into_drawing_area();
    root.fill(&WHITE)?;
    let (x_min, x_max, y_min, y_max) = (-16.0, 16.0, -12.0, 12.0);
    let mut chart = ChartBuilder::on(&root).build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    // Define colors
    let fat_color = hex2rgb(&fat_color);
    let thin_color = hex2rgb(&thin_color);
    let edge_color = hex2rgb(&edge_color);

    // Iterate over all unique pairs of grids (r, s)
    for r in 0..dimension {
        for s in (r + 1)..dimension {
            for kr in -num_lines..=num_lines {
                for ks in -num_lines..=num_lines {
                    // Solve for intersection point of grid lines (r, kr) and (s, ks)
                    let det = uv[r].0 * uv[s].1 - uv[r].1 * uv[s].0;
                    if det.abs() < 1e-12 {
                        continue; // Skip if lines are parallel
                    }
                    let b0 = kr as f64 - shifts[r];
                    let b1 = ks as f64 - shifts[s];
                    let x = (b0 * uv[s].1 - b1 * uv[r].1) / det;
                    let y = (b1 * uv[r].0 - b0 * uv[s].0) / det;

                    // Compute index vector for the intersection point
                    let mut index: Vec<i32> = uv
                        .iter()
                        .enumerate()
                        .map(|(i, &(ux, uy))| (ux * x + uy * y + shifts[i]).ceil() as i32)
                        .collect();

                    // Generate vertices for the rhombus
                    let mut vertices = Vec::new();
                    for (dr, ds) in &[(0, 0), (1, 0), (1, 1), (0, 1)] {
                        index[r] = kr + dr;
                        index[s] = ks + ds;
                        let (mut px, mut py) = (0.0, 0.0);
                        for (i, &(ux, uy)) in uv.iter().enumerate() {
                            px += index[i] as f64 * ux;
                            py += index[i] as f64 * uy;
                        }
                        vertices.push((px, py));
                    }

                    // Determine rhombus type and color
                    let is_adjacent = s - r == 1 || s - r == dimension - 1;
                    let fill_color = if is_adjacent { fat_color } else { thin_color };

                    // Draw filled polygon
                    chart.draw_series(std::iter::once(Polygon::new(
                        vertices.clone(),
                        ShapeStyle::from(&fill_color).filled(),
                    )))?;

                    // Draw border (closed path)
                    let mut border = vertices.clone();
                    border.push(vertices[0]);
                    chart.draw_series(std::iter::once(PathElement::new(
                        border,
                        ShapeStyle::from(&edge_color).stroke_width(1),
                    )))?;
                }
            }
        }
    }

    Ok(())
}
