use eframe::egui;
use std::sync::{Arc, Mutex};
use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use egui::{/*Context, Ui,*/ Color32, /*Align2,*/ Frame, Vec2};
use rusqlite::{params, Connection, Result};

//use egui::{Context, Ui, Color32, Align2, Frame, Vec2};
use std::collections::{HashMap, HashSet};
use crate::struc::matiere::Matiere;
use crate::struc::planning::{Planning, Creneaux, enum_type_id};
use crate::struc::teacher::{Teacher};
use crate::struc::assignation::{self, Assignation, Groupe};
use crate::struc::programme::{MatiereProg, Semaine, MatiereInterClasse}; 
use crate::app::filiere_window::{Filiere, Classe};
use crate::app::room_window::{Room,RoomType};
use crate::struc::horaire::{CreneauxEtablissement, TypeCreneau};


pub enum type_planning{
    prof,
    classe,
    salle,
}

#[derive(Clone, Debug)]
pub struct PlanningWindow {
    nb_essai:usize,
    selected_semaine_id: usize,
    selected_prof_id: usize,
    generation_reussi: bool,
    nb_semaine_max: usize,
    nb_semaine_max_par_filiere: HashMap<usize, usize>,// (id_filiere, nb_semaine)
    liste_creneau_a_placer: HashMap<(usize,usize,usize,usize,usize), (usize, usize, usize)>, //<(id_classe, id_prof,id_matiere, id_groupe,id_semaine), (nb_heure, duree_mini, duree_max)>
    liste_creneau_placer: HashMap<(usize,usize,usize,usize,usize), usize>,
    liste_creneau_non_placer: HashMap<(usize,usize,usize,usize,usize), usize>,
    planning_prof: HashMap<(usize,usize), Planning>, //(id_prof, num semaine), planning
    planning_classe: HashMap<(usize,usize), Planning>, //(id_classe, num semaine), planning
    planning_room: HashMap<(usize,usize,usize), Planning>, //(id_room,id_type_salle, num semaine), planning

    semaine: HashMap<(usize,usize), Arc<Semaine>>,
    classe: HashMap<usize, Arc<Classe>>,
    filiere: HashMap<usize, Arc<Filiere>>,
    matiere: HashMap<usize,Arc<Matiere>>,
    matiere_prog:  HashMap<usize, Arc<MatiereProg>>,
    matiere_inter_classe: HashMap<usize, Arc<MatiereInterClasse>>,
    teachers: HashMap<usize, Teacher>,
    groupe: HashMap<usize, Arc<Groupe>>,
    assignement: HashMap<usize, Arc<Assignation>>,
    salle: HashMap<usize, Room>,
    horaires: HashMap<(usize,usize), CreneauxEtablissement>,

}

impl  Default for PlanningWindow  {
    fn default() -> Self {
        Self {
            nb_essai: 0,
            selected_semaine_id: 0,
            selected_prof_id: 0,
            generation_reussi: false,
            nb_semaine_max: 0,
            nb_semaine_max_par_filiere: HashMap::new(),
            liste_creneau_a_placer: HashMap::new(),
            liste_creneau_placer: HashMap::new(),
            liste_creneau_non_placer: HashMap::new(),
            planning_prof: HashMap::new(),
            planning_classe: HashMap::new(),
            planning_room: HashMap::new(),

            semaine: HashMap::new(),
            classe: HashMap::new(),
            filiere: HashMap::new(),
            matiere: HashMap::new(),
            matiere_prog:  HashMap::new(),
            matiere_inter_classe: HashMap::new(),
            teachers: HashMap::new(),
            groupe: HashMap::new(),
            assignement: HashMap::new(),
            salle: HashMap::new(),
            horaires: HashMap::new(), 
        }
    }
}

