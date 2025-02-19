use eframe::egui;
use std::sync::{Arc, Mutex};

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


#[derive(Clone, Debug)]
pub struct PlanningWindow {
    selected_semaine_id: usize,
    selected_prof_id: usize,
    generation_reussi: bool,
    nb_semaine_max: usize,
    nb_semaine_max_par_filiere: HashMap<usize, usize>,// (id_filiere, nb_semaine)
    liste_creneau_a_placer: HashMap<(usize,usize,usize,usize,usize), usize>, //<(id_classe, id_prof,id_matiere, id_groupe,id_semaine), nb_heure>
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

}

impl  Default for PlanningWindow  {
    fn default() -> Self {
        Self {
            selected_semaine_id: 0,
            selected_prof_id: 0,
            generation_reussi: true,
            nb_semaine_max: 0,
            nb_semaine_max_par_filiere: HashMap::new(),
            liste_creneau_a_placer: HashMap::new(),
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
        }
    }
}

impl PlanningWindow {
    pub fn charge(&mut self, salle:HashMap<usize, Room>,  semaine: HashMap<(usize,usize), Arc<Semaine>>, classe: HashMap<usize, Arc<Classe>>, filiere: HashMap<usize, Arc<Filiere>>, matiere:HashMap<usize, Arc<Matiere>>,   matiere_prog: HashMap<usize, Arc<MatiereProg>>, matiere_inter_classe: HashMap<usize, Arc<MatiereInterClasse>>, teachers: HashMap<usize, Teacher>, groupe: HashMap<usize, Arc<Groupe>>,  assignement :HashMap<usize, Arc<Assignation>>) {
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
    }

