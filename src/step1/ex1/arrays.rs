fn arrays() {
    let array = [10, 20, 30];
    println!("array: {array:?}");

    print!("Iterating over array:");
    for n in &array {
        print!(" {n}");
    }
    println!();

    print!("Iterating over range:");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transpose_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..matrix.len() {
        for y in 0..matrix[i].len() {
            transpose_matrix[y][i] = matrix[i][y];
        }
    }
    return transpose_matrix;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in 0..matrix.len() {
        print!("(");
        for y in 0..matrix[i].len() {
            let mut separator: &str = "";
            if y != 0 {
                separator = " ";
            }
            print!("{}[{}]", separator, matrix[i][y])
        }
        println!(")");
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}