impl PlanningWindow {
    pub fn charge(&mut self, salle:HashMap<usize, Room>,  semaine: HashMap<(usize,usize), Arc<Semaine>>, classe: HashMap<usize, Arc<Classe>>, filiere: HashMap<usize, Arc<Filiere>>, matiere:HashMap<usize, Arc<Matiere>>,   matiere_prog: HashMap<usize, Arc<MatiereProg>>, matiere_inter_classe: HashMap<usize, Arc<MatiereInterClasse>>, teachers: HashMap<usize, Teacher>, groupe: HashMap<usize, Arc<Groupe>>,  assignement :HashMap<usize, Arc<Assignation>>, horaires: HashMap<(usize,usize), CreneauxEtablissement>) {
        self.semaine = semaine;
        self.classe =  classe;
        self.filiere = filiere;
        self.matiere = matiere;
        self.matiere_prog = matiere_prog;
        self.matiere_inter_classe =  matiere_inter_classe;
        self.teachers =  teachers;
        self.groupe = groupe;
        self.assignement = assignement;
        self.salle = salle;
        self.horaires = horaires;
    }

    pub fn build(&mut self, ctx: &egui::Context,) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.horizontal(|ui| { 
                    if ui.button("Générer plannings").clicked(){
                        let mut count = 0;
                        while !self.generation_reussi && count < 100 {
                            self.create_planning();
                            count += 1;
                        }
                        dbg!(&self.generation_reussi);
                        dbg!(&self.liste_creneau_non_placer);
                        if self.generation_reussi{
                            print!("Mission Reussi!");
                            self.generation_reussi = false;
                        }
                        else{
                            print!("Echec...");
                        }
                    }
                });  
                ui.horizontal(|ui| {
                    for (id_prof, prof) in self.teachers.iter() {
                            if ui.selectable_label(&self.selected_prof_id == id_prof,format!("{:}", prof.get_name())).clicked() {                                  
                                self.selected_prof_id = *id_prof; 
                            }   
                    }
                });

                ui.horizontal(|ui| {
                    for ((id_prof,id_semaine), planning) in self.planning_prof.iter()
                    .filter(|((id_prof_planning,id_semaine), planning)| {self.selected_prof_id == *id_prof_planning}) 
                    {
                        if ui.selectable_label(&self.selected_semaine_id == id_semaine,format!("{:}", id_semaine)).clicked() {                                  
                            self.selected_semaine_id = *id_semaine;
                        }
                    }
                });

                    
                    
                        ui.visuals_mut().selection.bg_fill = Color32::TRANSPARENT; //enleve l'effet de selection sur les label du planning (changement couleur du fond)
                        ui.separator();

                        ui.heading("Planning");
                        let cell_size = Vec2::new(100.0, 30.0);
                        let days = ["Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi"];
                        let hours = [
                            "8h-9h", "9h-10h", "10h-11h", "11h-12h", "12h-13h", "13h-14h",
                            "14h-15h", "15h-16h", "16h-17h", "17h-18h",
                        ];
                        
