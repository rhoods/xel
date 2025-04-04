use eframe::egui::{self, Label, Response, pos2, vec2, Rect};
use std::cmp;
use std::sync::{Arc, Mutex};
use egui::{Context, Ui, Color32, Align2, Frame, Vec2,Rounding, Stroke};
use std::collections::{HashMap, HashSet};
use crate::struc::assignation::{Groupe, Assignation};
use crate::struc::matiere::Matiere;
use crate::struc::programme::{MatiereProg, Semaine, MatiereInterClasse, OptionProgramme}; 
//use crate::struc::filiere::Filiere;
//use crate::struc::classe::Classe;

use crate::app::filiere_window::{Filiere, Classe};
use crate::app::room_window::RoomType;


#[derive(Clone, Debug)]
pub struct ProgrammeWindow {
 
    //programmes:   HashMap<usize, Arc<Programme>>
    id_groupe: usize,
    new_nb_groupe: String,
    nb_groupe: HashMap<usize,Option<usize>>, 
    select_matiere_prog_remove_id: Option<usize>,
    selected_all: HashMap<usize,bool>,
    selected_all_classe: HashMap<usize,bool>,
    selected_classe: HashMap<(usize,usize), usize>, 
    selected_semaine_onglet: HashMap<usize, usize>, //id_filiere, i
    selected_semaines: HashMap<(usize,usize), usize>,//HashSet<usize>, //Vec<Arc<Semaine>>,
    semaines:   HashMap<(usize,usize), Arc<Semaine>>, //(id_filiere, i)
    id_matiere_prog: usize,
    matiere_prog:   HashMap<usize, Arc<MatiereProg>>, //
    matiere_inter_classe:   HashMap<usize, Arc<MatiereInterClasse>>,

    groupe: HashMap<usize, Arc<Groupe>>,
    filieres:  HashMap<usize, Arc<Filiere>>,
    classes:  HashMap<usize, Arc<Classe>>,
    matieres: HashMap<usize, Arc<Matiere>>,
    salles_type: HashMap<usize, Arc<RoomType>>,
    assignement :HashMap<usize, Arc<Assignation>>,
    //new_matiere: String,

    new_duree_minimum: HashMap<usize,String>,
    duree_minimum: HashMap<usize, Option<usize>>,
    new_duree_maximum: HashMap<usize,String>,
    duree_maximum: HashMap<usize, Option<usize>>,

    new_nb_heure: HashMap<usize,String>,
    nb_heure: HashMap<usize, Option<usize>>,
    new_nb_sem: String,
    nb_sem: HashMap<usize, Option<usize>>,
    nb_sem_deja_valid: HashSet<usize>,

    selected_filiere_id: usize, 

    selected_en_groupe: HashMap<usize,bool>,
    selected_en_groupe_interclasse: HashMap<usize,bool>,
    liste_selected_matiere: HashMap<usize,Arc<Matiere>>,
    selected_matiere: String,
    ajout_matiere:bool,

    selected_option: String,
            
    liste_selected_option: HashMap<usize,Arc<OptionProgramme>>,

    nom_option: HashMap<(usize,String), String>,     
    new_nom_option:   HashMap<usize, String>,  
    liste_options: HashMap<usize, Arc<OptionProgramme>>,
    id_option: usize,
    show_option_window: bool,
    liste_selected_option_par_matiere: HashMap<(usize,usize), Arc<OptionProgramme>>, //id_filiere, id_matiere
    select_option_remove_id: Option<usize>,

}

impl  Default for ProgrammeWindow  {
    fn default() -> Self {
        Self {
            id_groupe: 0,
            new_nb_groupe:String::new(),
            nb_groupe: HashMap::new(),
            select_matiere_prog_remove_id: None,
            selected_all: HashMap::new(), //false,
            selected_all_classe: HashMap::new(),
            selected_classe: HashMap::new(),
            selected_semaine_onglet: HashMap::new(),
            selected_semaines: HashMap::new(),
            matieres: HashMap::new(),

            //programmes: HashMap::new(),// HashMap::new()
            groupe: HashMap::new(),
            semaines: HashMap::new(),
            id_matiere_prog:0,
            matiere_prog: HashMap::new(),
            matiere_inter_classe: HashMap::new(),
            filieres: HashMap::new(),
            classes:  HashMap::new(),
            salles_type: HashMap::new(),
            assignement :HashMap::new(),

            new_duree_minimum: HashMap::new(),
            duree_minimum: HashMap::new(),
            new_duree_maximum: HashMap::new(),
            duree_maximum: HashMap::new(),
            
            new_nb_heure: HashMap::new(),
            nb_heure: HashMap::new(),
            new_nb_sem: String::new(),
            nb_sem: HashMap::new(),
            nb_sem_deja_valid: HashSet::new(),

            selected_filiere_id: 0,

            selected_en_groupe: HashMap::new(), //false,
            selected_en_groupe_interclasse: HashMap::new(), //false,
            liste_selected_matiere: HashMap::new(),
            selected_matiere: String::new(),

            selected_option: String::new(),
            
            liste_selected_option: HashMap::new(),
            liste_selected_option_par_matiere : HashMap::new(),

            ajout_matiere: false,
            nom_option: HashMap::new(),
            new_nom_option: HashMap::new(),
            liste_options: HashMap::new(),
            id_option: 0,
            show_option_window : false,
            select_option_remove_id: None,
        }
    }
}


