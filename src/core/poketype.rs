use crate::CanBeNammed;

#[derive(Debug)]
pub struct Poketype {
    pub name: String,
}

impl Poketype {
    pub fn new(name: String) -> Poketype {
        Poketype { name }
    }
}

impl CanBeNammed for Poketype {
    fn get_my_name(&self) -> String {
        self.name.to_string()
    }
}
