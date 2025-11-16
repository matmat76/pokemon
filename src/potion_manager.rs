use crate::potion::Potion;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct PotionManager {
    pub potions: Arc<Mutex<Vec<Potion>>>,
    pub potion_counter: Arc<Mutex<u32>>,
}

impl PotionManager {
    /// Crée un nouveau gestionnaire de potions avec un thread générateur
    pub fn new() -> Self {
        let potions = Arc::new(Mutex::new(Vec::new()));
        let potion_counter = Arc::new(Mutex::new(0));

        let potions_clone = Arc::clone(&potions);
        let counter_clone = Arc::clone(&potion_counter);

        // Lance le thread générateur de potions
        thread::spawn(move || {
            loop {
                // Attend 2-3 secondes avant de générer une potion
                thread::sleep(Duration::from_secs(2));

                // Récupère l'ID de la prochaine potion
                let mut counter = counter_clone.lock().unwrap();
                let potion_id = *counter;
                *counter += 1;
                drop(counter); // Déverrouille explicitement

                // Crée une nouvelle potion à une position aléatoire
                let nouvelle_potion = Potion::random_position(potion_id);

                // Ajoute la potion à la liste partagée
                let mut potions_list = potions_clone.lock().unwrap();
                potions_list.push(nouvelle_potion);
                println!("✨ Nouvelle potion générée à ({}, {}) [ID: {}]", 
                         nouvelle_potion.x, nouvelle_potion.y, potion_id);
                drop(potions_list); // Déverrouille explicitement
            }
        });

        PotionManager {
            potions,
            potion_counter,
        }
    }

    /// Récupère la liste actuelle des potions (lecture depuis Mutex)
    pub fn get_potions(&self) -> Vec<Potion> {
        let potions_list = self.potions.lock().unwrap();
        potions_list.clone()
    }

    /// Supprime une potion par ID (appelé quand le joueur la récupère)
    pub fn remove_potion(&self, potion_id: u32) {
        let mut potions_list = self.potions.lock().unwrap();
        potions_list.retain(|p| p.id != potion_id);
        println!("🎯 Potion {} ramassée!", potion_id);
    }

    /// Vérifie les collisions avec le joueur et récupère les potions touchées
    pub fn collect_potions_at_position(&self, player_x: i32, player_y: i32) -> Vec<Potion> {
        let mut potions_list = self.potions.lock().unwrap();
        let mut collected = Vec::new();

        // Trouve les potions en collision avec le joueur
        potions_list.retain(|potion| {
            if potion.is_colliding_with_player(player_x, player_y) {
                collected.push(*potion);
                false // Supprime la potion (retain la garde si false)
            } else {
                true // Garde la potion
            }
        });

        if !collected.is_empty() {
            println!("📦 Le joueur a ramassé {} potion(s)!", collected.len());
        }

        collected
    }
}
