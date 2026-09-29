use std::io;

fn send_request(request: &str) -> String {
    println!("\0{request}");
    let mut response = String::new();
    io::stdin()
}
