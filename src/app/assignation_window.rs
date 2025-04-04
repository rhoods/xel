
use eframe::egui;
use std::sync::{Arc, Mutex};

use rusqlite::{params, Connection, Result};
//use egui::{Context, Ui, Color32, Align2, Frame, Vec2};
use std::collections::{HashMap, HashSet};
use crate::struc::matiere::Matiere;
use crate::struc::teacher::{self, Teacher};
use crate::struc::assignation::{Assignation, Groupe};
use crate::struc::programme::{MatiereProg, Semaine, MatiereInterClasse}; 
use crate::app::filiere_window::{Filiere, Classe};
use crate::app::room_window::{Room,RoomType};


#[derive(Clone, Debug)]
pub struct AssignationWindow {
    id_groupe: usize,
    id_assignement: usize,
    assignement : HashMap<usize, Arc<Assignation>>,
    groupe: HashMap<usize, Arc<Groupe>>,
    semaine: HashMap<(usize,usize),Arc<Semaine>>,
    classe: HashMap<usize,Arc<Classe>>,
    filiere: HashMap<usize,Arc<Filiere>>,
    matiere: HashMap<usize,Arc<Matiere>>,
    selected_classe: HashMap<(usize,usize), usize>,
    matiere_prog:   HashMap<usize, Arc<MatiereProg>>, 
    matiere_inter_classe:   HashMap<usize, Arc<MatiereInterClasse>>,
    teachers: HashMap<usize, Teacher>,
    selected_filiere_id: usize, 
    selected_classe_id: Option<usize>, 
    cours_a_afficher: HashMap<(usize,usize), Arc<MatiereProg>>,
    selected_option: HashMap<(usize,usize, usize), String>, //id_classe, id_matiere, id_groupe
    selected_option_id: Option<usize>,
    selected_prof: HashMap<(usize,usize,usize), usize>, //(id_classe, id_matiere, id_groupe), id_prof
    selected_all: HashMap<(usize, usize),bool>, //(id_classe,id_matiere_prog)
    selected_liste_classe: HashMap<(usize,usize,usize), usize>, //(id_classe,id_matiere_prog i), id_classe   ////Arc<Classe>
}


impl  Default for AssignationWindow  {
    fn default() -> Self {
        Self {
            id_groupe: 0,
            id_assignement: 0,
            assignement: HashMap::new(),
            groupe: HashMap::new(),
            semaine: HashMap::new(),
            classe: HashMap::new(),
            filiere: HashMap::new(),
            matiere: HashMap::new(),
            selected_classe: HashMap::new(),
            matiere_prog:   HashMap::new(), //
            matiere_inter_classe:   HashMap::new(),
            teachers: HashMap::new(),
            selected_filiere_id: 0, 
            selected_classe_id: None,
            cours_a_afficher: HashMap::new(),
            selected_option: HashMap::new(),
            selected_option_id: None,
            selected_prof: HashMap::new(),
            selected_all: HashMap::new(),
            selected_liste_classe: HashMap::new(),
        }
    }
}


impl AssignationWindow {

    pub fn get_groupe(&self) -> &HashMap<usize, Arc<Groupe>>{
        &self.groupe
    }

    pub fn get_assignement(&self) -> &HashMap<usize, Arc<Assignation>>{
        &self.assignement                                                                                                                                                                          
                                                                                                                                                                                                               
    }

    pub fn get_selected_inter_classe(&self) -> &HashMap<(usize,usize,usize), usize>{
        &self.selected_liste_classe                                                                                                                                                                          
                                                                                                                                                                                                               
    }

    pub fn charge(&mut self, semaine: HashMap<(usize,usize), Arc<Semaine>>, classe: HashMap<usize, Arc<Classe>>, filiere: HashMap<usize, Arc<Filiere>>, matiere:HashMap<usize, Arc<Matiere>>,   matiere_prog: HashMap<usize, Arc<MatiereProg>>, matiere_inter_classe: HashMap<usize, Arc<MatiereInterClasse>>, teachers: HashMap<usize, Teacher>, groupe: HashMap<usize, Arc<Groupe>>,  assignement :HashMap<usize, Arc<Assignation>>, selected_liste_classe: HashMap<(usize,usize,usize), usize>) {
        
        self.semaine = semaine;
        self.classe =  classe;
        self.filiere = filiere;
        
        self.matiere = matiere;
        self.matiere_prog = matiere_prog;
        self.matiere_inter_classe =  matiere_inter_classe;
        self.teachers =  teachers;
        self.groupe = groupe;
        //dbg!(&self.assignement);
        self.assignement = assignement;
        //self.salles_type = salles_type;   
        for (id, assignation) in self.assignement.iter(){
            let prof = assignation.get_prof();
            let classe = assignation.get_classe();
            let matiere =  assignation.get_matiere();
            let groupe =  assignation.get_groupe();
            //dbg!(&prof);
            //dbg!(&classe);
            //dbg!(&matiere);
            //dbg!(&groupe);
            self.selected_prof.insert((classe.get_id(), *matiere.get_id(), *groupe.get_id()), prof.get_id());
            //dbg!(&self.selected_prof);
            self.selected_option.insert((classe.get_id(), *matiere.get_id(), *groupe.get_id()), prof.get_name());
        }
        self.selected_liste_classe = selected_liste_classe;
    }

