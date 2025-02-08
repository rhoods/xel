use eframe::egui;
//use egui::{Context, Ui, Color32, Align2, Frame, HashMap2};
use std::collections::HashMap;
use crate::struc::matiere::Matiere;
use crate::app::room_window::RoomType;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct MatiereWindow {
    pub show_teachers_window: bool,
    matieres: HashMap<usize, Arc<Matiere>>,
    new_matiere: String,
    type_salle: HashMap<usize, Arc<RoomType>>,

    selected_option: String,
    selected_option_id: Option<usize>,
    selected_teacher_id: Option<usize>,
    editing_teacher_id: Option<usize>,
    supp_teacher_id: Option<usize>,

    //response_day: HashMap<usize, usize, bool>,
    selected_days: bool,
    id_selected_days: Option<usize>,//HashMap<usize, usize, bool>, //num du jour et bool: selectionné ou non
    selected_hours:HashMap<usize, bool>,

    id_matiere: usize,

    window_position: egui::Pos2, // Coordonnées (x, y) pour afficher les fenêtres
}

impl Default for MatiereWindow {
    fn default() -> Self {
        Self {
            show_teachers_window: true,
            matieres:  HashMap::new(),
            new_matiere: String::new(),
            type_salle: HashMap::new(),
            selected_option: String::new(),
            selected_option_id: None,
            selected_teacher_id: None,
            editing_teacher_id: None,
            supp_teacher_id: None,
           // response_day: HashMap::new(),
            selected_days: false,
            id_selected_days: None,//HashMap::new(),
            selected_hours: HashMap::new(),

            id_matiere: 1,

            window_position: egui::Pos2::new(0.0, 0.0), // Par exemple, x=200, y=100
        }
    }
}


impl MatiereWindow {
    
    pub fn charge(&mut self,  matieres:HashMap<usize, Arc<Matiere>> , type_salle: HashMap<usize, Arc<RoomType>>) {
        self.matieres = matieres;
        self.type_salle = type_salle;   
    }


    pub fn get_liste_matiere(&self) -> &HashMap<usize, Arc<Matiere>>{
        &self.matieres
    }

    pub fn build(&mut self, ctx: &egui::Context) {

        self.id_matiere = *self.matieres.keys().max().unwrap_or(&0) + 1;
        //self.id_classe = *self.classe.keys().max().unwrap_or(&0) + 1;
        egui::Window::new("Création des matière")
            .current_pos(self.window_position)
            .open(&mut self.show_teachers_window)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    //ajoute le prof si on clique sur valider ou sur la touche entrer
                    ui.label("Matière :");
                    let response = ui.text_edit_singleline(&mut self.new_matiere);

                    ui.label("Type de salle :");
                    let _response_type_salle = 
                        egui::ComboBox::from_id_source("Type de salle")
                            .selected_text(&self.selected_option)
                            .show_ui(ui, |ui| {
                                for  (id,option) in self.type_salle.iter() {
                                        if ui.selectable_label(self.selected_option == option.get_name() /*&& !self.selected_option.is_empty()*/, option.get_name()).clicked() {
                                            self.selected_option = (option.get_name()).to_string();
                                            self.selected_option_id = Some(*id);
                                        }
                                }
                            });

                    let ajout_prof = (ui.button("Ajouter").clicked() || 
                        (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))) 
                        && !self.new_matiere.is_empty()
                        && !self.selected_option.is_empty();
                        
                    if ajout_prof {
                        
                        let mut doublon = false;
                        for (_id, matiere) in self.matieres.iter(){
                            //doublon que si le type de matiere est également identifique?
                            if &matiere.get_name() == &self.new_matiere {
                                doublon = true;
                                break;
                            }
                        }
                        if !doublon {
                            self.matieres.insert(
                                self.id_matiere,
                                Arc::new(Matiere::new(self.id_matiere, self.new_matiere.clone(), Arc::clone(self.type_salle.get(&self.selected_option_id.unwrap()).unwrap()))),
                            );
                            self.new_matiere.clear();
                            self.id_matiere += 1;
                        }
                        response.request_focus(); //remet le focus sur le zone de texte
                    }
                    
                });

                //si on a cliquer sur le bouton de suppression, delete le prof saisie
                if self.supp_teacher_id != None {
                    self.matieres.remove(&self.supp_teacher_id.unwrap());
                    self.id_matiere -= 1;
                    self.supp_teacher_id = None;
                }

                ui.vertical(|ui| {
                    egui::ScrollArea::both() // Activer le défilement vertical et horizontal
                    .auto_shrink([false, true]) // Permet à la zone de se rétrécir horizontalement, mais de ne pas se rétrécir verticalement
                    .show(ui, |ui| {
                        egui::Grid::new("rooms_by_type")
                            .striped(true)
                            .show(ui, |ui| {
                                // En-têtes : un pour chaque type de matiere
                                for (id, type_salle) in self.type_salle.iter() { 
                                    ui.label(type_salle.get_name());
                                }
                                ui.end_row();
                                // Données : afficher les matieres sous chaque type
                                
                                //détermine le nombre de ligne du tableau
                                let max_salles_par_type = self.type_salle
                                    .keys()
                                    .map(|id_type| self.matieres.values().filter(|matiere| matiere.get_room_type().get_id() == *id_type).count())
                                    .max()
                                    .unwrap_or(0);


                                for i in 0..max_salles_par_type {
                                    for  (id, type_salle) in self.type_salle.iter() {
                                        let matieres: Vec<_> = self.matieres
                                            //.values()
                                            .iter()
                                            .filter(|( id, room)| room.get_room_type().get_id() == type_salle.get_id())
                                            .map(|(id,classe)|classe.clone())
                                            .collect();

                                        if let Some(matiere) = matieres.get(i) {
                                    
                                            //ui.horizontal sert à faire en sorte que les boutons ne s'affiche pas sur les autre colonnes et que la taille de la colonne s'adapte
                                            ui.horizontal( |ui|{
                                                if ui.selectable_label(
                                                    self.selected_teacher_id == Some(*matiere.get_id()),
                                                    
                                                    matiere.get_name(),
                                                )
                                                .clicked()
                                                {
                                                    self.selected_teacher_id = Some(*matiere.get_id());
                                                }
                                                
                                                if self.selected_teacher_id  == Some(*matiere.get_id()){
                                                    /*if ui.button("Modifier").clicked() {
                                                        self.editing_room_id =  self.selected_room_id
                                                    }*/
                                                    if ui.button("❌").clicked() {
                                                        self.supp_teacher_id =  self.selected_teacher_id  
                                                    } 
                                                }
                                            });
                                        } else {
                                            ui.label(""); // Cellule vide si pas de matieres
                                        }
                                    }
                                    ui.end_row();
                                }
                            });

                    });
            });
    });
}
    
}

/*impl Clone for MatiereWindow {
    fn clone(&self) -> Self {
        Self {
            show_teachers_window: self.show_teachers_window,
            matieres: self.matieres.clone(),
            new_matiere: self.new_matiere.clone(),

            selected_teacher_id: self.selected_teacher_id,
            editing_teacher_id: self.editing_teacher_id,
            supp_teacher_id: self.supp_teacher_id,

            selected_days: self.selected_days,
            id_selected_days: self.id_selected_days,
            selected_hours: self.selected_hours.clone(),

            id_matiere: self.id_matiere,

            window_position: self.window_position,
        }
    }
}*/
            