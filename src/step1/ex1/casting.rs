fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x, y));
}

fn main2() {
    let x: i8 = 15;
    let y: i32 = 10000;

    let modified_y: i16 = i16::try_from(y).unwrap_or(0);
    let modified_x: i16 = x.into();
    let result: i16 = i16::try_from(i32::from(modified_x) * i32::from(modified_y)).unwrap_or(i16::MAX);
    println!("{}", result);
}

fn main3() {
    let x: i8 = 15;
    let y: i32 = 10000;
    let result: i16 = i16::try_from(i32::from(x) * y).unwrap_or(i16::MAX);
    println!("{}", result);
}
