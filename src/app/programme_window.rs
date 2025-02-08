use eframe::egui;
use std::sync::{Arc, Mutex};
//use egui::{Context, Ui, Color32, Align2, Frame, Vec2};
use std::collections::{HashMap, HashSet};
use crate::struc::matiere::Matiere;
use crate::struc::programme::{MatiereProg, Semaine, MatiereInterClasse}; 
//use crate::struc::filiere::Filiere;
//use crate::struc::classe::Classe;

use crate::app::filiere_window::{Filiere, Classe};
use crate::app::room_window::RoomType;


#[derive(Clone, Debug)]
pub struct ProgrammeWindow {
 
    //programmes:   HashMap<usize, Arc<Programme>>
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


    filieres:  HashMap<usize, Arc<Filiere>>,
    classes:  HashMap<usize, Arc<Classe>>,
    matieres: HashMap<usize, Arc<Matiere>>,
    salles_type: HashMap<usize, Arc<RoomType>>,
    //new_matiere: String,

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
}

impl  Default for ProgrammeWindow  {
    fn default() -> Self {
        Self {
            new_nb_groupe:String::new(),
            nb_groupe: HashMap::new(),
            select_matiere_prog_remove_id: None,
            selected_all: HashMap::new(), //false,
            selected_all_classe: HashMap::new(),
            selected_classe: HashMap::new(),
            selected_semaine_onglet: HashMap::new(),
            selected_semaines: HashMap::new(),
            matieres: HashMap::new(),
            //programmes: HashMap::new(),// HashMap::new(),
            semaines: HashMap::new(),
            id_matiere_prog:0,
            matiere_prog: HashMap::new(),
            matiere_inter_classe: HashMap::new(),
            filieres: HashMap::new(),
            classes:  HashMap::new(),
            salles_type: HashMap::new(),
            
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
        }
    }
}


impl ProgrammeWindow {

    /*pub fn get_liste_programme(&self) -> &HashMap<usize, Arc<Programme>>{
        &self.programmes
    }*/
    pub fn get_liste_semaine(&self) -> &HashMap<(usize,usize), Arc<Semaine>>{
        &self.semaines
    }
    pub fn get_liste_matiere_prog(&self) -> &HashMap<usize, Arc<MatiereProg>>{
        &self.matiere_prog
    }

    pub fn charge(&mut self, semaines: HashMap<(usize,usize), Arc<Semaine>>, matiere_prog: HashMap<usize, Arc<MatiereProg>>,  filieres: HashMap<usize, Arc<Filiere>>, classes: HashMap<usize, Arc<Classe>>, matieres: HashMap<usize, Arc<Matiere>>, salles_type: HashMap<usize, Arc<RoomType>>) {
        
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
    }

