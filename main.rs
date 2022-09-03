// Data types rust lang -->https://doc.rust-lang.org/std/

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("{}",THREE_HOURS_IN_SECONDS)
//     }

// fn main() {
//     let x: i8 =3;
//     let y:i8=-3;
//     println!("{}",x+y)
//     }

// fn main() {
//     // addition
//     let sum:i8 = 5 + 10;
//     println!("{}",sum);
//     // subtraction
//     let difference:f64 = 95.5 - 4.3;
//     println!("{}",difference);
//     // multiplication
//     let product:i8 = 4 * 30;
//     println!("{}",product);
//     // division
//     let quotient:f64 = 56.7 / 32.2;
//     println!("{}",quotient);
//     let floored:f64 = 2.0 / 3.0; // Results in 0
//     println!("{}",floored);
//     // remainder
//     let remainder:i8 = 43 % 5;
//     println!("{}",remainder);
// }

// fn main() {
//     for elem in 1..6 {
//         if elem %2 == 0 {
//             println!("juft:{elem}");
//             continue;
//         }
//         println!("toq:{elem}")
//     }
    // let t = true;

    // let f: bool = false; // with explicit type annotation
    // println!("{}",t&f);
// }


// charakter


// fn main(){
//     let b = "banana";
//     let a = "apple";
//     let heart_eyed_cat = '😻';

//     println!("{a} , {b} , {heart_eyed_cat}")
// }

// Tuple

// fn main(){
//     let tup:(i32,f64,u8) = (45,7.9,5);
//     println!("{}",tup.1)
// }

// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {}", y);
// }

// elem tup

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;
// }


// massiv

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     println!("{}", a);
//     for elem in a{
//         println!("{elem}");
//     }
    
// }


// io

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim().parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}