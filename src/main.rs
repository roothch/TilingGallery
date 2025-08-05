use svg::node::element::{Path, Rectangle};
use svg::Document;
use rand::Rng;

// 定义二维向量
#[derive(Clone, Copy, Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // 向量加法
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 定义菱形瓦片
#[derive(Debug)]
struct Rhombus {
    vertices: [Vec2; 4],
    is_thick: bool,
}

fn main() {
    let width = 800.0;
    let height = 800.0;
    let num_lines = 20; // 每个方向的线条数量

    let rhombuses = generate_penrose_tiling(width, height, num_lines);
    draw_svg(width, height, &rhombuses);
}

// 生成彭罗斯镶嵌
fn generate_penrose_tiling(width: f64, height: f64, num_lines: i32) -> Vec<Rhombus> {
    let mut rng = rand::thread_rng();
    let mut rhombuses = Vec::new();

    // 五个方向的单位向量
    let mut directions = [Vec2::new(0.0, 0.0); 5];
    for i in 0..5 {
        let angle = (i as f64) * 2.0 * std::f64::consts::PI / 5.0;
        directions[i] = Vec2::new(angle.cos(), angle.sin());
    }

    // 随机偏移量
    let mut offsets = [0.0; 5];
    for i in 0..5 {
        offsets[i] = rng.gen::<f64>() * 10.0;
    }

    // 遍历所有线对的交点
    for i in 0..5 {
        for j in (i + 1)..5 {
            let dir1 = directions[i];
            let dir2 = directions[j];

            for k in -num_lines..num_lines {
                for l in -num_lines..num_lines {
                    // 计算两条线的交点
                    let p1 = k as f64 + offsets[i];
                    let p2 = l as f64 + offsets[j];

                    let det = dir1.x * dir2.y - dir1.y * dir2.x;
                    if det.abs() < 1e-6 { continue; } // 平行线

                    let x = (p1 * dir2.y - p2 * dir1.y) / det;
                    let y = (p2 * dir1.x - p1 * dir2.x) / det;
                    let intersection = Vec2::new(x, y);

                    // 确定菱形类型和顶点
                    let mut k_indices = [0; 5];
                    for m in 0..5 {
                        k_indices[m] = (intersection.x * directions[m].x + intersection.y * directions[m].y - offsets[m]).floor() as i32;
                    }

                    let mut sum = 0;
                    for &k_val in k_indices.iter() {
                        sum += k_val;
                    }
                    if sum != 0 { continue; }

                    let v1 = get_vertex(&k_indices, &directions);

                    let mut k2 = k_indices; k2[i] += 1;
                    let v2 = get_vertex(&k2, &directions);

                    let mut k3 = k_indices; k3[j] += 1;
                    let v3 = get_vertex(&k3, &directions);

                    let mut k4 = k_indices; k4[i] += 1; k4[j] += 1;
                    let v4 = get_vertex(&k4, &directions);

                    // 胖瘦菱形的角度判断
                    let angle = ((v2.x-v1.x)*(v3.x-v1.x) + (v2.y-v1.y)*(v3.y-v1.y)).acos() * 180.0 / std::f64::consts::PI;
                    let is_thick = angle.abs() > 40.0 && angle.abs() < 80.0;

                    rhombuses.push(Rhombus {
                        vertices: [v1, v2, v4, v3],
                        is_thick,
                    });
                }
            }
        }
    }
    rhombuses
}


fn get_vertex(k: &[i32; 5], directions: &[Vec2; 5]) -> Vec2 {
    let mut vertex = Vec2::new(0.0, 0.0);
    for m in 0..5 {
        vertex = vertex.add(Vec2::new(
            directions[m].x * k[m] as f64,
            directions[m].y * k[m] as f64,
        ));
    }
    vertex
}


// 绘制 SVG 图像
fn draw_svg(width: f64, height: f64, rhombuses: &[Rhombus]) {
    let mut document = Document::new()
        .set("width", width)
        .set("height", height)
        .add(Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "#f0f0f0"));

    let scale = 20.0; // 缩放因子
    let (tx, ty) = (width / 2.0, height / 2.0); // 平移

    for rhombus in rhombuses {
        let mut data = svg::node::element::path::Data::new()
            .move_to((rhombus.vertices[0].x * scale + tx, rhombus.vertices[0].y * scale + ty));

        for i in 1..4 {
            data = data.line_to((rhombus.vertices[i].x * scale + tx, rhombus.vertices[i].y * scale + ty));
        }
        data = data.close();

        // 根据菱形类型设置不同颜色
        let color = if rhombus.is_thick {
            "lightblue"
        } else {
            "lightcoral"
        };

        let path = Path::new()
            .set("fill", color)
            .set("stroke", "black")
            .set("stroke-width", 0.5)
            .set("d", data);

        document = document.add(path);
    }

    svg::save("penrose_tiling.svg", &document).unwrap();
    println!("Generated penrose_tiling.svg");
}
