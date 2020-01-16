#[derive(Debug)]
pub struct Poketype {
    pub name: String,
}

impl Poketype {
    pub fn new(name: String) -> Poketype {
        Poketype { name }
    }
}
