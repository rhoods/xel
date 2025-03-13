use eframe::egui::{self, Button};
use egui::{/*Context, Ui,*/ Color32, /*Align2,*/ Frame, Vec2, Sense, Label, Rect, Id};
use crate::struc::teacher::{Teacher, Etat};
use std::collections::HashMap;
use crate::struc::horaire::{CreneauxEtablissement, TypeCreneau};

#[derive(Clone, Debug)]
pub struct HoraireWindow {
    
    horaires: HashMap<(usize,usize), CreneauxEtablissement>,
    new_heure: String,
    new_jour: String,
    
    
    editing_jour_id: Option<usize>,
    editing_hour_id: Option<usize>,
    supp_teacher_id: Option<usize>,

    //response_day: HashMap<usize, usize, bool>,
    selected_days: bool,
    id_selected_days: Option<usize>,//HashMap<usize, usize, bool>, //num du jour et bool: selectionné ou non
    selected_hours:HashMap<usize, bool>,

    //id_teacher: usize,

    window_position: egui::Pos2, // Coordonnées (x, y) pour afficher les fenêtres
}

impl Default for HoraireWindow {
    fn default() -> Self {
        Self {
            
            horaires:  HashMap::new(),
            new_jour: String::new(),
            new_heure: String::new(),

            editing_jour_id: None,
            editing_hour_id: None,
            supp_teacher_id: None,
           // response_day: HashMap::new(),
            selected_days: false,
            id_selected_days: None,//HashMap::new(),
            selected_hours: HashMap::new(),

            //id_teacher: 0,

            window_position: egui::Pos2::new(0.0, 0.0), // Par exemple, x=200, y=100
        }
    }
}


impl HoraireWindow {

    pub fn charge(&mut self, horaires: HashMap<(usize,usize), CreneauxEtablissement>,) {
        
        self.horaires = horaires;
    }

    pub fn get_liste_horaires(&self) -> HashMap<(usize,usize), CreneauxEtablissement> {
        self.horaires.clone()
    }


    pub fn build(&mut self, ctx: &egui::Context) {
        egui::Window::new("Gestion des professeurs")
            .current_pos(self.window_position)
            .show(ctx, |ui| {
                
                    //ajoute le prof si on clique sur valider ou sur la touche entrer
                    ui.horizontal(|ui|{
                        ui.label("Nombre de jours");
                        let response_jour = ui.text_edit_singleline(&mut self.new_jour);

                        ui.label("Nombre d'heures par jour");
                        let response_heure = ui.text_edit_singleline(&mut self.new_heure);

                        let ajout_prof = (ui.button("Ajouter").clicked() || 
                            (response_heure.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))) 
                            && !self.new_jour.is_empty() && !self.new_heure.is_empty();
                    
                        
                        if ajout_prof {
                
                            for n_jour in 0..self.new_jour.parse::<usize>().unwrap(){
                                for n_heure in 0..self.new_heure.parse::<usize>().unwrap(){
                                    self.horaires.insert((n_jour,n_heure),CreneauxEtablissement::new(n_jour, n_heure));
                                }
                            }
                        }
                    });
                    
                    ui.end_row();
                    
                    let nb_jour = self.horaires.keys().map(|(id_jour,_id_heure)| { id_jour}).max();
                    let nb_heure = self.horaires.keys().map(|(_id_jour,id_heure)| { id_heure}).max();

                    if !nb_jour.is_none() && !nb_heure.is_none(){
                        
                        let mut liste_id_jours: Vec<usize> = vec![0; nb_jour.unwrap() + 1];
                        let mut liste_id_heures: Vec<usize> = vec![0; nb_heure.unwrap() + 1];
                        let mut jours: Vec<String> = vec![String::new(); nb_jour.unwrap() + 1];
                        let mut heures: Vec<String> = vec![String::new(); nb_heure.unwrap() + 1];
                        for ((id_jour, id_heure), creneau) in self.horaires.iter(){
                            liste_id_jours[*id_jour]= *id_jour;
                            liste_id_heures[*id_heure] = *id_heure;

                            jours[*id_jour]= format!("{:}",creneau.get_name_jour());
                            heures[*id_heure] = format!("{:}",creneau.get_name_heure());
                        }
                        liste_id_jours.sort();
                        liste_id_heures.sort();
                     
                        if !self.editing_hour_id.is_none(){
                            
                                let mut new_name = heures[self.editing_hour_id.unwrap()].clone();
                                let response = ui.text_edit_singleline(&mut new_name);
                                //response.has_focus();
                                if response.changed() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                                    for ((_, _), creneau) in self.horaires.iter_mut()
                                        .filter(|((_, id_h), _)| self.editing_hour_id.unwrap() == *id_h) {
                                        creneau.set_name_heure(new_name.clone());
                                    }
                                    if (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))){
                                        self.editing_hour_id = None;
                                        
                                    }
                                } 
                            
                        }

