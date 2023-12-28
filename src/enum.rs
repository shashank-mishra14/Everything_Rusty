
enum IpAddrKind {
    V4(String),
    V6(String),
    
}
struct message {
    message: String,
    sender: String,
    receiver: String,
    read: bool,
}

impl message {
    fn new(message: String, sender: String, receiver: String) -> message {
        message {
            message,
            sender,
            receiver,
            read: false,
        };
        print!("{} sent {} to {}", sender, message, receiver);
    }
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main(){
    let IPAdd= IpAddrKind::V4(String::from("124.0.0.1"));
}