    pub fn build(&mut self, ctx: &egui::Context,) /*-> Result<()>*/ {

        self.id_groupe = *self.groupe.keys().max().unwrap_or(&0) + 1;
        self.id_assignement = *self.assignement.keys().max().unwrap_or(&0) + 1;
        let conn = Connection::open("C:/Users/admin/source/repos/xel/bdd/bdd.db").expect("Impossible de se connecter à la base de données");
                        
        egui::CentralPanel::default()
            .show(ctx, |ui| {

                    ui.horizontal(|ui| {
                        for (id_filiere, filiere) in self.filiere.iter() {
                            if ui.selectable_label(&self.selected_filiere_id == id_filiere,format!("{:}", filiere.get_name())).clicked() {                                  
                                self.selected_filiere_id = *id_filiere;
                                self.selected_classe_id = None;
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        for (id, classe) in self.classe.iter().filter(|(id,classe)| {classe.get_filiere().get_id() == self.selected_filiere_id}){
                            if ui.selectable_label(self.selected_classe_id == Some(*id),format!("{:}", classe.get_name())).clicked() {                                  
                                self.selected_classe_id = Some(*id);
                            }
                        }
                    });
                    if !self.selected_classe_id.is_none(){
                        egui::ScrollArea::both().show(ui, |ui| {
                            egui::Grid::new("tableau")
                                .num_columns(7)
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
                                        ui.label("Groupe(s)");
                                    });
                                    ui.vertical_centered(|ui| {
                                        ui.label("Professeurs");
                                    });
                                    ui.vertical_centered(|ui| {
                                        ui.label("Cours inter_classe?");
                                    });
                                    ui.vertical_centered(|ui| {
                                        ui.label("Classes participantes");
                                    });
                                    ui.label(" ");
                                    ui.end_row();
                        
                                    let mut matiere_id: usize;
                                    for (id, matiere_prog) in self.matiere_prog.iter()
                                        .filter(|(_id, matiere_prog)| {matiere_prog.get_semaine().get_filiere().get_id() == self.selected_filiere_id 
                                        //&& Some(matiere_prog.get_matiere().get_id()) == self.selected_semaine_onglet.get(&self.selected_filiere_id)
                                        }) 
                                    {
                                        matiere_id = *matiere_prog.get_matiere().get_id();
                                        if !self.cours_a_afficher.contains_key(&(self.selected_filiere_id, matiere_id)){
                                            self.cours_a_afficher.insert((self.selected_filiere_id, matiere_id),Arc::clone(matiere_prog));
                                        }
                                    }
    
                                    let mut f_i: usize = 0;
                                    for (id_matiere_prog, matiere_prog) in self.cours_a_afficher.iter()
                                        .filter(|(id, matiere_prog)| {id.0 == self.selected_filiere_id 
                                        //&& Some(matiere_prog.get_matiere().get_id()) == self.selected_semaine_onglet.get(&self.selected_filiere_id)
                                        }) 
                                    {    
                                        //ui.horizontal(|ui| {
                                            ui.vertical_centered(|ui| {
                                                ui.label(matiere_prog.get_matiere().get_name());
                                            });
                                            ui.vertical_centered(|ui| {
                                                ui.label(format!("{:}", matiere_prog.get_nb_heure()));
                                            });
                                           // ui.vertical_centered(|ui| {
                                              
                                                ui.vertical(|ui| {
                                                    for i in 0..*matiere_prog.get_nb_groupe() {
                                                        ui.horizontal(|ui| {
                                                            ui.label(format!("Groupe {:}", i + 1));
                                                            if ui.button("-").clicked() {
                                                                // Supprimer le groupe
                                                            }
                                                            if i == *matiere_prog.get_nb_groupe() - 1 {
                                                                if ui.button("+").clicked() {
                                                                    // Ajouter un groupe
                                                                }
                                                            }
                                                            ui.end_row();
                                                        });
                                                    }
                                                });
    
                                                ui.vertical(|ui| {
                                                    let mut nb_groupe = *matiere_prog.get_nb_groupe();
                                                    if nb_groupe == 0 { //si 0 groupes n'affiche pas de combobox donc on alimente à 1
                                                        nb_groupe = 1;
                                                    }
                                                    //for i in 0..nb_groupe {
                                                    for (id, groupe) in self.groupe.iter().filter(|(id,groupe)|{ groupe.get_classe().get_id() == self.selected_classe_id.unwrap() && groupe.get_matiere().get_id() == matiere_prog.get_matiere().get_id()}){

                                                        //  let select = match self.selected_option.get(&(self.selected_classe_id.unwrap(),f_i)){
                                                        let mut select = match self.selected_option.get(&(self.selected_classe_id.unwrap(),*matiere_prog.get_matiere().get_id(), *id)){
                                                            
                                                            Some(sel) => {sel.clone()},
                                                            None => 
                                                                
                                                                if let Some((id,name)) = self.teachers.iter().next(){
                                                                    name.get_name()
                                                                }else{
                                                                    " ".to_string()
                                                                },
                                                        };
                                                        
                                                        //dbg!(&self.selected_option.get(&(self.selected_classe_id.unwrap(),*matiere_prog.get_matiere().get_id(), i)));
                                                        ui.vertical_centered(|ui| {
                                                                egui::ComboBox::from_id_source(format!("Professeurs {:}", f_i))
                                                                    .selected_text(&select)
                                                                    .show_ui(ui, |ui| {
                                                                        for  (id_teacher,option) in self.teachers.iter() {
                                                                                if ui.selectable_label(select == option.get_name() /*&& !self.selected_option.is_empty()*/, option.get_name()).clicked() {
                                                                                    //self.selected_option.insert(f_i, option.get_name());
                                                                                    let id_matiere = *matiere_prog.get_matiere().get_id();
                                                                                    let id_classe = self.selected_classe_id.unwrap();
                                                                                    let id_groupe = *id;
                                                                                    let classe = self.classe.get(&id_classe).unwrap();
                                                                                    let matiere = self.matiere.get(&id_matiere).unwrap();
                                                                                    let option_programme = matiere_prog.get_option();
                                                                                    self.selected_option_id = Some(*id);
                                                                                    self.selected_prof.insert((self.selected_classe_id.unwrap(), *matiere_prog.get_matiere().get_id(), id_groupe), *id_teacher);
                                                                                    self.selected_option.insert((self.selected_classe_id.unwrap(),*matiere_prog.get_matiere().get_id(), id_groupe), option.get_name());
                                                                                   
                                                                                    let mut verif_assign_exists = conn.prepare("SELECT id FROM Assignement WHERE id_matiere = ?1 and id_classe = ?2 and id_groupe = ?3").expect("Impossible de préparer la requete");
                        
                                                                                        let mut rows = verif_assign_exists.query(params![id_matiere, id_classe, id_groupe]).expect("Impossible d'executer la requete'");
                                                                                        let mut id_ass: Result<Option<usize>> = Ok(None);
                                                                                        //for row in rows {
                                                                               
                                                                                        while let Some(row) = rows.next().expect("Impossible de lire la base de données") {
                                                                                            id_ass = row.get(0); //.expect("Impossible de recupérer la données dans la bdd")
                                                                                   
                                                                                        }
                                                                                        if id_ass.is_ok(){
                                                                                            match id_ass.unwrap() {
                                                                                                Some(id_assignement) => {
                                                                                                                                self.assignement.insert(id_assignement, Arc::new(Assignation::new(id_assignement,Arc::clone(classe), Arc::clone(matiere), Arc::clone(groupe), option.clone(), Arc::clone(&option_programme), Arc::clone(matiere_prog))));
                                                                                                                                },
                                                                                                None => {
                                                                                                            self.assignement.insert(self.id_assignement, Arc::new(Assignation::new(self.id_assignement,Arc::clone(classe), Arc::clone(&matiere), Arc::clone(groupe), option.clone(), Arc::clone(&option_programme), Arc::clone(matiere_prog))));
                                                                                                            self.id_assignement += 1;
                                                                                                        },
                                                                                            }
                                                                                        }
                                                                                }
                                                                        }
                                                                    });
                                                            });
                                                        f_i += 1;
                                                    }
                                            });
                                           
    
                                            ui.vertical_centered(|ui| {
                                                ui.label(format!("{:}", matiere_prog.get_en_groupe_inter_classe().to_string()));
                                            });
                                            if *matiere_prog.get_en_groupe_inter_classe(){
                                                egui::CollapsingHeader::new("Sélection des classes")
                                                .id_source(id_matiere_prog)
                                                .show(ui, |ui| {
                                                    let options: Vec<Arc<Classe>> = 
                                                                    self.classe.clone()//.iter()
                                                                    .values()
                                                                    .filter(|classe| {classe.get_filiere().get_id() == self.selected_filiere_id /*&& self.selected_classe_id != Some(classe.get_id())*/})
                                                                    .map(|classe| Arc::clone(classe))
                                                                    .collect();
                                                    
                                                    ui.vertical(|ui| {     
                                                        egui::ScrollArea::both() 
                                                        .id_source(id_matiere_prog)
                                                        .auto_shrink([false, true])   
                                                        .show(ui, |ui| {  
                                                            
                                                            /*let mut i: usize = 0;
                                                            let mut selected = match self.selected_all.get(&(self.selected_classe_id.unwrap(),id_matiere_prog.1)){
                                                                Some(select) => *select,
                                                                None => false,
                                                            };
                                                        
                                                            if ui.checkbox(&mut selected, format!("Toutes")).changed() {
                                                                self.selected_all.insert((self.selected_classe_id.unwrap(), id_matiere_prog.1), selected);
                                                                for (_cle,option) in options.iter().enumerate(){
                                                                    if selected{
                                                                        self.selected_liste_classe.insert((self.selected_classe_id.unwrap(), id_matiere_prog.1,option.get_id()),self.selected_classe_id.unwrap());   //Arc::clone(option)
                                                                    }else {
                                                                        self.selected_liste_classe.remove(&(self.selected_classe_id.unwrap(), id_matiere_prog.1, option.get_id()));
                                                                    }
                                                                    i += 1 ;
                                                                }
                                                            }*/
    
                                                            //i = 0;
                                                            for (_cle,option) in options.iter().enumerate(){
                                                                //for (_cle2,option2) in options.iter().enumerate(){
                                                                    let mut selected = self.selected_liste_classe.contains_key(&(self.selected_classe_id.unwrap(), id_matiere_prog.1, option.get_id()));
                                                                    //let mut selected = self.selected_liste_classe.contains_key(&(option.get_id(), id_matiere_prog.1, option.get_id()));
                                                                    if ui.checkbox(&mut selected, format!("{:}",option.get_name())).changed() {
                                                                        
                                                                            if selected {
                                                                                self.selected_liste_classe.insert((self.selected_classe_id.unwrap(), id_matiere_prog.1,option.get_id()), self.selected_classe_id.unwrap());// Arc::clone(option)
                                                                                //self.selected_liste_classe.insert((option.get_id(), id_matiere_prog.1,option.get_id()), option.get_id());
                                                                            } else {
                                                                                //self.selected_liste_classe.remove(&(option.get_id(), id_matiere_prog.1,option.get_id()));
                                                                                //self.selected_all.insert((option.get_id(),id_matiere_prog.1), false);
                                                                                self.selected_liste_classe.remove(&(self.selected_classe_id.unwrap(),id_matiere_prog.1, option.get_id()));
                                                                                self.selected_all.insert((self.selected_classe_id.unwrap(),id_matiere_prog.1), false);
                                                                            }
                                                                        
                                                                    }
                                                                //}
                                                               // i += 1 ;
                                                            }
                                                            
                                                            let mut selected_liste_classe: HashMap<(usize,usize,usize), usize> = HashMap::new();
                                                            for ((classe_saisie, matiere,classe), _val) in self.selected_liste_classe.iter(){
                                                                selected_liste_classe.insert((*classe_saisie, *matiere, *classe), *classe_saisie);
                                                                selected_liste_classe.insert((*classe, *matiere, *classe), *classe);
                                                                selected_liste_classe.insert((*classe, *matiere, *classe_saisie), *classe);
                                                            }
                                                            self.selected_liste_classe = selected_liste_classe;
    
                                                        });
                                                    });
                                                });
                                            
                                            } else {
                                                ui.label(" ");
                                            }
                                        ui.end_row(); 
                                    }                                     
                                });
                        });                       
                    }
            });
    }
}