impl ProgrammeWindow {

    pub fn get_groupe(&self) -> &HashMap<usize, Arc<Groupe>>{
        &self.groupe
    }

    pub fn get_liste_semaine(&self) -> &HashMap<(usize,usize), Arc<Semaine>>{
        &self.semaines
    }
    pub fn get_liste_matiere_prog(&self) -> &HashMap<usize, Arc<MatiereProg>>{
        &self.matiere_prog
    }

    pub fn get_liste_options(&self) -> &HashMap<usize, Arc<OptionProgramme>>{
        &self.liste_options
    }

    pub fn get_assignement(&self) -> &HashMap<usize, Arc<Assignation>>{
        &self.assignement
    }

    pub fn get_matiere_inter_classe(&self) -> &HashMap<usize, Arc<MatiereInterClasse>>{
        &self.matiere_inter_classe
    }

    pub fn charge(&mut self, groupe: HashMap<usize, Arc<Groupe>>,semaines: HashMap<(usize,usize), Arc<Semaine>>, matiere_prog: HashMap<usize, Arc<MatiereProg>>,  filieres: HashMap<usize, Arc<Filiere>>, classes: HashMap<usize, Arc<Classe>>, matieres: HashMap<usize, Arc<Matiere>>, salles_type: HashMap<usize, Arc<RoomType>>, liste_options: HashMap<usize, Arc<OptionProgramme>>,  assignement :HashMap<usize, Arc<Assignation>>, matiere_inter_classe: HashMap<usize, Arc<MatiereInterClasse>>, ) {
        
            self.groupe = groupe;
            self.matieres =  matieres;
            self.semaines = semaines;
            self.matiere_prog = matiere_prog;
            //self.programmes =  programme;
            self.filieres =  filieres;
            self.classes =  classes;
            self.salles_type = salles_type;   
            for (id, filiere) in self.filieres.iter(){
                self.nb_sem.insert(*id,filiere.get_nb_semaine());
            }
            for (id, matiere_prog) in self.matiere_prog.iter(){
                self.duree_minimum.insert(*id,Some(*matiere_prog.get_duree_minimum()));
                self.duree_maximum.insert(*id,Some(*matiere_prog.get_duree_maximum()));
            }

            self.liste_options = liste_options;

            for (id, matiere_prog) in self.matiere_prog.iter(){   
                let id_filiere = matiere_prog.get_semaine().get_filiere().get_id();
                let id_matiere = *matiere_prog.get_matiere().get_id();
                let option = matiere_prog.get_option();
                
                //self.liste_selected_option_par_matiere.insert((id_filiere, id_matiere) , Arc::clone(&option));
                self.nom_option.insert((self.selected_filiere_id, option.get_name()),  option.get_name()); //pour ne pas pouvoir saisir de doublons sur les options
            }
            
            self.assignement = assignement;

            

    }



    pub fn fenetre_ajout_option(&mut self, ctx: &egui::Context){

            egui::Window::new("Saisie d'une option")
                .current_pos(pos2(50.0,50.0))
                .show(ctx, |ui| {
                    //while reste_ici{
                    ui.vertical(|ui|{
                        let mut nom_option: String =
                        match self.new_nom_option.get(&self.selected_filiere_id) {
                            Some(nom) => nom.clone(),
                            None => "".to_string(),
                        };
                        
                        ui.horizontal(|ui|{
                            ui.label("nom de l'option");
                            let response_nom_option = ui.add(egui::TextEdit::singleline(&mut nom_option).desired_width(100.0));
                            self.new_nom_option.insert(self.selected_filiere_id, nom_option.clone() );
                        });

                        ui.horizontal(|ui|{
                            if ui.button("Valider").clicked(){
                                if !self.nom_option.contains_key(&(self.selected_filiere_id, nom_option.clone())){
                                    let filiere = self.filieres.get(&self.selected_filiere_id).unwrap();
                                    self.liste_options.insert(self.id_option, Arc::new(OptionProgramme::new(self.id_option,nom_option.clone(),Arc::clone(filiere))));
                                    self.nom_option.insert((self.selected_filiere_id, nom_option.clone()), nom_option.clone());
                                    self.show_option_window = false;
                                }
                            }
                            if ui.button("Annuler").clicked(){
                                self.show_option_window = false;
                            }
                        });
                    });
            });
        
    }


    pub fn supprime_matiere_prog(&mut self, id_matiere_prog: usize){
        if let Some(matiere_prog) = self.matiere_prog.get(&id_matiere_prog) {
                                        
            let matiere = matiere_prog.get_matiere() ;
            let id_matiere = matiere.get_id();



            //suppression des groupes reliées à la matiereprog supprimée
            let ids_to_remove: Vec<_> = self.classes.iter()
                .filter(|(_, classe)| classe.get_filiere().get_id() == self.selected_filiere_id)
                .flat_map(|(id_classe, _)| {
                    self.groupe.iter()
                        .filter(move |(_, groupe)| groupe.get_classe().get_id() == *id_classe && groupe.get_matiere().get_id() == id_matiere)
                        .map(|(id_groupe, _)| id_groupe.clone())
                })
                .collect();

            for id_groupe in ids_to_remove {
                self.groupe.remove(&id_groupe);
            }

            //suppression des assignation reliées à la matiereprog supprimée
            let ids_to_remove: Vec<_> = 
                self.assignement.iter()
                .filter(|(_, assignement)|{assignement.get_matiere_prog().get_id() == matiere_prog.get_id()})
                .map(|(id_assignement, _)|{id_assignement.clone()})
                .collect();
            
            for id_assignement in ids_to_remove {
                self.assignement.remove(&id_assignement);
            }

            //suppression des interclasse reliées à la matiereprog supprimée   
            let ids_to_remove: Vec<_> = 
                self.matiere_inter_classe.iter()
                .filter(|(_, inter_classe)|{inter_classe.get_matiere_prog().get_id() == matiere_prog.get_id()})
                .map(|(id_inter_classe, _)|{id_inter_classe.clone()})
                .collect();
            
            for id_matiere_inter_classe in ids_to_remove {
                self.matiere_inter_classe.remove(&id_matiere_inter_classe);
            }
        }  
        //suppression à ajouter, assignement si il existe , pour chaque classe, interclasse egalement, faire le tour des listes de ce script
        self.matiere_prog.remove(&id_matiere_prog); 
        self.select_matiere_prog_remove_id = None

    }


