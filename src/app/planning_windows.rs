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
use crate::struc::teacher::{Teacher, Etat};
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
    //liste_creneau_a_placer: HashMap<(usize,usize,usize,usize,usize), (usize, usize, usize)>, //<(id_classe, id_prof,id_matiere, id_groupe,id_semaine), (nb_heure, duree_mini, duree_max)>
    liste_creneau_a_placer: HashMap<(usize,usize,usize,usize,usize), (usize, usize, usize, HashSet<usize>,HashSet<usize>,HashSet<(usize,usize)>,)>,  //<(id_classe, id_prof,id_matiere, id_groupe,id_semaine), (nb_heure, duree_mini, duree_max, liste des classe, liste des profs, les des groupes et prof correspondant)>
    liste_creneau_placer: HashMap<(usize,usize,usize,usize,usize), (usize, HashSet<usize>,HashSet<usize>,HashSet<(usize,usize)>)>,
    liste_creneau_non_placer: HashMap<(usize,usize,usize,usize,usize), (usize, HashSet<usize>,HashSet<usize>,HashSet<(usize,usize)>)>,
    planning_prof: HashMap<(usize,usize), Planning>, //(id_prof, num semaine), planning
    planning_classe: HashMap<(usize,usize), Planning>, //(id_classe, num semaine), planning
    planning_room: HashMap<(usize,usize,usize), Planning>, //(id_room,id_type_salle, num semaine), planning
    
    selected_liste_inter_classe: HashMap<(usize,usize,usize), usize>,
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

            selected_liste_inter_classe: HashMap::new(),
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
    pub fn charge(&mut self, salle:HashMap<usize, Room>,  semaine: HashMap<(usize,usize), Arc<Semaine>>, classe: HashMap<usize, Arc<Classe>>, filiere: HashMap<usize, Arc<Filiere>>, matiere:HashMap<usize, Arc<Matiere>>,   matiere_prog: HashMap<usize, Arc<MatiereProg>>, matiere_inter_classe: HashMap<usize, Arc<MatiereInterClasse>>, teachers: HashMap<usize, Teacher>, groupe: HashMap<usize, Arc<Groupe>>,  assignement :HashMap<usize, Arc<Assignation>>, horaires: HashMap<(usize,usize), CreneauxEtablissement>, selected_liste_inter_classe: HashMap<(usize, usize, usize), usize>) {
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
        self.selected_liste_inter_classe = selected_liste_inter_classe;
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
                        dbg!(&self.liste_creneau_non_placer);
                        if self.generation_reussi{
                            print!("Mission Reussi!");
                            dbg!(&count);
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
                            let mut cours_groupe: HashMap<(usize,usize),usize>= HashMap::new();

                            
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
                                        match creneau{
                                            Some(cre) => cours_groupe = cre.cours_groupe.clone().unwrap_or(HashMap::new()),
                                            None => cours_groupe = HashMap::new(),
                                        };
                                        
                                        
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
                                            
                                            let mut prof = self.teachers.get(&creneau.unwrap().id_prof.unwrap()).unwrap();
                                            let mut classe = self.classe.get(&creneau.unwrap().id_classe.unwrap()).unwrap();
                                            let mut salle = self.salle.get(&creneau.unwrap().id_salle.unwrap()).unwrap();
                                            let mut matiere = self.matiere.get(&creneau.unwrap().id_matiere.unwrap()).unwrap();
                                            let mut groupe = self.groupe.get(&creneau.unwrap().id_groupe.unwrap()).unwrap();

                                            let mut salle_name = salle.get_name().clone();
                                            let mut prof_name = prof.get_name().clone();
                                            //variable temp en attendant juste pour visualiser les données lors des tests /////
                                            let mut affiche_liste_groupe: String = String::new();
                                            let mut affiche_liste_prof: String = String::new();
                                            let mut affiche_liste_salle: String = String::new();
                                            ////////////////////////////////////////////////////////////////////////////
                                            
                                            let mut groupe_name: String = String::new();
                                            let mut l_groupe: HashMap<usize,usize> = HashMap::new();
                                            let mut l_prof: HashSet<usize> = HashSet::new();
                                            
                                            let mut numero_groupe = 1;
                                            if cours_groupe.len() > 1 {
                                                groupe_name = format!("groupe {:}", groupe.get_name().to_string());
                                                
                                                //pour voir si les profs sont des profs différents ou non
                                                for (i,((id_groupe, id_prof), id_salle) ) in cours_groupe.iter().enumerate()
                                                {
                                                    l_prof.insert(*id_prof);
                                                } 
                                                


                                            }
                                            if l_prof.len() > 1{
                                                for (i,((id_groupe, id_prof), id_salle) ) in cours_groupe.iter().enumerate()
                                                {
                                                    if self.selected_prof_id == *id_prof {
                                                        salle_name = id_salle.to_string();
                                                        prof_name = self.teachers.get(id_prof).unwrap().get_name();
                                                        //groupe_name = groupe.get_name().to_string();
                                                        groupe_name = format!("groupe {:}",self.groupe.get(id_groupe).unwrap().get_name().to_string());   
                                                    }
                                                }
                                            }
                                            if groupe_name.len()> 1{
                                                /*for (i, id_g) in l_groupe.iter().enumerate().filter(|(i,id_g)|{ *id_g == groupe.get_id()}){
                                                    groupe_name = format!("groupe {:}", i + 1);
                                                }*/
                                                
                                                format!(" {:} \n {:} - {:} \n {:}\n {:}", prof_name, classe.get_name(),groupe_name, salle_name, matiere.get_name() )
                                            }else{
                                                format!(" {:} \n {:} \n {:} \n {:}", prof_name, classe.get_name(), salle_name, matiere.get_name() )
                                            }
                                            
                                        } else { 
                                            format!("{:} \n {:} \n {:} \n {:} ", " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string()) 
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
                let teacher_schedule = self.teachers.get(id_prof).unwrap().schedule.clone() ;
                let mut etat: Etat;
                self.planning_prof.insert((*id_prof, num_sem), Planning::new(enum_type_id::id_prof, *id_prof,num_sem,5,8));
                let planning = self.planning_prof.get_mut(&(*id_prof, num_sem)).unwrap();
                //dbg!(&planning);
                for ((id_jour, id_heure), creneau) in self.horaires.iter()
                //.filter(|((id_jour, id_heure), creneau)| { creneau.get_dispo() != TypeCreneau::Desactive })
                {
                    //dbg!((&id_prof, id_jour, id_heure));
                    match teacher_schedule.get(&(*id_jour, *id_heure)){
                        Some(creneau) => etat = creneau.etat,
                        None => etat = Etat::Indisponible,
                    };
                    let actif_ou_repas = creneau.get_dispo();
                    
                    if actif_ou_repas == TypeCreneau::Actif || actif_ou_repas == TypeCreneau::Repas {
                        planning.init_planning(*id_jour, *id_heure, actif_ou_repas, etat);
                    }
                }
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
                    if actif_ou_repas == TypeCreneau::Actif || actif_ou_repas == TypeCreneau::Repas {
                        planning.init_planning(*id_jour, *id_heure, actif_ou_repas, Etat::Disponible);
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
                    if actif_ou_repas == TypeCreneau::Actif || actif_ou_repas == TypeCreneau::Repas {
                        planning.init_planning(*id_jour, *id_heure, actif_ou_repas, Etat::Disponible);
                    }
                }
            }
        }
    }



    pub fn alim_creneau_a_placer(&mut self, en_groupe: bool){
        //construction de la liste des creneaux à placer ( liste des matieres par classe et le nombre d'heure de chacune)
        //self.liste_creneau_non_placer = HashMap::new();
        //self.liste_creneau_placer = HashMap::new();
        let mut liste_prof: HashMap<(usize,usize), HashSet<usize>> = HashMap::new();
        let mut liste_groupe: HashMap<(usize,usize), HashSet<(usize,usize)>> = HashMap::new();
        let mut liste_classe: HashMap<(usize,usize), HashSet<usize>> = HashMap::new();

        self.liste_creneau_a_placer = HashMap::new();
        for (_id, assignation) in self.assignement.iter(){
            let id_filiere = assignation.get_classe().get_filiere().get_id();
            let id_matiere = assignation.get_matiere().get_id();
            let id_classe = assignation.get_classe().get_id();
            let id_groupe = assignation.get_groupe().get_id();
            let id_prof = assignation.get_prof().get_id();

            match liste_prof.get_mut(&(id_classe,*id_matiere)){
                Some(profs) => 
                        {   
                            profs.insert(id_prof);
                        },
                None => {
                            let mut profs = HashSet::new();
                            profs.insert(id_prof);
                            liste_prof.insert((id_classe, *id_matiere), profs);
                        },
            };

            match liste_groupe.get_mut(&(id_classe,*id_matiere)){
                Some(groupes) => 
                        {   
                            groupes.insert((*id_groupe, id_prof));
                        },
                None => {
                            let mut groupes = HashSet::new();
                            groupes.insert((*id_groupe, id_prof));
                            liste_groupe.insert((id_classe, *id_matiere), groupes);
                        },
            };

            for ((id_classe_select, id_matiere_select, id_classe_participante), _) in self.selected_liste_inter_classe.iter()
            .filter(|((id_classe_select, id_matiere_select, id_classe_participante), _)| 
            {
                *id_classe_select == id_classe
                && *id_matiere_select == *id_matiere
            })
            {
                //dbg!(&id_classe_participante);
                match liste_classe.get_mut(&(id_classe,*id_matiere)){
                    Some(classes) => 
                            {   
                                classes.insert(*id_classe_participante);
                            },
                    None => {
                                let mut classes = HashSet::new();
                                classes.insert(*id_classe_participante);
                                liste_classe.insert((id_classe, *id_matiere), classes);
                            },
                };
            }
            // pour les cas qui ne sont pas en cours interclasse
            if liste_classe.get_mut(&(id_classe,*id_matiere)).is_none(){
                //println!("toto");
                let mut classes = HashSet::new();
                classes.insert(id_classe);
                liste_classe.insert((id_classe, *id_matiere), classes);          
            }
        }


        
        for (_id, assignation) in self.assignement.iter(){
            let id_filiere = assignation.get_classe().get_filiere().get_id();
            let id_matiere = assignation.get_matiere().get_id();
            let id_classe = assignation.get_classe().get_id();
            let id_groupe = assignation.get_groupe().get_id();
            let id_prof = assignation.get_prof().get_id();

            // à modifier pour prendre en compte les liste précédente pour remplacer les clé de liste creneau à placer
            for (_id_mat_prog, matiere_prog) in self.matiere_prog.iter()
            .filter(|(_id_s, mat_prog)| 
                {
                    mat_prog.get_semaine().get_filiere().get_id() == id_filiere 
                    && mat_prog.get_matiere().get_id() == id_matiere
                    && (( en_groupe && *mat_prog.get_en_groupe()) || (!en_groupe && !*mat_prog.get_en_groupe())) // ) voir si quand en groupe est à false, si on prend toutes les cours meme ceux en groupe dans le cas où certains n'auraient pas été placés
                })
            {
                let id_semaine = *matiere_prog.get_semaine().get_id();

                let mut classe: HashSet<usize> =  HashSet::new();
                let mut prof: HashSet<usize> =  HashSet::new();
                let mut groupe: HashSet<(usize,usize)> =  HashSet::new();

                //dbg!(&liste_classe);
                classe = liste_classe.get(&(id_classe,*id_matiere)).unwrap().clone();
                prof = liste_prof.get(&(id_classe,*id_matiere)).unwrap().clone();
                groupe = liste_groupe.get(&(id_classe,*id_matiere)).unwrap().clone();

                self.liste_creneau_a_placer.insert((id_classe, id_prof, *id_matiere, *id_groupe, id_semaine),(*matiere_prog.get_nb_heure(),*matiere_prog.get_duree_minimum(),*matiere_prog.get_duree_maximum(), classe, prof, groupe));
                
                //self.liste_creneau_a_placer.insert((id_classe,id_prof, *id_matiere, *id_groupe, id_semaine),(*matiere_prog.get_nb_heure(),*matiere_prog.get_duree_minimum(),*matiere_prog.get_duree_maximum()));
            }
        }

        
    }

    pub fn liste_semaines(&self) -> HashSet<usize> {
        
        let mut liste_semaine: HashSet<usize> = HashSet::new();
        for (_id, assignation) in self.assignement.iter(){
            let id_filiere = assignation.get_classe().get_filiere().get_id();
            let id_matiere = assignation.get_matiere().get_id();
            for (_id_mat_prog, matiere_prog) in self.matiere_prog.iter()
                                                                    .filter(|(_id_s, mat_prog)| 
                                                                        {mat_prog.get_semaine().get_filiere().get_id() ==id_filiere 
                                                                        && mat_prog.get_matiere().get_id() == id_matiere})
            {
                let id_semaine = *matiere_prog.get_semaine().get_id();
                liste_semaine.insert(id_semaine);  
            }   
        }
        liste_semaine                                                             
    }

    pub fn liste_cours_en_groupe_prof_different(&self) ->  HashMap<(usize,usize), bool> {
        let mut liste_cours_en_groupe_prof_different: HashMap<(usize,usize), bool> = HashMap::new();
        let mut map_prof: HashMap<(usize,usize),usize> = HashMap::new();
        
        for (_id, assignation) in self.assignement.iter(){
            //let id_filiere = assignation.get_classe().get_filiere().get_id();
            let id_matiere = assignation.get_matiere().get_id();
            let id_classe = assignation.get_classe().get_id();
            //let id_groupe = assignation.get_groupe().get_id();
            let id_prof = assignation.get_prof().get_id();

            match map_prof.get(&(id_classe, *id_matiere)) {
                Some(prof) => {
                                        if *prof != id_prof {
                                            liste_cours_en_groupe_prof_different.insert((id_classe, *id_matiere), true);
                                        }
                                      },
                None => {
                            map_prof.insert((id_classe, *id_matiere), id_prof);
                        },
            };
        }
        liste_cours_en_groupe_prof_different
    }


    pub fn filtrage_cours_non_recurent(&mut self, liste_semaine: HashSet<usize>){

        for (_id, assignation) in self.assignement.iter(){
            let id_filiere = assignation.get_classe().get_filiere().get_id();
            let id_matiere = assignation.get_matiere().get_id();
            let id_classe = assignation.get_classe().get_id();
            let id_groupe = assignation.get_groupe().get_id();
            let id_prof = assignation.get_prof().get_id();
            
            let mut a_filtrer = false;
            for semaine in liste_semaine.iter(){
                if self.liste_creneau_a_placer.get(&(id_classe,id_prof, *id_matiere, *id_groupe, *semaine)).is_none(){
                    a_filtrer = true;
                }
            }
            if a_filtrer {
                for semaine in liste_semaine.iter(){
                    self.liste_creneau_a_placer.remove(&(id_classe,id_prof, *id_matiere, *id_groupe, *semaine));
                }
            }
        }

    }
   

    /*pub fn alim_creneau_a_placer(&mut self){
        //construction de la liste des creneaux à placer ( liste des matieres par classe et le nombre d'heure de chacune)
        //self.liste_creneau_non_placer = HashMap::new();
        //self.liste_creneau_placer = HashMap::new();
        self.liste_creneau_a_placer = HashMap::new();
        for (_id, assignation) in self.assignement.iter(){
            let id_filiere = assignation.get_classe().get_filiere().get_id();
            let id_matiere = assignation.get_matiere().get_id();
            let id_classe = assignation.get_classe().get_id();
            let id_groupe = assignation.get_groupe().get_id();
            let id_prof = assignation.get_prof().get_id();

            for (_id_mat_prog, matiere_prog) in self.matiere_prog.iter()
                                                                    .filter(|(_id_s, mat_prog)| 
                                                                        {mat_prog.get_semaine().get_filiere().get_id() ==id_filiere 
                                                                        && mat_prog.get_matiere().get_id() == id_matiere
                                                                        && !*mat_prog.get_en_groupe()
                                                                    })
            {
                let id_semaine = *matiere_prog.get_semaine().get_id();
                self.liste_creneau_a_placer.insert((id_classe,id_prof, *id_matiere, *id_groupe, id_semaine),(*matiere_prog.get_nb_heure(),*matiere_prog.get_duree_minimum(),*matiere_prog.get_duree_maximum()));
            }
        }
    }*/




    pub fn create_planning(&mut self) {
        
        self.liste_creneau_placer = HashMap::new();
        self.liste_creneau_non_placer = HashMap::new();
        let mut liste_semaine: HashSet<usize>;
        self.alim_nb_semaine_max();
        liste_semaine = self.liste_semaines();
        self.init_planning();
        
        let mut liste_cours_en_groupe_prof_different: HashMap<(usize,usize), bool> = HashMap::new();
        //on place d'abord les cours en groupe qui reviennent chaque semaine
        self.alim_creneau_a_placer(true);  // --> diviser en groupe avec le meme prof et groupe profs différents
        self.filtrage_cours_non_recurent(liste_semaine.clone()); 
        liste_cours_en_groupe_prof_different = self.liste_cours_en_groupe_prof_different();
        //dbg!(&liste_cours_en_groupe_prof_different);
        self.place_les_creneaux(true, &mut liste_semaine, true, &liste_cours_en_groupe_prof_different); //-> modifier la recherche de la durée du creneau si le prof est le même pour chaque groupe
                                    // --> modifier la recherche des creneaux disponibles si les profs sont différents pour chaque groupe
                                    // --> + cours récurent, donc dès qu'une occurrence est trouvée, la placer pour toutes les semaines et ne pas traiter les autres semaine dans la boucle
        //on place d'abord les cours restant en groupe et non en groupe qui reviennent chaque semaine
        self.alim_creneau_a_placer(false);
        self.filtrage_cours_non_recurent(liste_semaine.clone());
        self.place_les_creneaux(true, &mut liste_semaine, false, &liste_cours_en_groupe_prof_different);


        //on place d'abord les cours en groupe qui ne reviennent pas chaque semaine
        self.alim_creneau_a_placer(true);
        self.place_les_creneaux(false, &mut liste_semaine, true, &liste_cours_en_groupe_prof_different);

        //on place d'abord les cours restant en groupe et non en groupe qui ne reviennent pas chaque semaine
        self.alim_creneau_a_placer(false);
        //self.filtrage_cours_non_recurent(liste_semaine.clone());
        self.place_les_creneaux(false, &mut liste_semaine, false, &liste_cours_en_groupe_prof_different);
        //debut placement des creneaux
        
        //let mut id_type_salle: usize;

        if self.liste_creneau_non_placer.len() == 0{
            self.generation_reussi = true;
        }
        

        

    }

    pub fn place_les_creneaux(&mut self, recurrent: bool, liste_semaine: &mut HashSet<usize>, en_groupe: bool, liste_cours_en_groupe_prof_different: &HashMap<(usize,usize), bool>){

        let mut id_type_salle: usize;
        

        //ajout utilisation de la liste des classes
        //ajout utilisation de la liste des profs
        //ajout utilisation de la liste des groupes


        let mut rng_1 = thread_rng();
        let mut keys_a_placer: Vec<_> = self.liste_creneau_a_placer.iter()
                                        .filter(|((id_classe, id_prof, id_matiere, id_groupe, id_semaine), _ )| 
                                            { self.liste_creneau_placer.get(&(*id_classe, *id_prof, *id_matiere, *id_groupe, *id_semaine)).is_none()}
                                        )
                                        .collect();
        keys_a_placer.shuffle(&mut rng_1);
                        
        for ((id_classe, id_prof, id_matiere, id_groupe, id_semaine), (nb_heure, duree_min, duree_max, liste_classe, liste_prof, liste_groupe)) in keys_a_placer
        {
            //si on est dans le cas ou on place les cours recurrent et que ce cours a deja etait placé pour une des semaines, alors on passe au prochain cours
            if recurrent && !self.liste_creneau_placer.get(&(*id_classe, *id_prof, *id_matiere, *id_groupe, *id_semaine)).is_none(){
                continue;
            }

            if liste_prof.len() > 1 {
                let mut creneau_deja_place_via_autre_prof = false;
                for prof in liste_prof.iter(){
                    if self.liste_creneau_placer.contains_key(&(*id_classe, *prof, *id_matiere, *id_groupe, *id_semaine)){
                        creneau_deja_place_via_autre_prof = true;
                        break;
                    }
                }
                if creneau_deja_place_via_autre_prof {
                    continue;
                }
            }

            let mut nb_heure_restant = *nb_heure;
            let mut nb_max_passage = 0;
            let mut creneau_dispo:(usize,usize,usize,bool) = (1,1,1,false);
            let mut creneau_dispo_salle: (usize,usize,bool) = (0,0,false); 
            let mut id_salle: usize = 0;
            
            match self.matiere.get(id_matiere){
                Some(matiere) => id_type_salle = matiere.get_room_type().get_id(),
                None => {   
                            break; 
                        },
            };

            let mut nb_prof = 0;
            let mut l_prof: HashSet<usize> = HashSet::new();
            for (groupe, prof) in liste_groupe.iter()
            {
                l_prof.insert(*prof);
            }
            nb_prof = l_prof.len();
            
            if nb_prof == 2 {
                dbg!(&liste_prof);
            }
            let mut liste_planning :  HashMap<(usize, usize, usize), Planning> = HashMap::new();      //let liste_planning_groupe: HashMap<usize, &Planning> = HashMap::new();
                for prof in liste_prof.iter() {
                    match self.planning_prof.get(&(*prof, *id_semaine)) {
                        Some(planning) => {liste_planning.insert((1,*prof, *id_semaine),planning.clone())},
                        None => continue
                    };
                }
                for classe in liste_classe.iter() {
                    match self.planning_classe.get(&(*classe, *id_semaine)) {
                        Some(planning) => {liste_planning.insert((2,*classe, *id_semaine),planning.clone())},
                        None => continue
                    };
                }

            while nb_heure_restant > 0 {
                //dbg!(&nb_max_passage);
                if nb_max_passage > 4000 {
                    self.generation_reussi = false;
                    self.liste_creneau_non_placer.insert((*id_classe, *id_prof, *id_matiere, *id_groupe, *id_semaine), (nb_heure_restant, liste_prof.clone(), liste_classe.clone(), liste_groupe.clone()));   
                    break;
                }
                //trouve un creneau disponible pour le prof et la classe
                  
                creneau_dispo = self.trouve_creneau_dispo( *id_semaine, *id_prof,  *id_classe, *duree_min, *duree_max, &nb_heure_restant, &liste_planning, liste_groupe);
                
                //verification qu'une salle est disponible
                let mut id_salle: Vec<usize> = Vec::new();
                let mut dispo_salle: bool ;
                (id_salle, dispo_salle) = self.get_dispo_salle(&creneau_dispo, &id_type_salle, &id_semaine, nb_prof);

                if !dispo_salle {
                    nb_max_passage += 1;
                    continue;
                }else{
                    //CRENEAU TROUVE
                    if id_salle.len() == 0 && dispo_salle{
                        println!("ERREUR!!!!!!");
                        dbg!(&dispo_salle);
                        dbg!(&id_salle);
                        dbg!(&creneau_dispo);
                        dbg!(&(&id_classe, &id_prof, &id_matiere, &id_groupe, &id_semaine));
                    }

                    nb_heure_restant -= creneau_dispo.2;
                    
                    for semaine in liste_semaine.iter()
                    .filter(|semaine| 
                        { 
                            recurrent || (!recurrent && *semaine == id_semaine) //si cours recurrent on insert dans toutes les semaines, sinon seulement dans la semaine selectionnée
                        })
                    {
                        
                        /*let planning_prof = self.planning_prof.get_mut(&(*id_prof, *semaine)).unwrap();
                        let planning_classe = self.planning_classe.get_mut(&(*id_classe, *semaine)).unwrap();*/

                        
                        
                        for ((source, id, _semaine_planning), planning) in liste_planning.iter_mut()
                        {
                            let mut cours_groupe:HashMap<(usize,usize), usize> = HashMap::new();
                            if nb_prof > 1{
                                
                                
                                for i in 0..creneau_dispo.2 {
                                    let mut y: usize = 0;
                                    for (groupe, prof) in liste_groupe.iter(){
                                        planning.set_creneau(creneau_dispo.0, creneau_dispo.1 + i, *prof, *id_classe, *groupe, id_salle[y], *id_matiere);
                                        cours_groupe.insert((*groupe, *prof),id_salle[y]);
                                        y += 1;
                                        //set_cours_groupe
                                    }
                                    planning.set_creneau_cours_multiple(creneau_dispo.0, creneau_dispo.1 + i,cours_groupe.clone());
                                }
                            } else{
                                
                                for i in 0..creneau_dispo.2 {
                                    let mut y: usize = 0;
                                    for (groupe, prof) in liste_groupe.iter(){
                                        y += 1;
                                        planning.set_creneau(creneau_dispo.0, creneau_dispo.1 + i, *id_prof, *id_classe, *id_groupe, id_salle[0], *id_matiere);
                                        cours_groupe.insert((*groupe, *prof),id_salle[0]);
                                    }
                                    planning.set_creneau_cours_multiple(creneau_dispo.0, creneau_dispo.1 + i,cours_groupe.clone());
                                }
                                
                            }
                            
                        }

                        for salle in id_salle.iter(){
                            let planning_salle = self.planning_room.get_mut(&(*salle, id_type_salle, *semaine)).unwrap();
                            for i in 0..creneau_dispo.2 {
                                planning_salle.set_creneau(creneau_dispo.0, creneau_dispo.1 + i, *id_prof, *id_classe, *id_groupe, *salle, *id_matiere);     
                            } 
                        }
                          
                        
                        if nb_heure_restant == 0 {
                            if nb_prof > 1 {
                                for (groupe, prof) in liste_groupe.iter(){
                                    self.liste_creneau_placer.insert((*id_classe, *prof, *id_matiere, *groupe, *semaine), (*nb_heure, liste_prof.clone(), liste_classe.clone(), liste_groupe.clone()));   
                                }
                            } else{
                                self.liste_creneau_placer.insert((*id_classe, *id_prof, *id_matiere, *id_groupe, *semaine), (*nb_heure, liste_prof.clone(), liste_classe.clone(), liste_groupe.clone()));                        
                            }
                        }
                    }   
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

            for semaine in liste_semaine.iter()
                    .filter(|semaine| 
                        { 
                            recurrent || (!recurrent && *semaine == id_semaine) //si cours recurrent on insert dans toutes les semaines, sinon seulement dans la semaine selectionnée
                        })
                    {
                for ((source, id, semaine_planning), planning) in liste_planning.iter() 
                {
                    if *source == 1 {
                        self.planning_prof.insert((*id, *semaine), planning.clone()) ;
                    }else{
                        self.planning_classe.insert((*id, *semaine), planning.clone()) ;
                    }
                }
            }
        }
    }


   

    pub fn get_dispo_salle(&self, creneau_dispo: &(usize,usize,usize,bool), id_type_salle:&usize, id_semaine: &usize, nb_prof: usize) -> (Vec<usize>,bool) {
        
        let mut creneau_dispo_salle: (usize,usize,bool) = (creneau_dispo.0, creneau_dispo.1, false);  
        let mut id_salle: Vec<usize> = Vec::new();
        
        for (id_room, _room) in self.salle.iter().filter(|(_id, room)|{room.get_room_type().get_id() == *id_type_salle}){
            creneau_dispo_salle.2 = false;
            let mut dispo = true;
            if id_salle.len() < nb_prof {
                
                for salle in id_salle.iter(){
                    if salle == id_room {
                        dispo = false;
                        break;
                    }
                }
                if dispo {
                    for i in 0..creneau_dispo.2 {
                        match self.planning_room.get(&(*id_room, *id_type_salle, *id_semaine)) {
                            Some(planning_room) => {  
                                                                creneau_dispo_salle = planning_room.get_verif_creneau(creneau_dispo.0, creneau_dispo.1 + i);
                                                                
                                                                if !creneau_dispo_salle.2{
                                                                    dispo = false;
                                                                    break;
                                                                }
                                                            },
                            None => {dispo = false; creneau_dispo_salle.2 = false; break},
                        }
                    }
                }
                if dispo {
                    id_salle.push(*id_room);
                    if id_salle.len() == nb_prof {
                        break;
                    }
                }
                
            } else {
                break;
            }
        }
        (id_salle,creneau_dispo_salle.2)
    }


    pub fn trouve_duree_creneau(&self, duree_max: usize, duree_min: usize, planning: &Planning,  jour: &usize, heure: &usize) -> (usize,usize,usize,bool) {
        let mut creneau_trouve: bool = false;
        let mut creneau_dispo = (1,1,1,false);
        let mut new_duree_max = duree_max.clone();
        
        while new_duree_max >= duree_min {
            for i in 0..new_duree_max { 
                creneau_trouve = false;
                if let Some(new_creneau) = planning.get_creneau(*jour,*heure + i){
                    let creneau = new_creneau.clone();
                    let mut actif = 
                            match creneau.preference.unwrap() {
                                Etat::Disponible => true,
                                Etat::Preference => true,
                                Etat::Indisponible => false,
                                _ => false,
                            };
                    
                    let repas =
                    match creneau.actif_ou_repas.unwrap() {
                        TypeCreneau::Actif => false,
                        TypeCreneau::Repas => true,
                        TypeCreneau::Desactive => false,
                        _ => false,
                    };
                    if repas{
                        actif = false;
                        for h in 0..*heure{
                            if let Some(creneau_repas) = planning.get_creneau(*jour,h){
                                let creneau = creneau_repas.clone();
                                if creneau.clone().actif_ou_repas.unwrap() == TypeCreneau::Repas && creneau.get_prof().is_none(){
                                    actif = true;
                                    break;
                                }
                            }
                        }
                    }
                    
                    if new_creneau.get_prof().is_none() && actif{
                        creneau_trouve = true;
                    }      
                }
                if !creneau_trouve {
                    new_duree_max -= duree_min;
                    break;
                }
            }

            if creneau_trouve {
                creneau_dispo = (*jour, *heure, new_duree_max, true);
                break;
            }else if duree_max == duree_min{
                break;
            }
        }
        creneau_dispo
        
    }


    pub fn trouve_creneau_dispo(&self, id_semaine: usize, id_prof: usize,  id_classe: usize, duree_min:usize, duree_max:usize, nb_heure_restant:&usize, liste_planning: &HashMap<(usize,usize, usize), Planning>, liste_groupe: &HashSet<(usize,usize)>) -> (usize,usize,usize,bool){
        let mut creneau_dispo: (usize,usize,usize,bool) = (1,1,1,false);
        
        //let mut liste_planning_prof: HashMap<usize, &Planning> = HashMap::new();
        //let mut liste_planning_classe: HashMap<usize, &Planning> = HashMap::new();
    
        let mut rng = thread_rng();
        let mut keys: Vec<_> = self.horaires.keys().collect();
        keys.shuffle(&mut rng);

        for (jour, heure) in  keys{
            //creneau_dispo = (1,1,1,false);
            let mut new_duree_max: usize;
            if nb_heure_restant > &duree_max{
                new_duree_max = duree_max;
            }else{
                new_duree_max = *nb_heure_restant;
            }

            ////verifie si tous les profs et les classes sont disponibles à ce créneau////////////////////////////////
            let mut dispo: bool = true;
            for (_, planning) in liste_planning.iter() {
                if let Some(creneau) = planning.get_creneau(*jour, *heure){
                    let actif = 
                        match creneau.clone().preference.unwrap() {
                            Etat::Disponible => true,
                            Etat::Preference => true,
                            Etat::Indisponible => false,
                            _ => false,
                        };
                    if !actif {
                        dispo = false
                    }
                    if !creneau.get_prof().is_none(){
                        dispo = false
                    }
                }else{
                    dispo = false;
                }
            }
            if !dispo {
                continue;
            }
            ///////////// fin verification dispo //////////////////////////////

            for ((_, _id, _semaine), planning) in liste_planning.iter() {
                creneau_dispo = self.trouve_duree_creneau(new_duree_max, duree_min, planning, jour, heure);
                if !creneau_dispo.3 {
                    break;
                }
                new_duree_max = creneau_dispo.2; // recupere la valeur de creneau max trouvé qui respecte chaque planning controlé
            }

            if creneau_dispo.3 {
                break;
            }
        }
        creneau_dispo
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
    /*pub fn get_nb_creneau_restant(&self) -> usize {
        let mut count: usize = 0;
        for ((id_classe, id_prof, id_matiere, id_groupe, id_semaine), (nb_heure, _duree_mini, _duree_max)) in self.liste_creneau_a_placer.iter(){
            match self.liste_creneau_placer.get(&(*id_classe, *id_prof, *id_matiere, *id_groupe, *id_semaine)){
                Some(nb_heure_restant) => count += nb_heure_restant, // nb_heure_restant peut etre égale à 0,
                None => count += nb_heure, //aucun creneau de ce cour n'a été placé,
            }
        }
        count
    }*/

}