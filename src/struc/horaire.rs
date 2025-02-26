use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeCreneau {
    Actif,
    Repas,
    Desactive,
}

impl TypeCreneau {
    pub fn suivant(&mut self) -> Self {
        match self {
            TypeCreneau::Actif => TypeCreneau::Repas,
            TypeCreneau::Repas => TypeCreneau::Desactive,
            TypeCreneau::Desactive => TypeCreneau::Actif,
        }
    }

    pub fn to_int(&self) -> i8 {
        match self {
            TypeCreneau::Actif => 0,
            TypeCreneau::Repas => 1,
            TypeCreneau::Desactive => 2,
        }
    }

    pub fn from_int(value: i8) -> Self {
        match value {
            0 => TypeCreneau::Actif,
            1 => TypeCreneau::Repas,
            2 => TypeCreneau::Desactive,
            _ => TypeCreneau::Desactive, // Valeur par défaut en cas d'erreur
        }
    }
}

// Structure pour représenter une plage horaire
#[derive(Debug, Clone)]
pub struct CreneauxEtablissement {
    id_jour: usize,
    id_heure: usize,
    name_jour: String,
    name_heure: String,
    dispo: TypeCreneau, 
}

impl CreneauxEtablissement {
    
    pub fn new(id_jour: usize, id_heure:usize) -> Self{
        let name_jour = format!("jour {:}", id_jour);
        let name_heure = format!("heure {:}", id_heure);
        Self { id_jour: id_jour, id_heure: id_heure, name_jour: name_jour, name_heure: name_heure, dispo: TypeCreneau::Actif }
    }

    pub fn get_id_jour(&self) -> &usize {
        &self.id_jour
    }

    pub fn get_id_heure(&self) -> &usize {
        &self.id_heure
    }
    
    pub fn get_name_jour(&self) -> String {
        self.name_jour.clone()
    }
    
    pub fn get_name_heure(&self) -> String {
        self.name_heure.clone()
    }
    
    pub fn get_dispo(&self) -> TypeCreneau {
        self.dispo.clone()
    }

    pub fn set_id_jour(&mut self, id_jour:usize) {
        self.id_jour = id_jour;
    }

    pub fn set_id_heure(&mut self, id_heure:usize) {
        self.id_heure = id_heure;
    }
    
    pub fn set_name_jour(&mut self, name_jour:String) {
        self.name_jour = name_jour;
    }
    
    pub fn set_name_heure(&mut self, name_heure:String) {
        self.name_heure = name_heure;
    }
    pub fn set_dispo(&mut self, dispo:TypeCreneau) {
        self.dispo = dispo;
    }
    
    pub fn update(&mut self) {
        self.dispo = self.dispo.suivant();
    }

    pub fn charge_creneau(id_jour: usize, id_heure:usize, name_jour: String, name_heure:String, dispo:TypeCreneau)  -> Self {
        Self{
            id_jour,
            id_heure,
            name_jour,
            name_heure,
            dispo,
        }
        /*self.set_id_jour(id_jour);
        self.set_id_heure(id_heure);
        self.set_name_jour(name_jour);
        self.set_name_heure(name_heure);
        self.set_dispo(dispo);*/

    }
    
}