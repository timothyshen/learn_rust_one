
// 修复错误，让代码工作
struct Point<T> {
    x: T,
    y: T,
}

impl Point {
    fn distance_from_origin(&self) -> T {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point{x: 5, y: 10};
    println!("{}",p.distance_from_origin())
}