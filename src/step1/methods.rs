struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(sideSize: u32) -> Rectangle {
        Rectangle {
            width: sideSize,
            height: sideSize,
        }
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 10,
        height: 5,
    };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
    let mut rect2 = Rectangle::new(10, 6);
    println!("rect2 area: {}", rect2.area());
    let mut rect3 = Rectangle::square(10);
    println!("rect3 area: {}", rect3.area());
}
