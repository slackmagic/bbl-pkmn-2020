use crate::core::pokemon::Pokemon;
use crate::CanBeNammed;

#[derive(Debug)]
pub struct Pokeball<'a> {
    name: String,
    pokemon: Option<&'a Pokemon>,
}

impl<'a> Pokeball<'a> {
    pub fn new(name: String, pokemon: Option<&'a Pokemon>) -> Pokeball {
        Pokeball { name, pokemon }
    }

    pub fn get_pokemon(&self) -> Result<&'a Pokemon, String> {
        match self.pokemon {
            Some(pokemon_unwrapped) => Ok(pokemon_unwrapped),
            None => Err("Rien à l'interieur".to_string()),
        }
    }
}

impl<'a> CanBeNammed for Pokeball<'a> {
    fn get_my_name(&self) -> String {
        match &self.pokemon {
            Some(pokemon) => {
                format!("{} et je contiens un {:#?}", &self.name, &pokemon.name).to_string()
            }
            None => format!("{} et je suis vide", &self.name),
        }
    }
}