    pub fn build(&mut self, ctx: &egui::Context,) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.horizontal(|ui| { 
                    if ui.button("Générer plannings").clicked(){
                        self.create_planning();
                        dbg!(&self.generation_reussi);
                        if self.generation_reussi{
                            print!("Mission Reussi!");
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

    pub fn create_planning(&mut self) {

       //recup nb_semaine_max et nb_semaine_max par filiere
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
        //dbg!(&self.nb_semaine_max);
        
        //init des listes de planning de chaque prof, classe et salle
        //dbg!(&self.teachers);
        for (id_prof, _prof) in self.teachers.iter(){
            for num_sem in 0..self.nb_semaine_max {
                //dbg!(id_prof);
                //dbg!(num_sem);
                self.planning_prof.insert((*id_prof, num_sem), Planning::new(enum_type_id::id_prof, *id_prof,num_sem,5,8));
                let planning = self.planning_prof.get_mut(&(*id_prof, num_sem)).unwrap();
                //dbg!(&planning);
                planning.init_planning(5, 8);
                //dbg!(&planning);
            }
        }
        for (id_classe, _classe) in self.classe.iter(){
            for num_sem in 0..self.nb_semaine_max {
                self.planning_classe.insert((*id_classe, num_sem), Planning::new(enum_type_id::id_classe, *id_classe,num_sem,5,8));
                let planning = self.planning_classe.get_mut(&(*id_classe, num_sem)).unwrap();
                planning.init_planning(5, 8);
            }
        }
        for (id_room, room) in self.salle.iter(){
            for num_sem in 0..self.nb_semaine_max {
                let id_type_salle = room.get_room_type().get_id();
                self.planning_room.insert((*id_room, id_type_salle, num_sem), Planning::new(enum_type_id::id_salle, *id_room,num_sem,5,8));
                let planning = self.planning_room.get_mut(&(*id_room, id_type_salle, num_sem)).unwrap();
                planning.init_planning(5, 8);
            }
        }
        

        //construction de la liste des creneaux à placer ( liste des matieres par classe et le nombre d'heure de chacune)
        for (id, assignation) in self.assignement.iter(){
            let id_filiere = assignation.get_classe().get_filiere().get_id();
            let id_matiere = assignation.get_matiere().get_id();
            let id_classe = assignation.get_classe().get_id();
            let id_groupe = assignation.get_groupe().get_id();
            let id_prof = assignation.get_prof().get_id();

            for (id_mat_prog, matiere_prog) in self.matiere_prog.iter()
                                                                    .filter(|(id_s, mat_prog)| 
                                                                        {mat_prog.get_semaine().get_filiere().get_id() ==id_filiere 
                                                                        && mat_prog.get_matiere().get_id() == id_matiere})
            {
                let id_semaine = *matiere_prog.get_semaine().get_id();
                self.liste_creneau_a_placer.insert((id_classe,id_prof, *id_matiere, *id_groupe, id_semaine),*matiere_prog.get_nb_heure());
            }
        }
        //dbg!(&self.liste_creneau_a_placer);

        //debut placement des creneaux
        let mut id_type_salle: usize;
        for ((id_classe, id_prof, id_matiere, id_groupe, id_semaine), nb_heure) in self.liste_creneau_a_placer.iter(){
            let mut nb_heure_restant = *nb_heure;
            let mut nb_max_passage = 0;
            let mut creneau_dispo: (usize,usize,bool) = (0,0,false); 
            let mut creneau_dispo_salle: (usize,usize,bool) = (0,0,false); 
            let mut id_salle: usize = 0;
            
            match self.matiere.get(id_matiere){
                Some(matiere) => id_type_salle = matiere.get_room_type().get_id(),
                None => {   
                            println!(" id matiere non trouvé dans liste des matiere"); 
                            break; 
                        },
            };

            while nb_heure_restant > 0 {
                //dbg!(&nb_max_passage);
                if nb_max_passage > 20 {
                    break;
                }


                //essaye de placer un créneaux sur l'heure suivante
                if creneau_dispo.2 {

                    match self.planning_prof.get(&(*id_prof, *id_semaine)){
                        Some(planning) => {
                                                        if (creneau_dispo.1 + 1) >= *planning.get_nb_heure(){
                                                            creneau_dispo = (0,0,false);
                                                        }else{
                                                            creneau_dispo.1 += 1; //on verifie si l'heure suivante est disponible
                                                            creneau_dispo = planning.get_verif_creneau(creneau_dispo.0, creneau_dispo.1);
                                                        }
                                                        
                                                    },
                        None => {nb_max_passage += 1;
                                continue;
                                },
                    };
                }else{
                    // verifie dispo prof
                    match self.planning_prof.get(&(*id_prof, *id_semaine)){
                        Some(planning) => {
                                                        //planning_prof = planning; 
                                                        creneau_dispo = planning.get_verif_random_creneau();
                                                    },
                        None => {nb_max_passage += 1;
                                continue;
                                },
                    };
                }
                    if !creneau_dispo.2 {
                        nb_max_passage += 1;
                        continue;
                    }
                
                // verifie dispo classe
                match self.planning_classe.get(&(*id_classe, *id_semaine)){
                    Some(planning) => {
                                                    
                                                    creneau_dispo = planning.get_verif_creneau(creneau_dispo.0, creneau_dispo.1);
                                                },
                    None => {nb_max_passage += 1; continue;},
                };
                if !creneau_dispo.2 {
                    nb_max_passage += 1;
                    continue;
                }

                for (id_room, room) in self.salle.iter().filter(|(id, room)|{room.get_room_type().get_id() == id_type_salle}){
                    if !creneau_dispo_salle.2{
                        match self.planning_room.get(&(*id_room, id_type_salle, *id_semaine)) {
                            Some(planning_room) => {id_salle = *id_room;   creneau_dispo_salle = planning_room.get_verif_creneau(creneau_dispo.0, creneau_dispo.1);},
                            None => continue,
                        }
                    }
                }
                if !creneau_dispo_salle.2 {
                    nb_max_passage += 1;
                    continue;
                }else{
                    //CRENEAU TROUVE
                    let planning_prof = self.planning_prof.get_mut(&(*id_prof, *id_semaine)).unwrap();
                    planning_prof.set_creneau(creneau_dispo.0, creneau_dispo.1, *id_prof, *id_classe, id_salle, *id_matiere);
                    let planning_classe = self.planning_classe.get_mut(&(*id_classe, *id_semaine)).unwrap();
                    planning_classe.set_creneau(creneau_dispo.0, creneau_dispo.1, *id_prof, *id_classe, id_salle, *id_matiere);
                    let planning_salle = self.planning_room.get_mut(&(id_salle, id_type_salle, *id_semaine)).unwrap();
                    planning_salle.set_creneau(creneau_dispo.0, creneau_dispo.1, *id_prof, *id_classe, id_salle, *id_matiere);
                    
                    nb_heure_restant -= 1;
                }
              
                
                nb_max_passage += 1;
                
            }

            if nb_heure_restant > 0{
                self.generation_reussi = false;
            }
            
        }
        /*
        Partir de assignation et matiere prog
        pour déterminer pour chaque prof et classe le nombre de matiere et d'heure à placer

        peut etre faire un nouveau type qui mélange les deux avec les données de l'assignation + le nb d'heure qui diminura au fur et à mesure que l'on place les créneaux
        puis qui augmentera si la solution optimal n'est pas trouvé des le départ et qu'on récupére certains cours des plannings généré pour les regénéré autrement partiellement (algo génétique ou autre)

        boucle sur cette nouvelle structure (assignement + nb heure)
        à chaque occurrence on essaye d'insérer dans les créneaux du prof concerné et dans les créneaux de la classe concerné
        si créneaux vide pour les deux, alors on insert
        si tout placé ok sinon nouvelle gen?

        au début alimenté les creneaux du prof en fonction de la saisie de ses disponobilitées
        + complexifier verif en fonction de si le groupe et en cours, si les deux groupes ont le même prof, si le cours est interclasse, ajouter dispo des classes dans la verif
        + verid dispo salle du type nécessaire pour la matiere. (faire un planning pour chaque salle ?)
         */
        


    }


}