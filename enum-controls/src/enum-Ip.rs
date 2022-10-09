#[derive(Debug)]
enum IPAddrKind{
    V4,
    V6
}

struct IPAddr{
    kind : IPAddrKind,
    addres : String
}

fn main(){
    // let four = IPAddrKind::V4;
    // let six = IPAddrKind::V6;

    let home = IPAddr{
        kind: IPAddrKind::V4,
        addres: String::from("127.0.0.1")
    };

    let loopback = IPAddr{
        kind: IPAddrKind::V6,
        addres: String::from("::1")
    };

    println!("yor ip {},kind : {:?}",home.addres,home.kind)
}