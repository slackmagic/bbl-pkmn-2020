# RUST BBL 2020
## Pokemon references:

- 001 - Bulbizarre 
- 004 - Salameche
- 025 - Pikachu
- 098 - Krabby


## Part.1 : Salutations from Kanto

- let
- String
- mut


## Part.2 : Create some basic Pokemon

- Create a Pikachu
- Create a **Pokemon** struct with id, name
- Println

## Part.3 : Create a function

- Create a Salameche
- Create a **Appears** method
- Println
- Borrow checker => &


## Part.4 : Cleaning project

- Create **Core** folder
- Create a **pokemon** file
- Change the visibility of Pokemon attributes

## Part.5 : Create Poketype and a constructor

- Create a **Poketype** struct with name
- Create 
- Update **Pokemon** :
    - Add a Vec<Poketype> attributes
    - Create function **add_type**
    - Create function **add_type**
    - Create a **new** constructor


## Part.7 : Traits CanBeNammed, #[derive(Debug)]

- Create a **CanBeNammed** trait with func:
    - **get_my_name**
    - **who_am_i**
- Impl for **Pokemon** and **Poketype**

## Part.7 : Auto traits Debug and Generic

- Add implicit trait **#[derive(Debug)]** for **Pokemon** and **Poketype**
- Create a *show* method with sign :
    - ```fn show<T: CanBeNammed + std::fmt::Debug>(struct_to_show: &T)``` 

## Part.8 :  Create Pokeball and Option
