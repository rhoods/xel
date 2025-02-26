use std::collections::HashMap;
use crate::struc::horaire::CreneauxEtablissement;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Etat {
    Indisponible,
    Disponible,
    Preference,
}

impl Etat {
    pub fn suivant(&mut self) -> Self {
        match self {
            Etat::Indisponible => Etat::Disponible,
            Etat::Disponible => Etat::Preference,
            Etat::Preference => Etat::Indisponible,
        }
    }

    pub fn to_int(&self) -> i8 {
        match self {
            Etat::Indisponible => 0,
            Etat::Disponible => 1,
            Etat::Preference => 2,
        }
    }

    pub fn from_int(value: i8) -> Self {
        match value {
            0 => Etat::Indisponible,
            1 => Etat::Disponible,
            2 => Etat::Preference,
            _ => Etat::Indisponible, // Valeur par défaut en cas d'erreur
        }
    }
}

// Structure pour représenter une plage horaire
#[derive(Clone, Debug)]
pub struct TimeSlot {
    pub etat: Etat,
}
impl Default for  TimeSlot{
    fn default() -> Self {
        Self { etat: Etat::Disponible }
    }
}

impl TimeSlot {
    pub fn update(&mut self) {
        self.etat = self.etat.suivant();
    }
    pub fn get_available(&self) -> &Etat {
        &self.etat
    }
    pub fn set_available(&mut self, not_available: Etat) {
        self.etat = not_available;
    }
    pub fn charge(&mut self, not_available: Etat)  {
        self.etat = not_available;
    }
}

// Structure pour représenter un professeur et ses disponibilités

#[derive(Clone, Debug)]
pub struct Teacher {
    id: usize,
    pub name: String,
    pub schedule: HashMap<(usize,usize), TimeSlot>, // Clé : (jour,créneau ), Valeur : TimeSlot
}

impl Teacher {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            schedule: HashMap::new(),
        }
    }
    pub fn update(&mut self, id_jour: usize, id_heure: usize) {
        self.schedule.get_mut(&(id_jour, id_heure)).unwrap().update();
    }
    pub fn init_schedule(&mut self, id_jour: usize, id_heure: usize) {
        self.schedule.insert((id_jour, id_heure), TimeSlot::default());
       
    }

    pub fn get_id(&self) -> usize{
        self.id.clone()
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }
    pub fn set_name(&mut self, new_name:String) { 
        self.name = new_name;
    }

    pub fn set_availability(&mut self, day: usize, hour: usize){
        self.schedule.entry((day,hour)).or_insert(TimeSlot::default()).update();
        let etat = self.schedule.get_mut(&(day,hour)).unwrap();
        etat.update(); //.suivant();
    }
    pub fn get_available(&self, day: usize, hour: usize) -> Option<&TimeSlot> {
        self.schedule.get(&(day,hour))
    }

    pub fn get_creneau_mut(&mut self, day: usize, hour: usize ) -> Option<&mut TimeSlot>{
        self.schedule.get_mut(&(day,hour))
    }
    pub fn get_not_available_liste(&self) -> &HashMap<(usize,usize), TimeSlot>{
        &self.schedule

    }
    pub fn charge_creneau(&mut self, day:usize, hour:usize, not_available: Etat) {
        let mut time_slot = TimeSlot::default();
        time_slot.charge(not_available);
        self.schedule.entry((day,hour)).or_insert(time_slot);
    }


}
