pub struct Person{
    firstName : String,
    age : u32,
    number : String
}

struct User{
    email: String,
    username: String,
    active: bool,
    sign_count: u32
}

fn build_user(email :String,username: String)->User{
    User { 
        email: email, 
        username: username, 
        active: true, 
        sign_count: 1 
    }
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

struct AlwaysEqual;

fn getArea(with: f32 , hight: f32)->f32{
    return with*hight;
}

fn main() {

    // let user = Person{
    //     firstName:String::from("Sanjar"),
    //     age:22,
    //     number:String::from("998913374277")
    // };
    // println!("Mening ismim {}, yoshim {} da.",user.firstName,user.age);


    // mut user1 all structure mutable
    // let mut user1 = User{
    //     email: String::from("Sannjarbeksodiqov0302@gmail.com"),
    //     username: String::from("http_master"),
    //     active: true,
    //     sign_count: 1
    // };

    // user1.email=String::from("softpro937@gmail.com");// mutable user1 ,edited email
    // user1.sign_count=2;

    // print!("my email: {}, username: {}, signed: {}",user1.email,user1.username,user1.sign_count)

    // let user2 = build_user(String::from("softpro937@gmail.com"),String::from("http_master"));

    // let user3 : User = User{ 
    //     email: String::from("httpmaster@gmail.com"),
    //     ..user2
    //     };

    // print!("{}\n,{}\n,{}\n,{}",
    //     user3.email,
    //     user3.active,
    //     user3.sign_count,
    //     user3.username
    // );

    // let Black = Color(0,0,0);
    // let origin = Point(0,0,0);

    // let subject = AlwaysEqual;

    let with: f32 = 22.3;
    let hight: f32 = 11.4;

    println!("{}",
    getArea(with, hight)
)
    


}
