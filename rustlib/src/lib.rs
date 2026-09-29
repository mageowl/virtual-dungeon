use std::io;

pub use common::Direction;
use common::{Request, Tile};

fn send(req: Request) {
    println!("\0{req}")
}

fn get_response() -> String {
    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();
    response
}

fn wait_for_nl() {
    let response = get_response();
    if response != "done" {
        panic!("failed to finish action")
    }
}

pub fn do_move(dir: Direction) {
    send(Request::Move(dir));
    wait_for_nl();
}

pub fn do_attack(dir: Direction) {
    send(Request::Attack(dir));
    wait_for_nl();
}

pub fn do_scan(x: i8, y: i8) -> Tile {
    send(Request::Scan(x, y));
    Tile::try_from(get_response().as_str()).expect("invalid tile")
}
