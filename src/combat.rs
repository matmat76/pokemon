use crate::pokemon::Pokemon;
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionCombat {
    Attaquer,
    Pokeball,
    Potion,
    Fuir,
    Aucune,
}

pub struct CombatState {
    pub pokemon_joueur: Option<Box<dyn Pokemon>>,
    pub pokemon_sauvage: Option<Box<dyn Pokemon>>,
    pub action_selectionnee: ActionCombat,
    pub message_combat: String,
    pub tour_joueur: bool,
    pub combat_timer: f32,
}

impl CombatState {
    pub fn new(pokemon_joueur: Box<dyn Pokemon>, pokemon_sauvage: Box<dyn Pokemon>) -> Self {
        CombatState {
            pokemon_joueur: Some(pokemon_joueur),
            pokemon_sauvage: Some(pokemon_sauvage),
            action_selectionnee: ActionCombat::Aucune,
            message_combat: "C'est votre tour! Choisissez une action".to_string(),
            tour_joueur: true,
            combat_timer: 0.0,
        }
    }

    pub fn joueur_attaque(&mut self) {
        match (&self.pokemon_joueur, &self.pokemon_sauvage) {
            (Some(joueur), Some(_sauvage)) => {
                let degats = joueur.attaquer();
                let sauvage_nom: String = self.pokemon_sauvage.as_ref().unwrap().get_nom().clone();
                let sauvage_pv = self.pokemon_sauvage.as_ref().unwrap().get_pv() - degats;
                
                self.message_combat = format!(
                    "{} attaque! {} points de dégâts!\n{} a {} PV restants",
                    joueur.get_nom(),
                    degats,
                    sauvage_nom,
                    sauvage_pv
                );
                match &mut self.pokemon_sauvage {
                    Some(poke_sauvage) => poke_sauvage.prendre_degats(degats),
                    None => {},
                }
                self.combat_timer = 0.0;
                self.tour_joueur = false;
            },
            _ => {},
        }
    }

    pub fn sauvage_attaque(&mut self) {
        match (&self.pokemon_joueur, &self.pokemon_sauvage) {
            (Some(_joueur), Some(sauvage)) => {
                let degats = sauvage.attaquer();
                let joueur_nom = self.pokemon_joueur.as_ref().unwrap().get_nom().clone();
                let joueur_pv = self.pokemon_joueur.as_ref().unwrap().get_pv() - degats;
                
                self.message_combat = format!(
                    "{} sauvage attaque! {} points de dégâts!\n{} a {} PV restants",
                    sauvage.get_nom(),
                    degats,
                    joueur_nom,
                    joueur_pv
                );
                match &mut self.pokemon_joueur {
                    Some(poke_joueur) => poke_joueur.prendre_degats(degats),
                    None => {},
                }
                self.combat_timer = 0.0;
                self.tour_joueur = true;
            },
            _ => {},
        }
    }

    pub fn est_termine(&self) -> bool {
        match (&self.pokemon_joueur, &self.pokemon_sauvage) {
            (Some(j), Some(s)) => !j.est_vivant() || !s.est_vivant(),
            _ => true,
        }
    }

    pub fn joueur_utilise_potion(&mut self) -> bool {
        match &mut self.pokemon_joueur {
            Some(joueur) => {
                let max_pv = joueur.get_pv_max();
                let pv_actuel = joueur.get_pv();
                
                if pv_actuel >= max_pv {
                    self.message_combat = "Votre Pokémon est déjà en pleine santé!".to_string();
                    return false;
                }
                
                // Guérir de 30 PV (ou jusqu'au max)
                let guerison = 30;
                let nouveau_pv = (pv_actuel + guerison).min(max_pv);
                joueur.set_pv(nouveau_pv);
                
                self.message_combat = format!(
                    "Vous avez utilisé une Potion!\n{} retrouve {} PV!",
                    joueur.get_nom(),
                    nouveau_pv - pv_actuel
                );
                
                self.combat_timer = 0.0;
                self.tour_joueur = false;
                true
            },
            None => false,
        }
    }

    pub fn get_resultat(&self) -> String {
        match (&self.pokemon_joueur, &self.pokemon_sauvage) {
            (Some(j), Some(s)) => {
                if !j.est_vivant() {
                    "Défaite! Votre Pokémon est KO!".to_string()
                } else if !s.est_vivant() {
                    "Victoire! Pokémon sauvage vaincu!".to_string()
                } else {
                    "Combat en cours...".to_string()
                }
            }
            _ => "Erreur".to_string(),
        }
    }
}

