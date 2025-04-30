use std::collections::{HashMap,HashSet};

use std::sync::{Arc, Mutex};
use crate::app::filiere_window::Classe;
use crate::struc::teacher::{Teacher, Etat};
use crate::app::room_window::Room;

use crate::struc::horaire::{TypeCreneau};
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Creneaux {
    pub id_prof: Option<usize>,
    pub id_classe: Option<usize>, //Option<Arc<Classe>>,
    pub id_groupe: Option<usize>,
    pub id_salle: Option<usize>,//Option<Room>,
    pub id_matiere: Option<HashSet<usize>>,//Option<Room>,
    pub actif_ou_repas: Option<TypeCreneau>,
    pub preference: Option<Etat>,
    pub cours_groupe: Option<HashMap<(usize,usize),usize>>, //(groupe, prof), salle
    pub liste_classe: Option<HashMap<(usize,usize),HashSet<usize>>>, //(matiere, prof), hashset(classe)
}

impl Creneaux{
    pub fn new() -> Self{
        Self{
            id_prof: None,
            id_classe: None,
            id_groupe: None,
            id_salle: None,
            id_matiere: None,
            actif_ou_repas: None,
            preference: None,
            cours_groupe: None,
            liste_classe: None,
        }
    }

    pub fn get_actif_ou_repas(&self) -> &Option<TypeCreneau>{ //&Option<Teacher>{
        &self.actif_ou_repas
    }
    pub fn set_actif_ou_repas(&mut self, actif_ou_repas: TypeCreneau){ //&Option<Teacher>{
        self.actif_ou_repas = Some(actif_ou_repas);
    }

    pub fn get_preference(&self) -> &Option<Etat>{ //&Option<Teacher>{
        &self.preference
    }
    pub fn set_preference(&mut self, preference: Etat){ //&Option<Teacher>{
        self.preference = Some(preference);
    }

    pub fn get_prof(&self) -> &Option<usize>{ //&Option<Teacher>{
        &self.id_prof
    }
    pub fn get_classe(&self) -> &Option<usize>{ // &Option<Arc<Classe>>{
        &self.id_classe
    }
    pub fn get_groupe(&self) -> &Option<usize>{ // &Option<Arc<Classe>>{
        &self.id_groupe
    }
    pub fn get_salle(&self) -> &Option<usize>{ // &Option<Room>{
        &self.id_salle
    }
    pub fn get_matiere(&self) -> &Option<HashSet<usize>>{ // &Option<Room>{
        &self.id_matiere
    }

    pub fn set_prof(&mut self, prof: Option<usize>) { //Option<Teacher>) {
        self.id_prof = prof;
    }
    pub fn set_classe(&mut self, classe: Option<usize>) { //Option<Arc<Classe>>) {
        self.id_classe = classe;
    }
    pub fn set_groupe(&mut self, groupe: Option<usize>) { //Option<Arc<Classe>>) {
        self.id_groupe = groupe;
    }
    pub fn set_salle(&mut self, salle: Option<usize>) { //Option<Room>) {
        self.id_salle = salle;
    }
    pub fn set_matiere(&mut self, matiere: Option<HashSet<usize>>) { //Option<Room>) {
        self.id_matiere = matiere;
    }
    pub fn add_matiere(&mut self, matiere: usize) { //Option<Room>) {
        let mut liste_matiere = self.id_matiere.get_or_insert_with(HashSet::new);
        liste_matiere.insert(matiere);
        self.id_matiere = Some(liste_matiere.clone());

        
    }

    pub fn get_cours_groupe(&self) -> &Option<HashMap<(usize,usize),usize>>{ //&Option<Teacher>{
        &self.cours_groupe
    }
    pub fn set_cours_groupe(&mut self, cours_groupe: HashMap<(usize,usize),usize>){ //&Option<Teacher>{
        self.cours_groupe = Some(cours_groupe);
    }


    pub fn get_liste_classe(&self) -> &Option<HashMap<(usize,usize),HashSet<usize>>>{ //&Option<Teacher>{
        &self.liste_classe
    }