    pub fn supprime_option(&mut self, id_option: usize){
        
        let ids_to_remove: Vec<_> = 
                self.matiere_prog.iter()
                .filter(|(_, matiere_prog)|{matiere_prog.get_option().get_id() == &id_option})
                .map(|(id_matiere_prog, _)|{id_matiere_prog.clone()})
                .collect();
            
        for id_matiere_prog in ids_to_remove {
            self.supprime_matiere_prog(id_matiere_prog);
        }

        //self.nom_option.insert((self.selected_filiere_id, option.get_name()),  option.get_name());
        let ids_to_remove: Vec<_> = 
                self.nom_option.iter()
                .filter(|((id_filiere, _), nom_option)|
                {
                    id_filiere == &self.selected_filiere_id
                    && self.liste_options.get(&id_option).unwrap().get_name() == *nom_option.clone()
                })
                .map(|((id_filiere,nom_option), _)|{(*id_filiere,nom_option.clone())})
                .collect();
            
        for (id_filiere,nom_option) in ids_to_remove {
            self.nom_option.remove(&(id_filiere, nom_option.clone()));
        }

        self.liste_options.remove(&id_option);
    }





    pub fn build(&mut self, ctx: &egui::Context) 
    {
        
       
        //si suppression d'une option alors on supprime les matieres_prog renseignées dans cette option ainsi que les dependances de ces matiere progs
        match self.select_option_remove_id {
            Some(id_option) =>   {
                                            self.supprime_option(id_option);
                                            self.select_option_remove_id = None;
                                        },
            None => {self.select_option_remove_id = None;},
        };
        
        //suppression de la matiere prog supprimée et des element qui lui sont liés
        match self.select_matiere_prog_remove_id {
            Some(id) => {  
                                    self.supprime_matiere_prog(id); 
                                },
            None => self.select_matiere_prog_remove_id = None
        };


        self.id_groupe = *self.groupe.keys().max().unwrap_or(&0) + 1;
        self.id_matiere_prog = self.matiere_prog.keys().max().unwrap_or(&0) + 1;
  
        
        self.id_option = *self.liste_options.keys().max().unwrap_or(&0) + 1; 
        let mut init_option: HashSet<usize> = HashSet::new();
        for (cle, option) in self.liste_options.iter(){
            if option.get_name() == "Commun" {
                init_option.insert(option.get_filiere().get_id());
            }
        }
        for( cle, filiere) in self.filieres.iter(){
            match init_option.get(&filiere.get_id()){
                Some(id_fliere) => {},
                None => {
                            self.liste_options.insert(self.id_option, Arc::new(OptionProgramme::new(self.id_option,"Commun".to_string(),Arc::clone(filiere)))); 
                            self.id_option += 1;
                        },
            }
        }

        
        if self.show_option_window{
            self.fenetre_ajout_option(ctx);
        }

        
        egui::CentralPanel::default()
            .show(ctx, |ui| {

                ui.horizontal(|ui| {
                    for (id_filiere, filiere) in self.filieres.iter() {
                        if ui.selectable_label(&self.selected_filiere_id == id_filiere,format!("{:}", filiere.get_name())).clicked() {                                  
                            self.selected_filiere_id = *id_filiere;
                            if !self.nb_sem.get(&self.selected_filiere_id).is_none() {
                                self.new_nb_sem = self.nb_sem.get(&self.selected_filiere_id).unwrap().unwrap().to_string();
                            }
                        }
                    }
                });

                    //ui.end_row();
                   let screen_rect = ctx.screen_rect();
                   let available_rect = screen_rect;
                   
                   // Calculer les dimensions proportionnelles
                   let width_ratio = 0.3; // 30% de la largeur
                   let height_ratio = 0.2; // 20% de la hauteur
                   let  width = ui.available_width() / 9.0;
                   let  height = ui.available_height();
                   
                   let custom_region = Rect::from_min_max(
                    pos2(100.0, 100.0),
                    pos2(500.0, 400.0)
                    );
                        
                    let area0 = egui::Area::new("top_center")
                        .fixed_pos(pos2(
                        available_rect.min.x + (available_rect.width() * 0.05), // 5% de marge
                        available_rect.min.y + (available_rect.height() * 0.05) // 5% de marge
                    ))
                    //.constrain_to(custom_region)
                    .anchor(Align2::CENTER_TOP,egui::vec2(0.0, 30.0)) // Ancrage au coin inférieur gauche de l'Area
                    .pivot(Align2::CENTER_TOP)
                    .show(ctx, |ui| {
                        
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Nombre de semaine(s) du programme: ");
                            let response_nb_sem = ui.text_edit_singleline(&mut self.new_nb_sem);
                            if response_nb_sem.lost_focus() {
                                match self.new_nb_sem.parse::<usize>() {
                                    Ok(nombre) => {
                                        if nombre > 0 {
                                            self.nb_sem.insert(self.selected_filiere_id, Some(nombre));
                                            let filiere = self.filieres.get_mut(&self.selected_filiere_id).unwrap();
                                            //filiere.nb_semaine = nombre;
                                            let mut name_guard = filiere.nb_semaine.lock().unwrap();
                                            *name_guard = nombre;
                                            drop(name_guard);
                                        } else {
                                            self.new_nb_sem.clear();
                                        }  
                                    },
                                    Err(_) => {
                                        self.new_nb_sem.clear();
                                    }
                                }
                            }
                
                            let ajout_filiere = (ui.button("Afficher").clicked() 
                                && !self.nb_sem.get(&self.selected_filiere_id).is_none()
                                ) || self.nb_sem_deja_valid.get(&self.selected_filiere_id).is_some()
                                ;
                            if ajout_filiere {
                                //AFFICHE LES ELEMENTS DE SAISIE SI UN NOMBRE DE SEMAINE A ETE RENSEIGNER, A AMELIORER DANS LE CAS D'UN CHARGEMENT, CAR ON DOIT QUAND MEME CLIQUER POUR AFFICHER
                                self.nb_sem_deja_valid.insert(self.selected_filiere_id);

                                let exist = self.semaines.iter()
                                .filter(|(_cle,semaine)| {semaine.get_filiere().get_id() == self.selected_filiere_id})
                                .map(|(cle, _semaine)| *cle) // Clonage requis pour `Arc`
                                .count();

                                if exist == 0 {
                                    let nb_semaines = match self.nb_sem.get(&self.selected_filiere_id){
                                        Some(Some(semaines)) => semaines,
                                        Some(&None) => &0,
                                        None => &0,
                                    };

                                    if nb_semaines > &0 {
                                        if let Some(filiere) = self.filieres.get(&self.selected_filiere_id){
                                            let mut i = 0;
                                            while i < *nb_semaines {
                                                //self.semaines.insert(id_max + i, Arc::new(Semaine::new(id_max + i, Arc::clone(filiere)))); 
                                                self.semaines.insert((self.selected_filiere_id, i), Arc::new(Semaine::new(i, Arc::clone(filiere)))); 
                                                i += 1 ;
                                            }
                                        }
                                    } 
                                }                          
                            }
                            ui.add_space(50.0);

                        });    
                        //ui.separator();
                    });

                    
                    
               
                    let position = Vec2::new(
                        0.0, // 100 pixels depuis la gauche
                        0.0  // 100 pixels depuis le bas
                   );
                   
           
                   let screen_rect = ctx.screen_rect();
                   let available_rect = screen_rect;
                   
                   // Calculer les dimensions proportionnelles
                   let width_ratio = 0.3; // 30% de la largeur
                   let height_ratio = 0.2; // 20% de la hauteur

                   let custom_region = Rect::from_min_max(
                    pos2(100.0, 100.0),
                    pos2(500.0, 400.0)
                    );
                
                
                   // Créer une Area à cette positionsition

                    if self.nb_sem_deja_valid.contains(&self.selected_filiere_id){
                        
                        
                       // ui.vertical_centered(|ui|{
                            let area_matiere = egui::Area::new("saisie_matiere")
                                .fixed_pos(pos2(
                                    (available_rect.width() * 0.10), // 5% de marge
                                    f32::min(150.0,available_rect.height() * 0.20) // 5% de marge
                                ))
                                //.constrain_to(custom_region)
                                //.movable(true)
                                //.anchor(Align2::LEFT_TOP,egui::vec2(50.0, 100.0)) // Ancrage au coin inférieur gauche de l'Area
                                //.pivot(Align2::LEFT_TOP)
                                .show(ctx, |ui| 
                                {
                                    
                                    ui.set_max_width(available_rect.max.x * 0.8);

                                    egui::ScrollArea::both() // Activer le défilement vertical et horizontal
                                    .id_source(0)
                                    .auto_shrink([false, true]) // Permet à la zone de se rétrécir horizontalement, mais de ne pas se rétrécir verticalement
                                    .show(ui, |ui| {

                                    Frame::none()
                                    .fill(Color32::from_rgb(40, 40, 40)) // Couleur de fond
                                    .stroke(Stroke::new(1.0, Color32::from_rgb(100, 100, 100))) // Bordure
                                    .rounding(Rounding::same(8.0)) // Coins arrondis
                                    .inner_margin(20.0) // Marge intérieure
                                    //.outer_margin(-50.0)
                                    .show(ui, |ui| {
                                        ui.columns(1, |columns| 
                                            {
                                                
                                                columns[0].allocate_ui_with_layout(
                                                        Vec2::new(width/3.0, 15.0),
                                                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                                    |ui| 
                                                {
                                                    let available_width = ui.available_width().max(0.0);
                                                    ui.set_max_width(available_width.max(200.0));

                                                    egui::Grid::new("Grid111")
                                                    .num_columns(2)
                                                    .min_col_width(175.0)
                                                    //.striped(true)
                                                    .show(ui, |ui| 
                                                    {
                                                        
                                                        ui.add_space(width);
                                                        //ui.add_space(50.0);
                                                        ui.label("Ajout d'une matière: ");
                                                        
                                                        self.selected_matiere = 
                                                            match self.liste_selected_matiere.get(&self.selected_filiere_id) {
                                                                Some(matiere) => matiere.get_name(),
                                                                None => String::new()
                                                            };
                                                                
                                                        let mat_liste = egui::ComboBox::from_id_source("Matieres")
                                                            .selected_text(&self.selected_matiere)
                                                            .show_ui(ui, |ui| 
                                                            {
                                                                for  (id,matiere) in self.matieres.iter() {
                                                                    //println!("{:?}",matiere.get_room_type());
                                                                    if ui.selectable_label(self.selected_matiere == matiere.get_name(), matiere.get_name()).clicked() {
                                                                        self.selected_matiere = (matiere.get_name()).to_string();
                                                                        //self.selected_matiere_id = *id;
                                                                        self.liste_selected_matiere.insert(self.selected_filiere_id, Arc::clone(matiere));
                                                                    }
                                                                }
                                                            }).response;



                                                        ui.label("durée minimum d'un cours: ");
                                                        let mut nb_heure =
                                                                match self.new_duree_minimum.get(&self.selected_filiere_id) {
                                                                    Some(duree) => duree.clone(),
                                                                    None => String::new()
                                                                };    

                                                        let response_nb_heure = ui.add(egui::TextEdit::singleline(&mut nb_heure).desired_width(100.0));
                                                        self.new_duree_minimum.insert(self.selected_filiere_id, nb_heure.clone() );
                                                        if response_nb_heure.lost_focus() {
                                                            match self.new_duree_minimum.get(&self.selected_filiere_id).unwrap().parse::<usize>() {
                                                                Ok(nombre) => {
                                                                    if nombre > 0 {
                                                                        self.duree_minimum.insert(self.selected_filiere_id, Some(nombre));
                                                                    } else {
                                                                        self.new_duree_minimum.clear();
                                                                        self.duree_minimum.insert(self.selected_filiere_id, Some(0));
                                                                    }  
                                                                },
                                                                Err(_) => {
                                                                    self.new_duree_minimum.clear();
                                                                    self.duree_minimum.insert(self.selected_filiere_id, Some(0));
                                                                }
                                                            }
                                                        }


                                                        let mut en_groupe =
                                                            match self.selected_en_groupe.get(&self.selected_filiere_id) {
                                                                Some(groupe) => *groupe,
                                                                None => false
                                                            };

                                                        if ui.checkbox(&mut en_groupe, format!("Cours en groupe?")).changed(){
                                                            self.selected_en_groupe.insert(self.selected_filiere_id, en_groupe); 
                                                        }



                                                        let mut en_groupe_inter =
                                                        match self.selected_en_groupe_interclasse.get(&self.selected_filiere_id) {
                                                            Some(groupe) => *groupe,
                                                            None => false
                                                        };

                                                        if ui.checkbox(&mut en_groupe_inter, format!("Cours interclasse?")).changed(){
                                                            self.selected_en_groupe_interclasse.insert(self.selected_filiere_id, en_groupe_inter);
                                                        }

                                                        ui.end_row();
 

                                                        
                                                        ui.add_space(width);
                                                        //ui.add_space(50.0);
                                                        ui.label("Nombre d'heures: ");
                                                        
                                                        let mut nb_heure =
                                                                match self.new_nb_heure.get(&self.selected_filiere_id) {
                                                                    Some(heure) => heure.clone(),
                                                                    None => String::new()
                                                                };                           
                                                        let response_nb_heure = ui.add(egui::TextEdit::singleline(&mut nb_heure).desired_width(mat_liste.rect.size()[0]));
                                                        
                                                        self.new_nb_heure.insert(self.selected_filiere_id, nb_heure.clone() );
                                                        if response_nb_heure.lost_focus() {
                                                            match self.new_nb_heure.get(&self.selected_filiere_id).unwrap().parse::<usize>() {
                                                                Ok(nombre) => {
                                                                    if nombre > 0 {
                                                                        self.nb_heure.insert(self.selected_filiere_id, Some(nombre));
                                                                    } else {
                                                                        self.new_nb_heure.clear();
                                                                        self.nb_heure.insert(self.selected_filiere_id, Some(0));
                                                                    }  
                                                                },
                                                                Err(_) => {
                                                                    self.new_nb_heure.clear();
                                                                    self.nb_heure.insert(self.selected_filiere_id, Some(0));
                                                                }
                                                            }
                                                        }


                                                        ui.label("durée maximum d'un cours: ");
                                                    
                                                        let mut nb_heure =
                                                            match self.new_duree_maximum.get(&self.selected_filiere_id) {
                                                                Some(duree) => duree.clone(),
                                                                None => String::new()
                                                            };                           
                                                        //let response_nb_heure = ui.text_edit_singleline(&mut nb_heure);
                                                            
                                                        let response_nb_heure = ui.add(egui::TextEdit::singleline(&mut nb_heure).desired_width(100.0));
                                                        self.new_duree_maximum.insert(self.selected_filiere_id, nb_heure.clone() );
                                                        if response_nb_heure.lost_focus() {
                                                            match self.new_duree_maximum.get(&self.selected_filiere_id).unwrap().parse::<usize>() {
                                                                Ok(nombre) => {
                                                                    if nombre > 0 {
                                                                        self.duree_maximum.insert(self.selected_filiere_id, Some(nombre));
                                                                    } else {
                                                                        self.new_duree_maximum.clear();
                                                                        self.duree_maximum.insert(self.selected_filiere_id, Some(0));
                                                                    }  
                                                                },
                                                                Err(_) => {
                                                                    self.new_duree_maximum.clear();
                                                                    self.duree_maximum.insert(self.selected_filiere_id, Some(0));
                                                                }
                                                            }
                                                        }

                                                        
                                                        if *self.selected_en_groupe.get(&self.selected_filiere_id).unwrap_or(&false) {
                                                            ui.label("Nombre de groupes: ");
                                                            let response_nb_groupe = ui.text_edit_singleline(&mut self.new_nb_groupe);
                                                            if response_nb_groupe.lost_focus() {
                                                                match self.new_nb_groupe.parse::<usize>() {
                                                                    Ok(nombre) => {
                                                                        if nombre > 0 {
                                                                            self.nb_groupe.insert(self.selected_filiere_id, Some(nombre));
                                                                        } else {
                                                                            self.new_nb_groupe.clear();
                                                                        }  
                                                                    },
                                                                    Err(_) => {
                                                                        self.new_nb_groupe.clear();
                                                                    }
                                                                }
                                                            }
                                                        }else{
                                                            self.nb_groupe.insert(self.selected_filiere_id, Some(1));
                                                            self.new_nb_groupe.clear();
                                                        }
                                                        

                                                        ui.end_row();
                                                        
                                                        ui.add_space(width);
                                                        //ui.add_space(50.0);   
                                                        

                                                        

                                                        
                                                        ui.horizontal(|ui|{
                                                            ui.label("Options: ");
                                                            if ui.button("➕ Ajouter").clicked(){
                                                                self.show_option_window = true;
                                                            }
                                                        }) ;
                                                        
                                                        //pour que ca ne reste pas sur la selection en cours lorsque l'on change de filiere
                                                        if let Some(select) = self.liste_selected_option.get(&self.selected_filiere_id){
                                                            self.selected_option = select.get_name();
                                                        }else{
                                                            self.selected_option = String::new();
                                                        }
                                                         
                                                        let option_liste = egui::ComboBox::from_id_source("Options")
                                                        .selected_text(&self.selected_option)
                                                        .show_ui(ui, |ui| 
                                                        {
                                                            for  (id,option) in self.liste_options.iter() 
                                                            .filter(|(id,option)|
                                                                {
                                                                    self.selected_filiere_id == option.get_filiere().get_id()
                                                                })
                                                            {
                                                                
                                                                if ui.selectable_label(self.selected_option == option.get_name(), option.get_name()).clicked() {
                                                                    self.selected_option = (option.get_name()).to_string();
                                                                    self.liste_selected_option.insert(self.selected_filiere_id, Arc::clone(option));
                                                                }
                                                            }
                                                        }).response;

                                                        //ui.add_space(50.0);
                                                         
                                                        ui.end_row();
                                                        ui.add_space(10.0);
                                                        ui.end_row(); 
                                                    });
        
    
                                        });

                                        columns[0].allocate_ui_with_layout(
                                                Vec2::new(width/3.0, 15.0),
                                                egui::Layout::left_to_right(egui::Align::Center),
                                                |ui| 
                                                {

                                            ui.add_space(width);      
                                            //ui.add_space(50.0);
                                            ui.label("Liste des semaines auxquelles l'ajouter: ");
                                                
                                            egui::CollapsingHeader::new("Selection des semaines")
                                            .show(ui, |ui| {
                                                let options: Vec<usize> = 
                                                                self.semaines.clone()//.iter()
                                                                .keys()
                                                                .filter(|(id_filiere,_i)| {*id_filiere == self.selected_filiere_id})
                                                                .map(|(_id_filiere, i)| *i)
                                                                .collect();
                                                
                                                ui.vertical(|ui| {     
                                                    egui::ScrollArea::both() 
                                                    .id_source(1)
                                                    .auto_shrink([true, true])   
                                                    .show(ui, |ui| {  
                                                        
                                                        let mut i: usize = 0;
                                                        let mut selected = match self.selected_all.get(&self.selected_filiere_id){
                                                            Some(select) => *select,
                                                            None => false,
                                                        };
                                                        
                                                        if ui.checkbox(&mut selected, format!("Toutes")).changed() {
                                                            self.selected_all.insert(self.selected_filiere_id, selected);
                                                            for (_cle,option) in options.iter().enumerate(){
                                                                if selected{
                                                                    self.selected_semaines.insert((self.selected_filiere_id, i),*option);   
                                                                }else {
                                                                    self.selected_semaines.remove(&(self.selected_filiere_id, i));
                                                                }
                                                                i += 1 ;
                                                            }
                                                        }

                                                        i = 0;
                                                        for (_cle,option) in options.iter().enumerate(){
                                                            let mut selected = self.selected_semaines.contains_key(&(self.selected_filiere_id, i));
                                                            if ui.checkbox(&mut selected, format!("Semaine {:}",i)).changed() {
                                                                if selected {
                                                                    self.selected_semaines.insert((self.selected_filiere_id, i), *option);
                                                                } else {
                                                                    self.selected_semaines.remove(&(self.selected_filiere_id, i));
                                                                    self.selected_all.insert(self.selected_filiere_id, false);
                                                                }
                                                            }
                                                            i += 1 ;
                                                        }

                                                    });
                                                });
                                                
                                
                                            });
                                        });

                                        columns[0].allocate_ui_with_layout(
                                            Vec2::new(width/3.0, 15.0),
                                            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                            |ui| 
                                            {
                                                ui.add_space(50.0);
                                            });

                                        columns[0].allocate_ui_with_layout(
                                            Vec2::new(width/3.0, 15.0),
                                            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                            |ui| 
                                            {
                                                //ui.add_space(50.0);
                                                let i : usize = 0;



                                                let verif_matiere = if self.liste_selected_matiere.contains_key(&self.selected_filiere_id){ true } else { false };
                                                let verif_duree_minimum = if self.duree_minimum.contains_key(&self.selected_filiere_id){ true } else { false };
                                                let verif_duree_maximum = if self.duree_maximum.contains_key(&self.selected_filiere_id){ true } else { false };
                                                let verif_option = if self.selected_option.len() > 1 { true } else { false };
                                                let verif_en_groupe = 
                                                    match self.selected_en_groupe.get(&self.selected_filiere_id){
                                                        Some(en_groupe) => {
                                                                                        if !en_groupe {true} 
                                                                                        else{
                                                                                            match self.nb_groupe.get(&self.selected_filiere_id){
                                                                                                Some(nb_groupe) => {if *nb_groupe > Some(0) {true} else {false} },
                                                                                                None => false,
                                                                                            } 
                                                                                        }
                                                                                  },
                                                        None => true,
                                                    };
                                                let verif_nb_heure = 
                                                    match self.nb_heure.get(&self.selected_filiere_id){
                                                        Some(heure) => if *heure > Some(0) {true} else {false},
                                                        None => false,
                                                    };

                                                let mut verif_controle_duree = false;
                                                if verif_duree_minimum && verif_duree_maximum {
                                                    if self.duree_minimum.get(&self.selected_filiere_id).unwrap() <= self.duree_maximum.get(&self.selected_filiere_id).unwrap() && self.duree_minimum.get(&self.selected_filiere_id).unwrap().unwrap() > 0 {
                                                        verif_controle_duree = true;
                                                    }
                                                }

                                                let mut verif_controle_nb_heure = false;
                                                if verif_duree_maximum && verif_nb_heure{
                                                    if self.nb_heure.get(&self.selected_filiere_id).unwrap() >= self.duree_maximum.get(&self.selected_filiere_id).unwrap(){
                                                        verif_controle_nb_heure = true;
                                                    } 
                                                }


                                                let ajout_matiere = 
                                                        ui.button("Ajouter").clicked() 
                                                        && verif_matiere
                                                        && verif_duree_minimum
                                                        && verif_duree_maximum
                                                        && verif_option
                                                        && verif_en_groupe
                                                        && verif_nb_heure
                                                        && verif_controle_duree
                                                        && verif_controle_nb_heure;
                                                
                                                self.ajout_matiere = ajout_matiere;
                                            });
                     

                                    });
                                });
                                });
                                }).response;
                            //});
                        
                 

                    //SELECTION DES INFORMATIONS SUR LA MATIERE A AJOUTER
                    
                //dbg!(&area_matiere.rect.size());
                //dbg!(&area_matiere.rect.max.y);
                let y = area_matiere.rect.max.y;
                
                let areat_tableau = egui::Area::new("area_tableau")
                    .fixed_pos(pos2(
                    available_rect.min.x + (available_rect.width() * 0.05), // 5% de marge
                    y + 20.0 // 5% de marge
                ))
                //.constrain_to(custom_region)
                //.anchor(Align2::LEFT_BOTTOM,egui::vec2(0.0, -25.0)) // Ancrage au coin inférieur gauche de l'Area
                //.pivot(Align2::LEFT_BOTTOM)
                .show(ctx, |ui| {
                        if self.ajout_matiere {   
                            self.ajout_matiere = false;
                            //SAUVEGARDE DES MATIERE AJOUTER A CHAQUE FILIERE     
                            let nb_groupe = match self.nb_groupe.get(&self.selected_filiere_id) {
                                Some(Some(nb)) => nb,
                                Some(&None) => &1,
                                None => &1,
                            };
                            let i: usize = 0;                  
                            for (cle, semaine) in self.semaines.iter().filter(|(id,_semaine)| {id.0 == self.selected_filiere_id}){
                                if self.selected_semaines.contains_key(&(cle)){
                                    dbg!(&self.duree_minimum);
                                    self.matiere_prog.insert(
                                        self.id_matiere_prog, 
                                        
                                        Arc::new(MatiereProg::new( self.id_matiere_prog,
                                                             Arc::clone(self.liste_selected_matiere.get(&self.selected_filiere_id).unwrap()),
                                                             self.nb_heure.get(&self.selected_filiere_id).unwrap().unwrap(), 
                                                             self.duree_minimum.get(&self.selected_filiere_id).unwrap().unwrap(), 
                                                             self.duree_maximum.get(&self.selected_filiere_id).unwrap().unwrap(), 
                                                             *self.selected_en_groupe.get(&self.selected_filiere_id).unwrap_or(&false),
                                                             *nb_groupe,
                                                             *self.selected_en_groupe_interclasse.get(&self.selected_filiere_id).unwrap_or(&false), 
                                                             Arc::clone(semaine),
                                                             Arc::clone(self.liste_selected_option.get(&self.selected_filiere_id).unwrap())
                                                            )));
                                    self.id_matiere_prog += 1;
                                }
                            }

                            for i in 0..*nb_groupe {
                                for (id, classe) in self.classes.iter().filter(|(id, classe)| { classe.get_filiere().get_id() == self.selected_filiere_id}){
                                    self.groupe.insert(self.id_groupe, Arc::new(Groupe::new(self.id_groupe,i + 1, Arc::clone(classe), Arc::clone(self.liste_selected_matiere.get(&self.selected_filiere_id).unwrap()))));
                                    self.id_groupe += 1;
                                }
                            } 
                        }

                        //AFFICHAGE DES SEMAINES
                        egui::Grid::new("tableau1")
                            .min_col_width(100.0)
                            .striped(true)
                            .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                for (id, _semaine) in self.semaines.iter().filter(|(id,_semaine)| {id.0 == self.selected_filiere_id}) {
                                    if ui.selectable_label(self.selected_semaine_onglet.get(&self.selected_filiere_id) == Some(&id.1),format!("{:}", id.1)).clicked() {                                  
                                        self.selected_semaine_onglet.insert(self.selected_filiere_id, id.1);   
                                    }
                                }
                            });
                        });    
                        
