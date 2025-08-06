use rand::Rng;
use svg::node::element::{Group, Path};
use svg::Document;

// 定义图像尺寸和迭代次数
const IMAGE_SIZE: (u32, u32) = (800, 400);
const NUM_ITERATIONS: u32 = 5;

// 定义颜色常量
const FACE_COLORS: [(f64, f64, f64); 5] = [
    (0.9, 0.9, 0.5),
    (0.5, 1.0, 0.9),
    (0.3, 0.5, 0.8),
    (0.4, 0.7, 0.2),
    (1.0, 0.5, 0.25),
];
const EDGE_COLOR: (f64, f64, f64) = (0.3, 0.3, 0.3);

// 使用结构体表示二维平面上的复数/点
#[derive(Clone, Copy, Debug)]
struct Complex {
    re: f64,
    im: f64,
}

// 为点的运算实现操作符重载
impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl std::ops::Div<f64> for Complex {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

// 定义三角形结构体，包含颜色索引和三个顶点
#[derive(Clone, Copy, Debug)]
struct Triangle {
    color_index: usize,
    a: Complex,
    b: Complex,
    c: Complex,
}

// 随机细分三角形的函数
fn subdivide(triangles: &[Triangle]) -> Vec<Triangle> {
    let mut result = Vec::new();
    let mut rng = rand::rng();

    for &triangle in triangles {
        let (a, b, c) = (triangle.a, triangle.b, triangle.c);
        let d = (a + b) / 2.0;
        let e = a * 0.6 + c * 0.4;
        let f = a * 0.2 + c * 0.8;
        let g = (f + b) / 2.0;

        // 为每个新生成的子三角形随机分配一个颜色
        result.extend_from_slice(&[
            Triangle { color_index: rng.random_range(0..FACE_COLORS.len()), a, b: e, c: d },
            Triangle { color_index: rng.random_range(0..FACE_COLORS.len()), a: f, b: e, c: d },
            Triangle { color_index: rng.random_range(0..FACE_COLORS.len()), a: d, b: g, c: f },
            Triangle { color_index: rng.random_range(0..FACE_COLORS.len()), a: d, b: g, c: b },
            Triangle { color_index: rng.random_range(0..FACE_COLORS.len()), a: b, b: f, c },
        ]);
    }
    result
}

pub fn generate_tilings() {
    // 定义初始的两个大三角形，它们共同构成一个矩形
    let mut triangles = vec![
        Triangle {
            color_index: 0,
            a: Complex { re: 0.0, im: 0.0 },
            b: Complex { re: 2.0, im: 0.0 },
            c: Complex { re: 2.0, im: 1.0 },
        },
        Triangle {
            color_index: 0,
            a: Complex { re: 2.0, im: 1.0 },
            b: Complex { re: 0.0, im: 1.0 },
            c: Complex { re: 0.0, im: 0.0 },
        },
    ];

    // `tiles` 用于存储每次迭代时的三角形，以便后续绘制不同粗细的轮廓
    let mut tiles = Vec::new();

    // 迭代生成更小的三角形
    for _ in 0..NUM_ITERATIONS {
        tiles.push(triangles.clone());
        triangles = subdivide(&triangles);
    }
    tiles.push(triangles); // 加入最后一次细分的结果

    // 计算基础线宽，相对于数学坐标系
    let base_lw = {
        let first_tri = tiles.last().unwrap()[0];
        ((first_tri.b.re - first_tri.a.re).powi(2) + (first_tri.b.im - first_tri.a.im).powi(2)).sqrt() / 20.0
    };

    // 创建一个 SVG <g> 元素来处理坐标变换
    // 这会将我们的数学坐标系 [0, 2] x [0, 1] 映射到 SVG 图像的 [0, 800] x [0, 400] 区域
    // 并翻转 Y 轴，使 (0,0) 位于左下角
    let mut group = Group::new().set(
        "transform",
        format!(
            "translate(0, {}) scale({}, -{})",
            IMAGE_SIZE.1,
            IMAGE_SIZE.0 as f64 / 2.0,
            IMAGE_SIZE.1 as f64
        ),
    );

    // RGB元组转换为SVG颜色字符串的辅助函数
    let to_rgb_string = |color: (f64, f64, f64)| -> String {
        format!(
            "rgb({},{},{})",
            (color.0 * 255.0) as u8,
            (color.1 * 255.0) as u8,
            (color.2 * 255.0) as u8
        )
    };

    let edge_color_str = to_rgb_string(EDGE_COLOR);

    // 1. 绘制最精细的、带填充色的三角形
    if let Some(final_triangles) = tiles.last() {
        for triangle in final_triangles {
            let path_data = format!(
                "M {} {} L {} {} L {} {} Z",
                triangle.a.re, triangle.a.im, triangle.b.re, triangle.b.im, triangle.c.re, triangle.c.im
            );

            let fill_color_str = to_rgb_string(FACE_COLORS[triangle.color_index]);

            let path = Path::new()
                .set("d", path_data)
                .set("fill", fill_color_str)
                .set("stroke", edge_color_str.clone())
                .set("stroke-width", base_lw)
                .set("stroke-linejoin", "round");
            group = group.add(path);
        }
    }

    // 2. 绘制之前迭代中较粗的轮廓线
    for (k, triangles_level) in tiles.iter().take(NUM_ITERATIONS as usize).enumerate() {
        let lw = base_lw * 2.0 * (k as f64 + 1.0) / NUM_ITERATIONS as f64;
        for triangle in triangles_level {
            let path_data = format!(
                "M {} {} L {} {} L {} {} Z",
                triangle.a.re, triangle.a.im, triangle.b.re, triangle.b.im, triangle.c.re, triangle.c.im
            );

            let path = Path::new()
                .set("d", path_data)
                .set("fill", "none") // 轮廓没有填充色
                .set("stroke", edge_color_str.clone())
                .set("stroke-width", lw)
                .set("stroke-linejoin", "round");
            group = group.add(path);
        }
    }

    // 创建SVG文档并保存到文件
    let document = Document::new()
        .set("viewBox", (0, 0, IMAGE_SIZE.0, IMAGE_SIZE.1))
        .add(group);

    svg::save("pinwheel_random_svg.svg", &document).unwrap();
    println!("成功生成 'pinwheel_random_svg.svg'");
}