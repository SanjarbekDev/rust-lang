// fn main() {
//     // println!("Hello, world!");

//     let x:String = String::from("hello world");
//     // let bytes:&[u8] = x.as_bytes();

//     // for i in bytes {
//     //     println!("{}",i)
//     // }

// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i; // byte turida tekshiryabdi va mos indexni qaytaryabdi
//         }
//     }

//     s.len()
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }
fn main() {
    // let mut s = String::from("hello world");

    // let word = first_word(&s); // word will get the value 5

    // print!("{}",word);

    // let hello = &s[..5];
    // let world = &s[5..];

    // print!("{}{}",hello,world);

    // s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // let array: [u32;5] = [1,2,3,4,5];
    // let slice = &array[1..3];

    // assert_eq!(slice, &[2, 3]);

    let a = 3;
    let b = 1 + 2;
    assert_eq!(a, b);

    assert_eq!(a, b, "we are testing addition with {} and {}", a, b);

    print!("{}",a==b);

    // for i in slice{
    //     print!("{}\n",i);
    // }
}



