mod map;

fn main() {
    let map = map::Room::new(64);
    println!("Map: [{}]", map);
}