                        //dbg!(&self.matiere_prog);
                        // AFFICHAGE DES MATIERES AJOUTEES A LA SEMAINE ET A LA FILIERE SELECTIONNEES
                        egui::ScrollArea::both()
                        .id_source(2)
                        .auto_shrink([true, true])
                        .show(ui, |ui| {

                            let mut i: usize = 0;
                            egui::Grid::new("tableau de placement")//option.get_id()
                                .num_columns(2)
                                .striped(true)
                                .spacing((20.0,20.0))
                                .min_col_width(800.0)
                                .show(ui, |ui| {

                                for (id, option) in self.liste_options.iter()
                                    .filter(|(id, option)|{option.get_filiere().get_id() == self.selected_filiere_id})
                                {
                                    
                                    ui.vertical(|ui|{
                                       
                                        ui.horizontal(|ui|{
                                            ui.heading(option.get_name());
                                            if ui.button("❌").clicked(){
                                                self.select_option_remove_id = Some(id.clone());
                                            }
                                        });
                                        
                                     
                                        egui::Grid::new(option.get_id())//option.get_id()
                                        .num_columns(8)
                                        .striped(true)
                                        .spacing((10.0,10.0))
                                        .min_col_width(100.0)
                                        .show(ui, |ui| {
                                                //ui.heading(option.get_name());
                                                ui.vertical_centered(|ui| {
                                                    ui.label("Matiere");
                                                });
                                                ui.vertical_centered(|ui| {
                                                    ui.label("Nombre d'heure(s)");
                                                });
                                                ui.vertical_centered(|ui| {
                                                    ui.label("Durée minimum");
                                                });
                                                ui.vertical_centered(|ui| {
                                                    ui.label("Durée maximum");
                                                });
                                                ui.vertical_centered(|ui| {
                                                    ui.label("Cours en groupe?");
                                                });
                                                ui.vertical_centered(|ui| {
                                                    ui.label("Nombre de groupe(s)");
                                                }); 
                                                ui.vertical_centered(|ui| {
                                                    ui.label("Cours inter-classe?");
                                                });
                                                ui.label(" ");
                                                ui.end_row();

                                                
                                                for (i, matiere) in self.matiere_prog.iter()
                                                    .filter(|(id, matiere_prog)| 
                                                    {
                                                        matiere_prog.get_semaine().get_filiere().get_id() == self.selected_filiere_id 
                                                        && Some(matiere_prog.get_semaine().get_id()) == self.selected_semaine_onglet.get(&self.selected_filiere_id)
                                                        && matiere_prog.get_option().get_id() == option.get_id()
                                                    }) 
                                                {
                                                    ui.vertical_centered(|ui| {
                                                        ui.label(matiere.get_matiere().get_name());
                                                    });
                                                    ui.vertical_centered(|ui| {
                                                        ui.label(format!("{:}", matiere.get_nb_heure()));
                                                    });
                                                    ui.vertical_centered(|ui| {
                                                        ui.label(format!("{:}", matiere.get_duree_minimum()));
                                                    });
                                                    ui.vertical_centered(|ui| {
                                                        ui.label(format!("{:}", matiere.get_duree_maximum()));
                                                    });
                                                    ui.vertical_centered(|ui| {
                                                        ui.label(format!("{:}", matiere.get_en_groupe().to_string()));
                                                    });
                                                    ui.vertical_centered(|ui| {
                                                        ui.label(format!("{:}", matiere.get_nb_groupe()));
                                                    });
                                                    ui.vertical_centered(|ui| {
                                                        ui.label(format!("{:}", matiere.get_en_groupe_inter_classe().to_string()));
                                                    });
                                                    ui.horizontal_centered(|ui| {
                                                        if ui.button("❌").clicked() {
                                                            self.select_matiere_prog_remove_id = Some(*i);
                                                        }
                                                    });                        
                                                    ui.end_row();
                                                }
                                            });
                                        });
                                            if i%2 > 0{
                                                ui.end_row();
                                            }
                                            i += 1;
                                        
                                }
                                
                                
                                
                            });    
                        });

                });
                   // }
                    
                }  
            });

        
                                                                
            
            
    }
}
                