                        if !self.editing_jour_id.is_none(){
                            let mut new_name = jours[self.editing_jour_id.unwrap()].clone();
                            let response = ui.text_edit_singleline(&mut new_name);
                            if response.changed() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                                for ((id_j, id_h), creneau) in self.horaires.iter_mut()
                                    .filter(|((id_j, id_h), _)| self.editing_jour_id.unwrap() == *id_j) {
                                    creneau.set_name_jour(new_name.clone());
                                }
                                if (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))){
                                    self.editing_jour_id = None;
                                }
                            } 
                        }

                        let cell_size = Vec2::new(100.0, 30.0);

                        ui.visuals_mut().selection.bg_fill = Color32::TRANSPARENT; //enleve l'effet de selection sur les label du planning (changement couleur du fond)
                        ui.separator();
                        ui.heading("Planning de l'établissement");
                        egui::Grid::new("schedule_grid")
                        .show(ui, |ui| {
                            ui.vertical(|ui|{
                                ui.horizontal(|ui|{
                                    ui.add_sized(cell_size,egui::Label::new(""));
                                    for (id_jour,jour) in liste_id_jours.iter().enumerate() {
                                        ui.vertical(|ui|{
                                            let response_day= ui.add_sized(cell_size, egui::SelectableLabel::new(false, jours[id_jour].clone()));
                                            if response_day.clicked() {
                                                self.editing_jour_id =  Some(id_jour);
                                            }
                                        });
                                    }
                                });
                                ui.end_row();

                                for (id_heure,heure) in liste_id_heures.iter().enumerate() {
                                    ui.horizontal(|ui|{
                                        let response_hours = ui.add_sized(cell_size, egui::SelectableLabel::new(false,heures[id_heure].clone()));
                                        
                                        if response_hours.clicked() {
                                            self.editing_hour_id =  Some(id_heure);
                                        }

                                        for (id_jour,_jour) in liste_id_jours.iter().enumerate() {
                                            let creneau = self.horaires.get_mut(&(id_jour, id_heure)).unwrap();
                                            let dispo = creneau.get_dispo();
                                            let (bg_color, text_color, text) = match dispo {
                                                TypeCreneau::Actif => (Color32::from_rgb(81, 121, 53), Color32::WHITE, "Actif"),
                                                TypeCreneau::Repas => (Color32::from_rgb(53, 77, 121), Color32::WHITE, "Repas"),
                                                TypeCreneau::Desactive => (Color32::from_rgb(63, 63, 63), Color32::WHITE, "Desactive"),
                                            };

                                            let response =  ui.add(Button::new(egui::RichText::new(text).color(text_color))
                                                                                    .fill(bg_color) // Change la couleur de fond
                                                                                    .min_size(Vec2::new(100.0,50.0))
                                                                                    .sense(egui::Sense::click())
                                                                                    .frame(false)
                                                                            );
            
                                            if response.clicked() {
                                                creneau.update();
                                            }
                                        } 
                                        ui.end_row();
                                    });    
                                } 
                            });
                        });
                    }
            });
    }
} 