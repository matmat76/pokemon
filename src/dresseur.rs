use crate::pokemon::Pokemon;

pub struct Dresseur{
    nom: String,
    equipe: Vec<Box<dyn Pokemon>>,
    pokeballs: i32,
    potions: i32,
}

impl Dresseur{
    pub fn new(nom: String) -> Dresseur {
        Dresseur {
            nom,
            equipe: vec![],
            pokeballs: 10,
            potions: 5,
        }
    }

    pub fn ajouter_pokemon(&mut self, poke: Box<dyn Pokemon>){
        self.equipe.push(poke);
    }

    pub fn afficher_pokedex(&self){
        println!("=== Pokédex de {} ===", self.nom);
        for (index, pokemon) in self.equipe.iter().enumerate(){
            println!("{}. ", index + 1);
            pokemon.afficher_infos();
        }
        println!("Pokéballs: {} | Potions: {}", self.pokeballs, self.potions);
    }

    pub fn recuperer_pokemon(&mut self, index: usize) -> Option<Box<dyn Pokemon>>{
        if index < self.equipe.len(){
            Some(self.equipe.remove(index))
        }else{
            None
        }
    }
}