fn main() {
    println!("Enums are great!");
    
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
    
    
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrKind {
    V4,
    V6,
}