pub struct WorldMap {
    map: [u16; 8],
}

impl WorldMap {
    pub const fn new(map: [u16; 8]) -> Self {
        Self { map }
    }

    pub fn is_wall(&self, x: f32, y: f32) -> bool {
        match self.map.get(y as usize) {
            Some(line) => (line & (0b1 << x as usize)) != 0,
            None => true,
        }
    }
}