                        egui::Grid::new("schedule_grid")
                        .show(ui, |ui| {
                            // En-têtes des colonnes
                            ui.label("");

                            for (id_day,day) in days.iter().enumerate() {
                                //genere les noms de colonnes
                                let response_day= ui.add_sized(cell_size, egui::SelectableLabel::new(false, *day));
                                if response_day.clicked() {
                                    for hour_idx in 0..hours.len() {
                                        //teacher.set_availability(id_day, hour_idx);
                                    } 
                                }              
                            }

                            ui.end_row();
                            
                            let mut dispo: (usize,usize,bool);
                            let mut creneau: Option<&Creneaux>;
                            for (hour_idx, hour) in hours.iter().enumerate() {
                                //genere les noms des plages horaires
                                let response_hours = ui.add_sized(cell_size, egui::SelectableLabel::new(false,*hour)); //ui.label(*hour);
                                //permet de cliquer sur l'heure pour changer l'état de cette plage horaire pour chaque journée
                                if response_hours.clicked() {
                                    for id_day in 0..days.len() {
                                        //teacher.set_availability(id_day, hour_idx);
                                    } 
                                }
                           
                                for days_idx in 0..days.len() {

                                    if self.planning_prof.contains_key(&(self.selected_prof_id, self.selected_semaine_id)){
                                        let planning = self.planning_prof.get(&(self.selected_prof_id, self.selected_semaine_id)).unwrap();
                                        dispo  = planning.get_verif_creneau(days_idx, hour_idx);
                                        creneau = planning.get_creneau(days_idx, hour_idx);
                                    }else{
                                        dispo  = (days_idx, hour_idx, true);
                                        creneau = None;
                                    }  
                                    
                                    // Définition des couleurs d'arrière-plan
                                    let (bg_color, text_color) = if !dispo.2 && !creneau.is_none(){
                                        (Color32::from_rgb(255, 200, 200), Color32::RED)  // Rouge clair pour indisponible
                                    } else {
                                        (Color32::from_rgb(200, 255, 200), Color32::DARK_GREEN)  // Vert clair pour disponible
                                    };

                                    //dbg!(&creneau);
                                    let text = 
                                        if !dispo.2 && !creneau.is_none() { 
                                            let prof = self.teachers.get(&creneau.unwrap().id_prof.unwrap()).unwrap();
                                            let classe = self.classe.get(&creneau.unwrap().id_classe.unwrap()).unwrap();
                                            let salle = self.salle.get(&creneau.unwrap().id_salle.unwrap()).unwrap();
                                            let matiere = self.matiere.get(&creneau.unwrap().id_matiere.unwrap()).unwrap();

                                            format!("{:} \n {:} \n {:} \n {:}", prof.get_name(), classe.get_name(), salle.get_name(), matiere.get_name()) 
                                        } else { 
                                            format!("{:} \n {:} \n {:} \n {:}", " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string()) 
                                        };
                                    // Création d'un Frame aHashMap la couleur d'arrière-plan
                                    Frame::none()
                                        .fill(bg_color)
                                        .inner_margin(egui::style::Margin::symmetric(4.0, 0.0))
                                        .show(ui, |ui| {
                                            let response = ui.add_sized(
                                                cell_size,
                                                egui::SelectableLabel::new(
                                                    dispo.2,
                                                    egui::RichText::new(text).color(text_color)
                                                )
                                            );
                                            
                                            /*if response.clicked() {
                                                teacher.set_availability(days_idx, hour_idx);

                                            }*/
                                        });
                                }
                                ui.end_row();
                            }
                        });

                });
        //    });
    }

    pub fn alim_nb_semaine_max(&mut self){
        for (id, filiere) in self.filiere.iter(){
            if filiere.get_nb_semaine() > Some(self.nb_semaine_max) {
                self.nb_semaine_max = filiere.get_nb_semaine().unwrap();
            }
            /*if !self.nb_semaine_max_par_filiere.get(&filiere.get_id()).is_none() {
                self.nb_semaine_max_par_filiere.insert(filiere.get_id(),filiere.get_nb_semaine().unwrap_or(0));
            } else if  self.nb_semaine_max_par_filiere.get(&filiere.get_id()).unwrap() < &filiere.get_nb_semaine().unwrap_or(0){
                self.nb_semaine_max_par_filiere.insert(filiere.get_id(),filiere.get_nb_semaine().unwrap_or(0));
            }*/
        }
    }
    
    
    pub fn init_planning(&mut self){
        for (id_prof, _prof) in self.teachers.iter(){
            for num_sem in 0..self.nb_semaine_max {
                //dbg!(id_prof);
                //dbg!(num_sem);
                self.planning_prof.insert((*id_prof, num_sem), Planning::new(enum_type_id::id_prof, *id_prof,num_sem,5,8));
                let planning = self.planning_prof.get_mut(&(*id_prof, num_sem)).unwrap();
                //dbg!(&planning);
                for ((id_jour, id_heure), creneau) in self.horaires.iter()
                //.filter(|((id_jour, id_heure), creneau)| { creneau.get_dispo() != TypeCreneau::Desactive })
                {
                    let actif_ou_repas = creneau.get_dispo();
                    if actif_ou_repas == TypeCreneau::Actif{
                        planning.init_planning(*id_jour, *id_heure, actif_ou_repas);
                    }
                    
                }
                //dbg!(&planning);
            }
        }
        for (id_classe, _classe) in self.classe.iter(){
            for num_sem in 0..self.nb_semaine_max {
                self.planning_classe.insert((*id_classe, num_sem), Planning::new(enum_type_id::id_classe, *id_classe,num_sem,5,8));
                let planning = self.planning_classe.get_mut(&(*id_classe, num_sem)).unwrap();
                for ((id_jour, id_heure), creneau) in self.horaires.iter()
                //.filter(|((id_jour, id_heure), creneau)| { creneau.get_dispo() != TypeCreneau::Desactive })
                {
                    let actif_ou_repas = creneau.get_dispo();
                    if actif_ou_repas == TypeCreneau::Actif{
                        planning.init_planning(*id_jour, *id_heure, actif_ou_repas);
                    }
                }
            }
        }
        for (id_room, room) in self.salle.iter(){
            for num_sem in 0..self.nb_semaine_max {
                let id_type_salle = room.get_room_type().get_id();
                self.planning_room.insert((*id_room, id_type_salle, num_sem), Planning::new(enum_type_id::id_salle, *id_room,num_sem,5,8));
                let planning = self.planning_room.get_mut(&(*id_room, id_type_salle, num_sem)).unwrap();
                for ((id_jour, id_heure), creneau) in self.horaires.iter()
                //.filter(|((id_jour, id_heure), creneau)| { creneau.get_dispo() != TypeCreneau::Desactive })
                {
                    let actif_ou_repas = creneau.get_dispo();
                    if actif_ou_repas == TypeCreneau::Actif{
                        planning.init_planning(*id_jour, *id_heure, actif_ou_repas);
                    }
                }
            }
        }
    }





    pub fn alim_creneau_a_placer(&mut self){
        //construction de la liste des creneaux à placer ( liste des matieres par classe et le nombre d'heure de chacune)
        self.liste_creneau_non_placer = HashMap::new();
        for (_id, assignation) in self.assignement.iter(){
            let id_filiere = assignation.get_classe().get_filiere().get_id();
            let id_matiere = assignation.get_matiere().get_id();
            let id_classe = assignation.get_classe().get_id();
            let id_groupe = assignation.get_groupe().get_id();
            let id_prof = assignation.get_prof().get_id();

            for (_id_mat_prog, matiere_prog) in self.matiere_prog.iter()
                                                                    .filter(|(_id_s, mat_prog)| 
                                                                        {mat_prog.get_semaine().get_filiere().get_id() ==id_filiere 
                                                                        && mat_prog.get_matiere().get_id() == id_matiere})
            {
                let id_semaine = *matiere_prog.get_semaine().get_id();
                self.liste_creneau_a_placer.insert((id_classe,id_prof, *id_matiere, *id_groupe, id_semaine),(*matiere_prog.get_nb_heure(),*matiere_prog.get_duree_minimum(),*matiere_prog.get_duree_maximum()));
            }
        }
    }






