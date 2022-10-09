// #[derive(Debug)]
// enum IPAddr{
//     V4(u8, u8, u8, u8),
//     V6(String)
// }

#[derive(Debug)]
struct IpAddrv4{
    addres: String
}
#[derive(Debug)]
struct IpAddrv6{
    addres: String
}

#[derive(Debug)]
enum IPAddrKind{
    V4(IpAddrv4),
    V6(IpAddrv6),
}

fn main(){
    let home = IPAddrKind::V4(IpAddrv4{
            addres: String::from("127.0.0.1")
        });
    // let loopback = IPAddr::V6(String::from("::1"));

    println!("your ip : {:?}",home)
}