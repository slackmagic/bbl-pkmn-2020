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
}

fn appears(pokemon: &Pokemon) {
    println!("Un {} sauvage apparaît", pokemon.name);
}

struct Pokeball {
    name: String,
    pokemon: Pokemon,
}

#[derive(Debug)]
struct Pokemon {
    id: u32,
    name: String,
    poketypes: Vec<Poketype>,
}

#[derive(Debug)]
struct Poketype {
    name: String,
}

fn show<T: CanBeNammed + std::fmt::Debug>(struct_to_show: &T) {
    println!("{:#?}", struct_to_show);
}

trait CanBeNammed {
    fn get_my_name(&self) -> &String;
    fn who_am_i(&self) {
        println!("Je suis un(e) {}", self.get_my_name());
    }
}

impl Pokemon {
    fn new(id: u32, name: String) -> Pokemon {
        Pokemon {
            id,
            name,
            poketypes: Vec::new(),
        }
    }

    fn add_type(&mut self, str_type: String) {
        self.poketypes.push(Poketype::new(str_type));
    }

    fn add_types(&mut self, vec_types: Vec<String>) {
        self.poketypes.clear();
        for str_type in vec_types {
            self.poketypes.push(Poketype::new(str_type));
        }
    }

    fn show(&self) {
        println!("{:#?}", &self);
    }
}

impl CanBeNammed for Pokemon {
    fn get_my_name(&self) -> &String {
        &self.name
    }
}

impl Poketype {
    fn new(name: String) -> Poketype {
        Poketype { name }
    }
}
