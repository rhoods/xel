use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use crate::app::filiere_window::Classe;
use crate::struc::teacher::Teacher;
use crate::app::room_window::Room;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Creneaux {
    pub id_prof: Option<usize>,
    pub id_classe: Option<usize>, //Option<Arc<Classe>>,
    pub id_salle: Option<usize>,//Option<Room>,
}

impl Creneaux{
    pub fn new() -> Self{
        Self{
            id_prof: None,
            id_classe: None,
            id_salle: None,
        }
    }

    pub fn get_prof(&self) -> &Option<usize>{ //&Option<Teacher>{
        &self.id_prof
    }
    pub fn get_classe(&self) -> &Option<usize>{ // &Option<Arc<Classe>>{
        &self.id_classe
    }
    pub fn get_salle(&self) -> &Option<usize>{ // &Option<Room>{
        &self.id_salle
    }

    pub fn set_prof(&mut self, prof: Option<usize>) { //Option<Teacher>) {
        self.id_prof = prof;
    }
    pub fn set_classe(&mut self, classe: Option<usize>) { //Option<Arc<Classe>>) {
        self.id_classe = classe;
    }
    pub fn set_salle(&mut self, salle: Option<usize>) { //Option<Room>) {
        self.id_salle = salle;
    }

}

#[derive(Clone, Debug)]
pub enum enum_type_id{
    id_prof,
    id_classe,
    id_salle,
}

#[derive(Clone, Debug)]
pub struct Planning {
    type_id: enum_type_id,
    id: usize,
    numero_semaine: usize,
    nb_heure: usize,
    nb_jour: usize,
    planning: HashMap<(usize,usize), Creneaux>, // Clé : (jour,créneau ), Valeur : TimeSlot
}

impl Planning {
    pub fn new(type_id: enum_type_id, id: usize,numero_semaine: usize, nb_jour: usize, nb_heure: usize, ) -> Self {
        Self {
            type_id,
            id,
            numero_semaine,
            nb_heure,
            nb_jour,
            planning : HashMap::new(),
        }
    }
    pub fn init_planning(&mut self, nb_jour: usize, nb_heure: usize, ){
        //let mut planning: HashMap<(usize,usize), Creneaux>;
        for nb_j in 0..nb_jour {
            for nb_h in 0..nb_heure{
                self.planning.insert((nb_j, nb_h), Creneaux::new());
            }
        } 
    }

    pub fn get_creneau(&self, id_jour: usize, id_heure: usize,) -> Option<&Creneaux> {
        self.planning.get(&(id_jour, id_heure))
    }
    pub fn set_creneau(&mut self, id_jour: usize, id_heure: usize, id_prof: usize, id_classe: usize, id_salle: usize) {
        let creneaux = self.planning.get_mut(&(id_jour, id_heure)).unwrap();   
        creneaux.id_classe =  Some(id_classe);
        creneaux.id_prof =  Some(id_prof);
        creneaux.id_salle =  Some(id_salle);

    }

    pub fn get_verif_random_creneau(&self,) -> (usize,usize,bool) {
        let mut rng = rand::thread_rng();
        let jour = rng.gen_range(0..self.nb_jour);
        let heure = rng.gen_range(0..self.nb_heure);
        let creneau = self.planning.get(&(jour, heure));
        //dbg!(&creneau);
        match creneau {
            Some(cre) => if let Some(_) = cre.get_prof() { 
                                        (jour, heure, false)
                                    } else { 
                                        (jour, heure, true)
                                    },
            None => (jour, heure, false)
        }

    }

    pub fn get_verif_creneau(&self, jour:usize, heure: usize,) -> (usize,usize,bool) {
        let creneau = self.planning.get(&(jour, heure));
        match creneau {
            Some(cre) => if let Some(_) = cre.get_prof() { 
                                        (jour, heure, false)
                                    } else { 
                                        (jour, heure, true)
                                    },
            None => (jour, heure, false)
        }

    }
}