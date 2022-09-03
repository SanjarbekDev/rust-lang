#[derive(Debug)]
struct Rectangle{
    witdh: u32,
    hight: u32
}

impl Rectangle {
    fn are(&self)->u32{
        &self.witdh * &self.hight
    }

    fn witdh(&self)->bool{
        self.witdh > 0
    }
}

fn main() {

    let rect1 = Rectangle{
        witdh: 11,
        hight: 22
    };
    println!("To'rtburchak yuzi : {}\n{}",
        rect1.are(),rect1.witdh()
    );
}
