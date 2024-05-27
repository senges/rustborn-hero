use core::fmt;

type Object = String;
type Texture = String;

#[derive(Clone)]
enum TileKind {
    Floor,
    Wall,
    Door,
    Object(Object),
}

#[derive(Clone)]
struct Tile {
    kind: TileKind,
    texture: Texture,
}

impl Tile {
    fn new(kind: TileKind) -> Tile {
        Tile { kind: kind, texture: "default".to_string()}
    }
}

pub struct Room {
    map: Vec<Tile>,
    size: usize,
}

impl Room {
    pub fn new(size: usize) -> Room {
        Room { map: vec![Tile::new(TileKind::Floor); size], size: size }
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ascii_map = String::from("");
        for tile in self.map.iter() {
            match tile.kind {
                TileKind::Floor => ascii_map.push('#'),
                _ => ascii_map.push(' ')
            }
        }
        write!(f, "{}", ascii_map)
    }
}
