/**
Types	Literals

Signed integers    	        i8,     i16,    i32,    i64,            i128,       isize	    -10,        0,              1_000,      123_i64
Unsigned integers	        u8,     u16,    u32,    u64,            u128,       usize	    0,          123,            10_u16
Floating point numbers	    f32,    f64	                                                    3.14,       -10.0e20,       2_f32
Strings	                    &str	                                                        "foo",      "two\nlines"
Unicode scalar values	    char	                                                        'a',        'α',            '∞'
Booleans	                bool	                                                        true,       false

The types have widths as follows:

iN, uN, and fN are N bits wide,
isize and usize are the width of a pointer,
char is 32 bits wide,
bool is 8 bits wide.
*/

fn main() {
    // Program entry point
    let x: f32 = 2_f32; // Mutable variable binding
    println!("{x}"); // Macro for printing, like printf


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
            let mut separator: &str = if y != 0 {
                " "
            } else {
                ""
            };
            print!("{}[{}]", separator, matrix[i][y])
        }
        println!(")");
    }
}