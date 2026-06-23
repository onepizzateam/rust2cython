pub struct Point { pub x: f64, pub y: f64 }
pub struct Color { pub r: u8, pub g: u8, pub b: u8 }
pub fn distance(p: Point) -> f64 { (p.x * p.x + p.y * p.y).sqrt() }
pub fn blend(a: Color, b: Color) -> Color { Color { r: (a.r/2 + b.r/2), g: (a.g/2 + b.g/2), b: (a.b/2 + b.b/2) } }
