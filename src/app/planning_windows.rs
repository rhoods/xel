use eframe::egui;
use std::sync::{Arc, Mutex};

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
            });
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
            let mut creneau_dispo: (usize,usize,bool); 
            let mut creneau_dispo_salle: (usize,usize,bool) = (0,0,false); 
            let mut id_salle: usize = 0;
            match self.matiere.get(id_matiere){
                Some(matiere) => id_type_salle = matiere.get_room_type().get_id(),
                None => { println!(" id matiere non trouvé dans liste des matiere"); dbg!(&self.matiere); dbg!(&self.matiere); break; },
            };
            

            while nb_heure_restant > 0 {
                //dbg!(&nb_max_passage);
                if nb_max_passage > 20 {
                    break;
                }
                //dbg!(&id_prof);
                //dbg!(&id_semaine);
                //dbg!(&self.planning_prof);
                //dbg!(&self.planning_prof.get(&(*id_prof, *id_semaine)));
                match self.planning_prof.get(&(*id_prof, *id_semaine)){
                    Some(planning) => {
                                                    //planning_prof = planning; 
                                                    creneau_dispo = planning.get_verif_random_creneau();
                                                    if creneau_dispo.2{
                                                        dbg!(&creneau_dispo);
                                                    }
                                                 },
                    None => {nb_max_passage += 1;
                             continue;
                            },
                };
                if !creneau_dispo.2 {
                    nb_max_passage += 1;
                    continue;
                }

                match self.planning_classe.get(&(*id_classe, *id_semaine)){
                    Some(planning) => {
                                                    //planning_classe = planning;
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
                    planning_prof.set_creneau(creneau_dispo.0, creneau_dispo.1, *id_prof, *id_classe, id_salle);

                    let planning_classe = self.planning_classe.get_mut(&(*id_classe, *id_semaine)).unwrap();
                    planning_classe.set_creneau(creneau_dispo.0, creneau_dispo.1, *id_prof, *id_classe, id_salle);

                    let planning_salle = self.planning_room.get_mut(&(id_salle, id_type_salle, *id_semaine)).unwrap();
                    planning_salle.set_creneau(creneau_dispo.0, creneau_dispo.1, *id_prof, *id_classe, id_salle);
                    
                    nb_heure_restant -= 1;
                }


                println!("{:}, {:}, {:}", creneau_dispo.0, creneau_dispo.1, creneau_dispo.2);
                
                
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