pub fn dessiner_interface_combat(combat: &CombatState) {
    // Fond semi-transparent noir
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.7));

    // Fenêtre de combat (centré à l'écran)
    let window_width = 600.0;
    let window_height = 400.0;
    let x = (screen_width() - window_width) / 2.0;
    let y = (screen_height() - window_height) / 2.0;

    // Fond de la fenêtre (bleu foncé)
    draw_rectangle(x, y, window_width, window_height, Color::new(0.1, 0.3, 0.6, 1.0));
    
    // Bordure blanche
    draw_rectangle_lines(x, y, window_width, window_height, 3.0, WHITE);

    // === AFFICHAGE DES POKÉMON ===
    // Pokémon du joueur (en bas à gauche)
    match &combat.pokemon_joueur {
        Some(joueur) => {
            let joueur_y = y + window_height - 120.0;
            draw_text(&format!("{}", joueur.get_nom()), x + 20.0, joueur_y, 20.0, WHITE);
            let pv_text = format!("PV: {}", joueur.get_pv());
            draw_text(&pv_text, x + 20.0, joueur_y + 25.0, 18.0, YELLOW);
            
            // Barre de PV
            let max_pv = 50.0;
            let pv_ratio = (joueur.get_pv() as f32) / max_pv;
            let bar_width = 150.0;
            draw_rectangle(x + 20.0, joueur_y + 35.0, bar_width, 15.0, DARKGRAY);
            draw_rectangle(x + 20.0, joueur_y + 35.0, bar_width * pv_ratio, 15.0, GREEN);
        },
        None => {},
    }

    // Pokémon sauvage (en haut à droite)
    match &combat.pokemon_sauvage {
        Some(sauvage) => {
            let sauvage_y = y + 20.0;
            draw_text(&format!("{} (Sauvage)", sauvage.get_nom()), x + window_width - 250.0, sauvage_y, 20.0, WHITE);
            let pv_text = format!("PV: {}", sauvage.get_pv());
            draw_text(&pv_text, x + window_width - 250.0, sauvage_y + 25.0, 18.0, YELLOW);
            
            // Barre de PV
            let max_pv = 50.0;
            let pv_ratio = (sauvage.get_pv() as f32) / max_pv;
            let bar_width = 150.0;
            draw_rectangle(x + window_width - 250.0, sauvage_y + 35.0, bar_width, 15.0, DARKGRAY);
            draw_rectangle(x + window_width - 250.0, sauvage_y + 35.0, bar_width * pv_ratio, 15.0, GREEN);
        },
        None => {},
    }

    // === MESSAGE DE COMBAT ===
    let msg_y = y + window_height / 2.0;
    draw_text(&combat.message_combat, x + 20.0, msg_y, 16.0, WHITE);

    // === BOUTONS D'ACTION ===
    let boutons_y = y + window_height - 80.0;
    let bouton_width = 130.0;
    let bouton_height = 35.0;
    let spacing = 20.0;

    // Bouton 1: Attaquer
    let btn1_x = x + 20.0;
    dessiner_bouton(btn1_x, boutons_y, bouton_width, bouton_height, "Attaquer (1)", 
                    combat.action_selectionnee == ActionCombat::Attaquer);

    // Bouton 2: Pokéball
    let btn2_x = btn1_x + bouton_width + spacing;
    dessiner_bouton(btn2_x, boutons_y, bouton_width, bouton_height, "Pokéball (2)", 
                    combat.action_selectionnee == ActionCombat::Pokeball);

    // Bouton 3: Potion
    let btn3_x = btn2_x + bouton_width + spacing;
    dessiner_bouton(btn3_x, boutons_y, bouton_width, bouton_height, "Potion (3)", 
                    combat.action_selectionnee == ActionCombat::Potion);

    // Bouton 4: Fuir
    let btn4_x = btn3_x + bouton_width + spacing;
    dessiner_bouton(btn4_x, boutons_y, bouton_width, bouton_height, "Fuir (4)", 
                    combat.action_selectionnee == ActionCombat::Fuir);
}

fn dessiner_bouton(x: f32, y: f32, width: f32, height: f32, label: &str, selected: bool) {
    let couleur = if selected { Color::new(0.2, 0.8, 0.2, 1.0) } else { Color::new(0.3, 0.3, 0.3, 1.0) };
    let bordure_couleur = if selected { GREEN } else { WHITE };
    let epaisseur = if selected { 3.0 } else { 1.0 };

    draw_rectangle(x, y, width, height, couleur);
    draw_rectangle_lines(x, y, width, height, epaisseur, bordure_couleur);
    draw_text(label, x + 5.0, y + height / 2.0 + 5.0, 14.0, WHITE);
}

pub fn traiter_input_combat(combat: &mut CombatState, inventaire_potions: &mut i32) {
    if !combat.tour_joueur {
        return; // C'est au tour du sauvage
    }
    
    // Touches numériques pour les actions (alternative à ABCD)
    if is_key_pressed(KeyCode::Key1) || is_key_pressed(KeyCode::A) {
        println!("Action: Attaque!");
        combat.action_selectionnee = ActionCombat::Attaquer;
        combat.joueur_attaque();
    } else if is_key_pressed(KeyCode::Key2) || is_key_pressed(KeyCode::B) {
        println!("Action: Pokéball!");
        combat.action_selectionnee = ActionCombat::Pokeball;
        combat.message_combat = "Vous avez lancé une Pokéball!".to_string();
        combat.combat_timer = 0.0;
        combat.tour_joueur = false;
    } else if is_key_pressed(KeyCode::Key3) || is_key_pressed(KeyCode::C) {
        println!("Action: Potion!");
        combat.action_selectionnee = ActionCombat::Potion;
        
        if *inventaire_potions > 0 {
            *inventaire_potions -= 1;
            combat.joueur_utilise_potion();
            println!("Potions restantes: {}", inventaire_potions);
        } else {
            combat.message_combat = "Vous n'avez pas de potion!".to_string();
            combat.tour_joueur = true;  // Rester au tour du joueur
        }
    } else if is_key_pressed(KeyCode::Key4) || is_key_pressed(KeyCode::D) {
        println!("Action: Fuir!");
        combat.action_selectionnee = ActionCombat::Fuir;
        combat.message_combat = "Vous tentez de fuir...".to_string();
    }
}
