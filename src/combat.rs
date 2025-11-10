use crate::pokemon::Pokemon;

pub fn combat_simple(mut joueur_pokemon: Box<dyn Pokemon>, mut sauvage_pokemon: Box<dyn Pokemon>){
    println!("=== DÉBUT DU COMBAT ===");
    println!("Votre Pokémon vs Pokémon Sauvage\n");

    let mut tour = 1;
    while joueur_pokemon.est_vivant() && sauvage_pokemon.est_vivant(){
        println!("--- TOUR {} ---", tour);

        let degats = joueur_pokemon.attaquer();
        sauvage_pokemon.prendre_degats(degats);
        println!("{} attaque ! {} dégâts !", joueur_pokemon.get_nom(), degats);
        println!("{} perds {} de PV. PV restant {}", sauvage_pokemon.get_nom(), degats, sauvage_pokemon.get_pv());

        if !sauvage_pokemon.est_vivant(){
            println!("Le Pokémon sauvage est vaincu !");
            break;
        }

        let degats = sauvage_pokemon.attaquer();
        joueur_pokemon.prendre_degats(degats);
        println!("{} sauvage attaque ! {} degats", sauvage_pokemon.get_nom(), degats);
        println!("{} perds {} de PV. PV restant {}", joueur_pokemon.get_nom(), degats, joueur_pokemon.get_pv());

        if !joueur_pokemon.est_vivant(){
            println!("Votre Pokémon est KO"); 
            break;
        }
        tour += 1;
    }
}