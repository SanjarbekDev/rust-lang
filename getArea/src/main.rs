#[derive(Debug)]
struct Rectangle{
    witdh: u32,
    hight: u32
}

// fn getArea(rectangle: &Rectangle)->u32{
//     return rectangle.witdh * rectangle.hight;
// }

// fn area(rectangle: (u32,u32))->u32{
//     rectangle.0*rectangle.1
// }

fn main() {
    let rect1 = Rectangle{
        witdh: 22,
        hight: 11
    };

    // println!("{}",
    //     getArea(&rect1)
    // );

    // let rect2: (u32,u32) = (4,44);

    // print!("{}",
    //     area(rect2)
    // );

    println!("{:#?}",rect1)
}
