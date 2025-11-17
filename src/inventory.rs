use crate::pokemon::Pokemon;

pub struct Inventory {
    pub pokemons: Vec<Box<dyn Pokemon>>,
    pub current_pokemon_index: usize,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            pokemons: Vec::new(),
            current_pokemon_index: 0,
        }
    }
    /*Permet d'ajouter un pokémon dans son inventaire */
    pub fn add_pokemon(&mut self, pokemon: Box<dyn Pokemon>) {
        self.pokemons.push(pokemon);
    }
}