//-> (usize,usize,bool) --> devient (usize,usize,usize,bool) pour ajouter la durée?
    pub fn trouve_creneau_dispo_prof(&self, id_prof: &usize, id_semaine: &usize, nb_heure_restant: &usize, duree_min: &usize, duree_max: &usize) -> (usize, usize, usize, bool){
        let mut new_duree_max = 0;
        let mut dispo = false;
        let mut nb_essai: usize = 0;
        //let mut longueur_creneau = *duree_max;
        //let mut nb_essai: usize = 0;
        let mut creneau_dispo: (usize,usize,bool) = (0,0,false);
        while nb_essai < 400 {
            creneau_dispo  = self.trouve_creneau_alea_dispo_prof(id_prof, id_semaine);
            if creneau_dispo.2 {
                new_duree_max = 1;
                dispo = true;
                break;
            }
            nb_essai += 1;
        }

        if creneau_dispo.2 {
            new_duree_max = *duree_max;
            while self.trouve_longueur_creneau_max(id_prof, id_semaine, creneau_dispo.0,creneau_dispo.1 , &new_duree_max) <  *duree_min && new_duree_max > *duree_min{
                new_duree_max -= *duree_min;  
            };
        }
        (creneau_dispo.0, creneau_dispo.1, new_duree_max, dispo ) //jour, heure, durée
    }
    




    pub fn trouve_longueur_creneau_max(&self, id_prof: &usize, id_semaine: &usize, jour:usize , heure:usize , duree_max: &usize) -> usize{
        let mut i_max =  *duree_max;
        let mut ok: bool  = true;
        //let mut i_min =  *duree_min;
        let mut creneau_dispo = (jour,heure, true);
        for i in 1..i_max{
            match self.planning_prof.get(&(*id_prof, *id_semaine)){
                Some(planning) => {
                                               creneau_dispo = planning.get_verif_creneau(jour, heure + i);
                                                if !creneau_dispo.2 {
                                                    ok = false;
                                                    break;
                                                }
                                            },
                None => {ok = false; break;},
            };
        }

        match ok {
            true => i_max,
            false => 1
        }
    }


    pub fn trouve_creneau_alea_dispo_prof(&self, id_prof: &usize, id_semaine: &usize) -> (usize,usize,bool){
        match self.planning_prof.get(&(*id_prof, *id_semaine)){
            Some(planning) => planning.get_verif_random_creneau(),
            None => (0,0,false),
        }
    }

    

    pub fn create_planning(&mut self) {

        self.alim_nb_semaine_max();
        self.init_planning();
        self.alim_creneau_a_placer();
        //debut placement des creneaux
        let mut id_type_salle: usize;

        self.generation_reussi = true;

        
        let mut rng_1 = thread_rng();
        let mut keys_a_placer: Vec<_> = self.liste_creneau_a_placer.iter().collect();
        keys_a_placer.shuffle(&mut rng_1);
                       

                        
        //for ((id_classe, id_prof, id_matiere, id_groupe, id_semaine), (nb_heure, duree_min, duree_max)) in self.liste_creneau_a_placer.iter(){
        for ((id_classe, id_prof, id_matiere, id_groupe, id_semaine), (nb_heure, duree_min, duree_max)) in keys_a_placer{
             
            let mut nb_heure_restant = *nb_heure;
            let mut nb_max_passage = 0;
            let mut creneau_dispo: (usize,usize,usize,bool) = (1,1,1,false); 
            let mut creneau_dispo_salle: (usize,usize,bool) = (0,0,false); 
            let mut id_salle: usize = 0;
            
            match self.matiere.get(id_matiere){
                Some(matiere) => id_type_salle = matiere.get_room_type().get_id(),
                None => {   
                            break; 
                        },
            };

            while nb_heure_restant > 0 {
                //dbg!(&nb_max_passage);
                if nb_max_passage > 40000 {
                    self.generation_reussi = false;
                    self.liste_creneau_non_placer.insert((*id_classe, *id_prof, *id_matiere, *id_groupe, *id_semaine), nb_heure_restant);   
                    break;
                }

                if let Some(planning_prof) = self.planning_prof.get(&(*id_prof, *id_semaine)){
                    if let Some(planning_classe) = self.planning_classe.get(&(*id_classe, *id_semaine)){
                        let mut creneau_trouve = false;

                        let mut rng = thread_rng();
                        let mut keys: Vec<_> = planning_prof.get_planning().keys().collect();
                        keys.shuffle(&mut rng);
                       

                        //for ((jour, heure), _creneau_prof) in planning_prof.get_planning().iter() {
                        for    (jour, heure) in  keys{
                            creneau_dispo = (1,1,1,false);
                            
                            let mut new_duree_max: usize;
                            if nb_heure_restant > *duree_max{
                                new_duree_max = *duree_max;
                            }else{
                                new_duree_max = nb_heure_restant;
                            }

                            while new_duree_max >= *duree_min {

                                for i in 0..new_duree_max { 
                                    creneau_trouve = false;
                                    if let Some(new_creneau_prof) = planning_prof.get_creneau(*jour,*heure + i){
                                        if let Some(creneau_classe) = planning_classe.get_creneau(*jour,*heure + i){
                                            if new_creneau_prof.get_prof().is_none() && creneau_classe.get_prof().is_none() {
                                                creneau_trouve = true;
                                            }      
                                        }   
                                    }
                                    if !creneau_trouve {
                                        new_duree_max -= *duree_min;
                                        break;
                                    }
                                }

                                if creneau_trouve {
                                    creneau_dispo = (*jour, *heure, new_duree_max, true);
                                    break;
                                }else if new_duree_max == *duree_min{
                                    nb_max_passage += 1;
                                    break;
                                }
                            }
                            //on sort de la boucle for
                            if creneau_dispo.3 {
                                break;
                            }
                        }

                    }
                }

                if creneau_dispo.3 {
                    for (id_room, _room) in self.salle.iter().filter(|(_id, room)|{room.get_room_type().get_id() == id_type_salle}){
                        if !creneau_dispo_salle.2{
                            for i in 0..creneau_dispo.2 {
                                match self.planning_room.get(&(*id_room, id_type_salle, *id_semaine)) {
                                    Some(planning_room) => {
                                                                        id_salle = *id_room;   
                                                                        creneau_dispo_salle = planning_room.get_verif_creneau(creneau_dispo.0, creneau_dispo.1 + i);
                                                                    },
                                    None => continue,
                                }
                            }
                        }
                    }
                }

                if !creneau_dispo_salle.2 {
                    nb_max_passage += 1;
                    continue;
                }else{
                    //CRENEAU TROUVE
                    let planning_prof = self.planning_prof.get_mut(&(*id_prof, *id_semaine)).unwrap();
                    let planning_classe = self.planning_classe.get_mut(&(*id_classe, *id_semaine)).unwrap();
                    let planning_salle = self.planning_room.get_mut(&(id_salle, id_type_salle, *id_semaine)).unwrap();
                    
                    for i in 0..creneau_dispo.2 {
                        planning_prof.set_creneau(creneau_dispo.0, creneau_dispo.1 + i, *id_prof, *id_classe, id_salle, *id_matiere);
                        planning_classe.set_creneau(creneau_dispo.0, creneau_dispo.1 + i, *id_prof, *id_classe, id_salle, *id_matiere);
                        planning_salle.set_creneau(creneau_dispo.0, creneau_dispo.1 + i, *id_prof, *id_classe, id_salle, *id_matiere);
                    }   
                    nb_heure_restant -= creneau_dispo.2; //duree du creneau
                    self.liste_creneau_placer.insert((*id_classe, *id_prof, *id_matiere, *id_groupe, *id_semaine), *nb_heure);   
                    
                }
              
                //si plus d'heure de repas dispo, alors on annule les inserts
                /*if !self.get_dispo_repas(*id_prof, *id_semaine,creneau_dispo.0) || !self.get_dispo_repas(*id_classe, *id_semaine,creneau_dispo.0) {
                    let planning_prof = self.planning_prof.get_mut(&(*id_prof, *id_semaine)).unwrap();
                    let planning_classe = self.planning_classe.get_mut(&(*id_classe, *id_semaine)).unwrap();
                    let planning_salle = self.planning_room.get_mut(&(id_salle, id_type_salle, *id_semaine)).unwrap();

                    for i in 0..creneau_dispo.2 {
                        planning_prof.reset_creneau(creneau_dispo.0, creneau_dispo.1 + i);
                        planning_classe.reset_creneau(creneau_dispo.0, creneau_dispo.1 + i);
                        planning_salle.reset_creneau(creneau_dispo.0, creneau_dispo.1 + i);
                    }
                    nb_max_passage += 1;
                    continue;

                } else {
                    nb_heure_restant -= creneau_dispo.2; //duree du creneau
                    self.liste_creneau_placer.insert((*id_classe, *id_prof, *id_matiere, *id_groupe, *id_semaine), *nb_heure);
                }*/
   
            }      
        }
    }

    //verifie que une des heures de repas est disponible
    pub fn get_dispo_repas(&self, id: usize, id_semaine: usize, jour_creneau: usize/*, dispo_ou_repas: TypeCreneau*/) -> bool{
        
        let mut dispo_repas = false;
        for ((id_jour, id_heure), _creneau) in self.horaires.iter()
        .filter(|((id_jour, _id_heure), creneau)| { 
            *id_jour == jour_creneau
            && creneau.get_dispo() == TypeCreneau::Repas }) 
        {
            if let Some(planning) = self.planning_prof.get(&(id, id_semaine)){
                if let Some(creneau) = planning.get_creneau(*id_jour, *id_heure){
                    match creneau.get_prof() {
                        Some(_) => continue,
                        None => {
                                    dispo_repas = true;
                                    break;
                                },
                    };
                }
            } 
        }
        dispo_repas
    }

    //compte le nombre de creneaux non placés pour la partie fitness
    pub fn get_nb_creneau_restant(&self) -> usize {
        let mut count: usize = 0;
        for ((id_classe, id_prof, id_matiere, id_groupe, id_semaine), (nb_heure, _duree_mini, _duree_max)) in self.liste_creneau_a_placer.iter(){
            match self.liste_creneau_placer.get(&(*id_classe, *id_prof, *id_matiere, *id_groupe, *id_semaine)){
                Some(nb_heure_restant) => count += nb_heure_restant, // nb_heure_restant peut etre égale à 0,
                None => count += nb_heure, //aucun creneau de ce cour n'a été placé,
            }
        }
        count
    }

}