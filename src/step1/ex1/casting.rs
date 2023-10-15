fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x, y));
}


fn multiply2(x: i16, y: i16) -> i16 {
    x * y
}

fn main2() {
    let x: i8 = 15;
    let y: i32 = 1000;

    println!("{x} * {y} = {}", multiply2(x.into(), i32::to_i16(y)));
}
