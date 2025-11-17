pub trait Pokemon {
    // Méthode pour attaquer
    fn attaquer(&self) -> i32;

    // Méthode pour prendre des dégâts
    fn prendre_degats(&mut self, degats: i32);

    // Méthode pour vérifier s'il est vivant
    fn est_vivant(&self) -> bool;

    // Pour récupérer le nom du Pokémon 
    fn get_nom(&self) -> &String;

    // Pour afficher les PV restants
    fn get_pv(&self) -> i32;

    // Pour définir les PV (guérison)
    fn set_pv(&mut self, pv: i32);

    // Pour obtenir les PV maximum
    fn get_pv_max(&self) -> i32;
}
pub struct Flamby{
    pub nom: String,
    vie: i32,
    attaque: i32, 
}

pub struct Aquali{
    pub nom: String,
    vie: i32,
    attaque: i32, 
}

pub struct Florizarre{
    pub nom: String,
    vie: i32,
    attaque: i32, 
}

impl Pokemon for Flamby {
    fn attaquer(&self) -> i32 {
        self.attaque
    }

    fn prendre_degats(&mut self, degats: i32) {
        self.vie -= degats;
    }

    fn est_vivant(&self) -> bool{
        self.vie > 0
    }

    fn get_nom(&self) -> &String {
        &self.nom
    }
    fn get_pv(&self) -> i32 {
        self.vie
    }

    fn set_pv(&mut self, pv: i32) {
        self.vie = pv;
    }

    fn get_pv_max(&self) -> i32 {
        50  // PV max = PV initiaux
    }
}

impl Pokemon for Aquali {
    fn attaquer(&self) -> i32 {
        self.attaque
    }

    fn prendre_degats(&mut self, degats: i32) {
        self.vie -= degats;
    }

    fn est_vivant(&self) -> bool{
        self.vie > 0
    }

    fn get_nom(&self) -> &String {
        &self.nom
    }
    fn get_pv(&self) -> i32 {
        self.vie
    }

    fn set_pv(&mut self, pv: i32) {
        self.vie = pv;
    }

    fn get_pv_max(&self) -> i32 {
        70  // PV max = PV initiaux
    }
}

impl Pokemon for Florizarre {
    fn attaquer(&self) -> i32 {
        self.attaque
    }

    fn prendre_degats(&mut self, degats: i32) {
        self.vie -= degats;
    }

    fn est_vivant(&self) -> bool{
        self.vie > 0
    }

    fn get_nom(&self) -> &String {
        &self.nom
    }
    fn get_pv(&self) -> i32 {
        self.vie
    }

    fn set_pv(&mut self, pv: i32) {
        self.vie = pv;
    }

    fn get_pv_max(&self) -> i32 {
        70  // PV max = PV initiaux
    }
}

impl Flamby{
    pub fn new(nom: String) -> Flamby {
        Flamby{
            nom,
            vie: 50,
            attaque: 15,
        }
    }
}

impl Aquali{
    pub fn new(nom: String) -> Aquali {
        Aquali{
            nom,
            vie: 70,
            attaque: 10,
        }
    }
}

impl Florizarre{
    pub fn new(nom: String) -> Florizarre {
        Florizarre{
            nom,
            vie: 60,
            attaque: 12,
        }
    }
}