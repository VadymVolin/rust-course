///Compound Types
///Types	Literals
///Arrays	[T; N]	    [20, 30, 40],       [0; 3]
///Tuples	(),         (T,),               (T1, T2), …	(),     ('x',),     ('x', 1.2)

fn main() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {:?}", a);

    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
}
