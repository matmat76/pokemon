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
                let mut counter = counter_clone.lock().unwrap();
                let potion_id = *counter;
                *counter += 1;
                drop(counter);
                let nouvelle_potion = Potion::random_position(potion_id);
                let mut potions_list = potions_clone.lock().unwrap();
                potions_list.push(nouvelle_potion);
                println!("✨ Nouvelle potion générée à ({}, {}) [ID: {}]", 
                         nouvelle_potion.x, nouvelle_potion.y, potion_id);
                drop(potions_list);
            }
        });

        PotionManager {
            potions,
            potion_counter,
        }
    }

    pub fn get_potions(&self) -> Vec<Potion> {
        let potions_list = self.potions.lock().unwrap();
        potions_list.clone()
    }

    pub fn remove_potion(&self, potion_id: u32) {
        let mut potions_list = self.potions.lock().unwrap();
        potions_list.retain(|p| p.id != potion_id);
        println!("🎯 Potion {} ramassée!", potion_id);
    }
    
    pub fn collect_potions_at_position(&self, player_x: i32, player_y: i32) -> Vec<Potion> {
        let mut potions_list = self.potions.lock().unwrap();
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
            println!("📦 Le joueur a ramassé {} potion(s)!", collected.len());
        }

        collected
    }
}
