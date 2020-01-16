use crate::core::poketype::*;
use crate::CanBeNammed;

#[derive(Debug)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub poketypes: Vec<Poketype>,
}

impl Pokemon {
    pub fn new(id: u32, name: String) -> Pokemon {
        Pokemon {
            id,
            name,
            poketypes: Vec::new(),
        }
    }

    pub fn add_type(&mut self, str_type: String) {
        self.poketypes.push(Poketype::new(str_type));
    }

    pub fn add_types(&mut self, vec_types: Vec<String>) {
        self.poketypes.clear();
        for str_type in vec_types {
            self.poketypes.push(Poketype::new(str_type));
        }
    }

    pub fn show(&self) {
        println!("{:#?}", &self);
    }
}

impl CanBeNammed for Pokemon {
    fn get_my_name(&self) -> &String {
        &self.name
    }
}