    pub fn add_liste_classe(&mut self, id_matiere: usize, id_prof: usize, id_classe: usize){ 
        if let Some(map) = self.liste_classe.as_mut() {
            let entry = map.entry((id_matiere, id_prof)).or_insert_with(HashSet::new);
            entry.insert(id_classe);
        } else {
            let mut map = HashMap::new();
            let mut new_set = HashSet::new();
            new_set.insert(id_classe);
            map.insert((id_matiere, id_prof), new_set);
            self.liste_classe = Some(map);
        }
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
    pub fn get_planning(&self) -> &HashMap<(usize,usize), Creneaux> {
        &self.planning
    }

    pub fn init_planning(&mut self, id_jour: usize, id_heure: usize, actif_ou_repas: TypeCreneau, etat: Etat){
        //let mut planning: HashMap<(usize,usize), Creneaux>;
        //for nb_j in 0..nb_jour {
        //    for nb_h in 0..nb_heure{
                self.planning.insert((id_jour, id_heure), Creneaux::new());
                let creneaux = self.planning.get_mut(&(id_jour, id_heure)).unwrap();
                creneaux.actif_ou_repas = Some(actif_ou_repas);
                creneaux.preference = Some(etat);
                if id_jour == 2 && id_heure > 4{
                    println!("ne devrait pas etre généré");
                }
       //     }
       // } 
    }

    pub fn get_creneau(&self, id_jour: usize, id_heure: usize,) -> Option<&Creneaux> {
        self.planning.get(&(id_jour, id_heure))
    }
    
    pub fn set_creneau(&mut self, id_jour: usize, id_heure: usize, id_prof: usize, id_classe: usize, id_groupe: usize, id_salle: usize, id_matiere: HashSet<usize>) {
        let creneaux = self.planning.get_mut(&(id_jour, id_heure)).unwrap();   
        creneaux.id_classe =  Some(id_classe);
        creneaux.id_groupe = Some(id_groupe);
        creneaux.id_prof =  Some(id_prof);
        creneaux.id_salle =  Some(id_salle);
        creneaux.id_matiere =  Some(id_matiere);
    }

    pub fn set_creneau_cours_multiple(&mut self, id_jour: usize, id_heure: usize, cours_groupe: HashMap<(usize,usize),usize>){
        let creneaux: &mut Creneaux = self.planning.get_mut(&(id_jour, id_heure)).unwrap();
        creneaux.cours_groupe = Some(cours_groupe); 
    }

    pub fn set_creneau_cours_interclasse(&mut self, id_jour: usize, id_heure: usize, liste_classe: HashMap<(usize,usize),HashSet<usize>>){
        let creneaux: &mut Creneaux = self.planning.get_mut(&(id_jour, id_heure)).unwrap();
        creneaux.liste_classe = Some(liste_classe); 
    }

    pub fn reset_creneau(&mut self, id_jour: usize, id_heure: usize) {
        let creneaux = self.planning.get_mut(&(id_jour, id_heure)).unwrap();   
        creneaux.id_classe =  None;
        creneaux.id_groupe = None;
        creneaux.id_prof =  None;
        creneaux.id_salle =  None;
        creneaux.id_matiere =  None;
        creneaux.cours_groupe = None;
    }

    pub fn get_verif_random_creneau(&self) -> (usize,usize,bool) {
        let mut rng = rand::thread_rng();
        //let jour = rng.gen_range(0..self.nb_jour);
        //let heure = rng.gen_range(0..self.nb_heure);

        let mut vec_planning: Vec<(usize,usize, bool)> = Vec::new();
        for ((jour,heure), creneau) in self.planning.iter()
            .filter(|((_jour,_heure), creneau)| {creneau.get_prof().is_none()})
        {
            vec_planning.push((*jour, *heure, true));
        }

        let id = rng.gen_range(0..vec_planning.len()); 
        vec_planning[id]
        //let creneau = self.planning.get(&(jour, heure));
        //dbg!(&creneau);
        /*match creneau {
            Some(cre) => if let Some(_) = cre.get_prof() { 
                                        (jour, heure, false)
                                    } else { 
                                        (jour, heure, true)

                                    },
            None => (jour, heure, false)
        }*/

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

    pub fn get_nb_heure(&self) -> &usize{
        &self.numero_semaine
    }
}