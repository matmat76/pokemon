use crate::potion::Potion;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct PotionManager {
    pub potions: Arc<Mutex<Vec<Potion>>>,
    #[allow(dead_code)]
    potion_counter: Arc<Mutex<u32>>,
}

impl PotionManager {
    pub fn new() -> Self {
        let potions = Arc::new(Mutex::new(Vec::new()));
        let potion_counter = Arc::new(Mutex::new(0));

        let potions_clone = Arc::clone(&potions);
        let counter_clone = Arc::clone(&potion_counter);

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(2));
                
                // Gestion d'erreur pour le compteur
                match counter_clone.lock() {
                    Ok(mut counter) => {
                        let potion_id = *counter;
                        *counter += 1;
                        drop(counter);
                        
                        let nouvelle_potion = Potion::random_position(potion_id);
                        
                        // Gestion d'erreur pour la liste de potions
                        match potions_clone.lock() {
                            Ok(mut potions_list) => {
                                potions_list.push(nouvelle_potion);
                                println!("Nouvelle potion générée à ({}, {}) [ID: {}]", 
                                         nouvelle_potion.x, nouvelle_potion.y, potion_id);
                                drop(potions_list);
                            }
                            Err(e) => {
                                eprintln!("Erreur: Impossible d'accéder à la liste de potions: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Erreur: Impossible d'accéder au compteur: {}", e);
                    }
                }
            }
        });

        PotionManager {
            potions,
            potion_counter,
        }
    }

    pub fn get_potions(&self) -> Vec<Potion> {
        match self.potions.lock() {
            Ok(potions_list) => potions_list.clone(),
            Err(e) => {
                eprintln!("Erreur: Impossible de récupérer les potions: {}", e);
                Vec::new()  // Retourne une liste vide en cas d'erreur
            }
        }
    }

    pub fn remove_potion(&self, potion_id: u32) {
        match self.potions.lock() {
            Ok(mut potions_list) => {
                potions_list.retain(|p| p.id != potion_id);
                println!("Potion {} ramassée!", potion_id);
            }
            Err(e) => {
                eprintln!("Erreur: Impossible de supprimer la potion {}: {}", potion_id, e);
            }
        }
    }
    
    pub fn collect_potions_at_position(&self, player_x: i32, player_y: i32) -> Vec<Potion> {
        match self.potions.lock() {
            Ok(mut potions_list) => {
                let mut collected = Vec::new();

                potions_list.retain(|potion| {
                    if potion.is_colliding_with_player(player_x, player_y) {
                        collected.push(*potion);
                        false
                    } else {
                        true 
                    }
                });

                if !collected.is_empty() {
                    println!("Le joueur a ramassé {} potion(s)!", collected.len());
                }

                collected
            }
            Err(e) => {
                eprintln!("Erreur: Impossible de vérifier les collisions: {}", e);
                Vec::new()  // Retourne une liste vide en cas d'erreur
            }
        }
    }
}
