pub mod core;

use crate::core::pokeball::*;
use crate::core::pokemon::*;

fn main() {
    //PART 1 : Salutations
    println!("--------------------------------------------");
    let mut world: String = String::new();
    world = "Kanto".to_string();

    println!("Hello, {} !", world);

    //PART 2 : Create some basic Pokemon
    println!("--------------------------------------------");
    // 25 Pikachu
    let pikachu: Pokemon = Pokemon {
        id: 25,
        name: "Pikachu".to_string(),
        poketypes: Vec::new(),
    };

    println!("Un {} sauvage apparaît", pikachu.name);

    //PART 3 : Create a function
    println!("--------------------------------------------");
    // 04 Salameche
    let salameche = Pokemon {
        id: 4,
        name: "Salameche".to_string(),
        poketypes: Vec::new(),
    };

    appears(&salameche);
    println!("Un {} sauvage apparaît", &salameche.name);

    //PART 4 : Poketypes, vecs, new and Traits show and CanBeNammed
    println!("--------------------------------------------");
    // 01 Bulbizarre
    let bulbizarre = &mut Pokemon::new(1, "Bulbizarre".to_string());
    bulbizarre.add_type("plante".to_string());
    bulbizarre.add_types(vec!["plante".to_string()]);
    bulbizarre.show();
    bulbizarre.who_am_i();
    show(bulbizarre);

    //PART 5 : Cleaning & Pokeball
    println!("--------------------------------------------");
    let pokeball = Pokeball::new("pokeball".to_string(), Some(bulbizarre));
    show(&pokeball);
    pokeball.who_am_i();
    match pokeball.get_pokemon() {
        Ok(pokemon) => pokemon.who_am_i(),
        Err(error_message) => println!("{}", error_message),
    }

    let masterball = Pokeball::new("masterball".to_string(), None);
    show(&masterball);
    masterball.who_am_i();
    match masterball.get_pokemon() {
        Ok(pokemon) => pokemon.who_am_i(),
        Err(error_message) => println!("{}", error_message),
    }
}

fn appears(pokemon: &Pokemon) {
    println!("Un {} sauvage apparaît", pokemon.name);
}

fn show<T: CanBeNammed + std::fmt::Debug>(struct_to_show: &T) {
    println!("{:#?}", struct_to_show);
}

trait CanBeNammed {
    fn get_my_name(&self) -> String;
    fn who_am_i(&self) {
        println!("Je suis un(e) {}", self.get_my_name());
    }
}