    pub fn build(&mut self, ctx: &egui::Context) {
        //egui::TopBottomPanel::top("onglets_filiere") //::new("Création des programmes")
        match self.select_matiere_prog_remove_id {
            Some(id) => self.matiere_prog.remove(&id),
            None => None
        };

        self.id_matiere_prog = self.matiere_prog.keys().max().unwrap_or(&0) + 1;

        egui::CentralPanel::default()
            .show(ctx, |ui| {
                //ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        //ajoute le prof si on clique sur valider ou sur la touche entrer
                        for (id_filiere, filiere) in self.filieres.iter() {
                            if ui.selectable_label(&self.selected_filiere_id == id_filiere,format!("{:}", filiere.get_name())).clicked() {                                  
                                self.selected_filiere_id = *id_filiere;
                                if !self.nb_sem.get(&self.selected_filiere_id).is_none() {
                                    self.new_nb_sem = self.nb_sem.get(&self.selected_filiere_id).unwrap().unwrap().to_string();
                                }
                            }
                        }
                    });
                    ui.end_row();
                    ui.horizontal(|ui| {
                        ui.label("Nombre de semaine(s) du programme: ");
                        let response_nb_sem = ui.text_edit_singleline(&mut self.new_nb_sem);
                        if response_nb_sem.lost_focus() {
                            match self.new_nb_sem.parse::<usize>() {
                                Ok(nombre) => {
                                    if nombre > 0 {
                                        self.nb_sem.insert(self.selected_filiere_id, Some(nombre));
                                    } else {
                                        self.new_nb_sem.clear();
                                    }  
                                },
                                Err(_) => {
                                    self.new_nb_sem.clear();
                                }
                            }
                        }
    
                        let ajout_filiere = ((ui.button("Afficher").clicked() || 
                            (response_nb_sem.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))) 
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
                    });
                    
                    //SELECTION DES INFORMATIONS SUR LA MATIERE A AJOUTER
                    if self.nb_sem_deja_valid.contains(&self.selected_filiere_id){

                        ui.horizontal(|ui| {
                            ui.label("Ajout d'une matière: ");

                            self.selected_matiere = 
                                    match self.liste_selected_matiere.get(&self.selected_filiere_id) {
                                        Some(heure) => heure.get_name(),
                                        None => String::new()
                                    };
                            egui::ComboBox::from_id_source("Matieres")
                                .selected_text(&self.selected_matiere)
                                .show_ui(ui, |ui| {
                                    for  (id,matiere) in self.matieres.iter() {
                                            //println!("{:?}",matiere.get_room_type());
                                            if ui.selectable_label(self.selected_matiere == matiere.get_name(), matiere.get_name()).clicked() {
                                                self.selected_matiere = (matiere.get_name()).to_string();
                                                //self.selected_matiere_id = *id;
                                                self.liste_selected_matiere.insert(self.selected_filiere_id, Arc::clone(matiere));
                                            }
                                    }
                                });

                                ui.label("Nombre d'heures: ");
                                let mut nb_heure =
                                    match self.new_nb_heure.get(&self.selected_filiere_id) {
                                        Some(heure) => heure.clone(),
                                        None => String::new()
                                    };                           
                                let response_nb_heure = ui.text_edit_singleline(&mut nb_heure);
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
                            });
                            ui.horizontal(|ui| {
                                let mut en_groupe =
                                    match self.selected_en_groupe.get(&self.selected_filiere_id) {
                                        Some(groupe) => *groupe,
                                        None => false
                                    };

                                if ui.checkbox(&mut en_groupe, format!("Cours en groupe?")).changed(){
                                    self.selected_en_groupe.insert(self.selected_filiere_id, en_groupe); 
                                }

                                if *self.selected_en_groupe.get(&self.selected_filiere_id).unwrap_or(&false) {
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
                                }
                                
                                
                                let mut en_groupe_inter =
                                    match self.selected_en_groupe_interclasse.get(&self.selected_filiere_id) {
                                        Some(groupe) => *groupe,
                                        None => false
                                    };

                                if ui.checkbox(&mut en_groupe_inter, format!("Cours interclasse?")).changed(){
                                    self.selected_en_groupe_interclasse.insert(self.selected_filiere_id, en_groupe_inter);
                                }
                            });
                        
                            //POUR FACILITER LA SAISIE DES COURS PRESENTS SUR PLUSIEURS SEMAINES
                            ui.label("Liste des semaines auxquelles l'ajouter: ");
                            egui::CollapsingHeader::new("Cliquez pour choisir")
                                .show(ui, |ui| {
                                    let options: Vec<usize> = 
                                                    self.semaines.clone()//.iter()
                                                    .keys()
                                                    .filter(|(id_filiere,_i)| {*id_filiere == self.selected_filiere_id})
                                                    .map(|(_id_filiere, i)| *i)
                                                    .collect();
                                    
                                    ui.vertical(|ui| {     
                                        egui::ScrollArea::both() 
                                        .auto_shrink([false, true])   
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

                        let i : usize = 0;
                        let ajout_matiere = ui.button("Ajouter").clicked() 
                            && self.liste_selected_matiere.contains_key(&self.selected_filiere_id)
                            && self.nb_heure.get(&self.selected_filiere_id).unwrap_or(&Some(i)) > &Some(i);

                        if ajout_matiere {   
                            //SAUVEGARDE DES MATIERE AJOUTER A CHAQUE FILIERE     
                            let nb_groupe = match self.nb_groupe.get(&self.selected_filiere_id) {
                                Some(Some(nb)) => nb,
                                Some(&None) => &0,
                                None => &0,
                            };
                            let i: usize = 0;                  
                            for (cle, semaine) in self.semaines.iter().filter(|(id,_semaine)| {id.0 == self.selected_filiere_id}){
                                if self.selected_semaines.contains_key(&(cle)){
                                    self.matiere_prog.insert(
                                        self.id_matiere_prog, 
                                        Arc::new(MatiereProg::new( self.id_matiere_prog,
                                                             Arc::clone(self.liste_selected_matiere.get(&self.selected_filiere_id).unwrap()),
                                                             self.nb_heure.get(&self.selected_filiere_id).unwrap().unwrap(), 
                                                             *self.selected_en_groupe.get(&self.selected_filiere_id).unwrap_or(&false),
                                                             *nb_groupe,
                                                             *self.selected_en_groupe_interclasse.get(&self.selected_filiere_id).unwrap_or(&false), 
                                                             Arc::clone(semaine)
                                                            )));
                                    self.id_matiere_prog += 1;
                                }
                            } 
                        }

                        //AFFICHAGE DES SEMAINES
                        ui.horizontal(|ui| {
                            for (id, _semaine) in self.semaines.iter().filter(|(id,_semaine)| {id.0 == self.selected_filiere_id}) {
                                if ui.selectable_label(self.selected_semaine_onglet.get(&self.selected_filiere_id) == Some(&id.1),format!("{:}", id.1)).clicked() {                                  
                                    self.selected_semaine_onglet.insert(self.selected_filiere_id, id.1);   
                                }
                            }
                        });
                        // AFFICHAGE DES MATIERES AJOUTEES A LA SEMAINE ET A LA FILIERE SELECTIONNEES
                        egui::ScrollArea::both().show(ui, |ui| {
                            egui::Grid::new("tableau")
                                .num_columns(6)
                                .striped(true)
                                .spacing((10.0,10.0))
                                .min_col_width(100.0)
                                .show(ui, |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.label("Matiere");
                                    });
                                    ui.vertical_centered(|ui| {
                                        ui.label("Nombre d'heure(s)");
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
                                        .filter(|(id, matiere_prog)| {matiere_prog.get_semaine().get_filiere().get_id() == self.selected_filiere_id 
                                        && Some(matiere_prog.get_semaine().get_id()) == self.selected_semaine_onglet.get(&self.selected_filiere_id)
                                        }) 
                                    {
                                        ui.vertical_centered(|ui| {
                                            ui.label(matiere.get_matiere().get_name());
                                        });
                                        ui.vertical_centered(|ui| {
                                            ui.label(format!("{:}", matiere.get_nb_heure()));
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

                                        /*ui.vertical(|ui| {
                                            for i in 0..2 {
                                                ui.horizontal(|ui| {
                                                    ui.label("Groupe");
                                                    if ui.button("-").clicked() {
                                                        // Supprimer le groupe
                                                    }
                                                    if i == 1 {
                                                        if ui.button("+").clicked() {
                                                            // Ajouter un groupe
                                                        }
                                                    }
                                                });
                                            }
                                        });*/

                                        ui.end_row();
                                    }
                                });
                        });







                    }

                });
    }
}
                