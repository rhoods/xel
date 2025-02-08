use eframe::egui;
use egui::{/*Context, Ui,*/ Color32, /*Align2,*/ Frame, Vec2};
use crate::struc::teacher::Teacher;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TeacherWindow {
    pub show_teachers_window: bool,
    teachers: HashMap<usize, Teacher>,
    new_teacher: String,
    
    
    selected_teacher_id: Option<usize>,
    editing_teacher_id: Option<usize>,
    supp_teacher_id: Option<usize>,

    //response_day: HashMap<usize, usize, bool>,
    selected_days: bool,
    id_selected_days: Option<usize>,//HashMap<usize, usize, bool>, //num du jour et bool: selectionné ou non
    selected_hours:HashMap<usize, bool>,

    id_teacher: usize,

    window_position: egui::Pos2, // Coordonnées (x, y) pour afficher les fenêtres
}

impl Default for TeacherWindow {
    fn default() -> Self {
        Self {
            show_teachers_window: true,
            teachers:  HashMap::new(),
            new_teacher: String::new(),

            selected_teacher_id: None,
            editing_teacher_id: None,
            supp_teacher_id: None,
           // response_day: HashMap::new(),
            selected_days: false,
            id_selected_days: None,//HashMap::new(),
            selected_hours: HashMap::new(),

            id_teacher: 0,

            window_position: egui::Pos2::new(0.0, 0.0), // Par exemple, x=200, y=100
        }
    }
}


impl TeacherWindow {
    
    pub fn get_liste_teacher(&self) -> &HashMap<usize, Teacher>{
        &self.teachers
    }

    pub fn charge(&mut self, teachers: HashMap<usize, Teacher>,) {
        self.teachers = teachers;
    }
    
    pub fn build(&mut self, ctx: &egui::Context) {

        self.id_teacher = *self.teachers.keys().max().unwrap_or(&0) + 1;
        //self.id_room = *self.rooms.keys().max().unwrap_or(&0) + 1;

        egui::Window::new("Gestion des professeurs")
            .current_pos(self.window_position)
            .open(&mut self.show_teachers_window)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    //ajoute le prof si on clique sur valider ou sur la touche entrer
                    let response = ui.text_edit_singleline(&mut self.new_teacher);
                    let ajout_prof = (ui.button("Ajouter").clicked() || 
                        (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))) 
                        && !self.new_teacher.is_empty();
                    if ajout_prof {
                        
                        let mut doublon = false;
                        for (_id, teacher) in self.teachers.iter(){
                            //doublon que si le type de salle est également identifique?
                            if &teacher.get_name() == &self.new_teacher {
                                doublon = true;
                                break;
                            }
                        }
                        if !doublon {
                            self.teachers.insert(
                                self.id_teacher,
                                Teacher::new(self.id_teacher, self.new_teacher.clone()),
                            );
                            self.new_teacher.clear();
                            self.id_teacher += 1;
                        }
                        response.request_focus(); //remet le focus sur le zone de texte
                    }
                    
                });

                //si on a cliquer sur le bouton de suppression, delete le prof saisie
                if self.supp_teacher_id != None {
                    self.teachers.remove(&self.supp_teacher_id.unwrap());
                    self.id_teacher -= 1;
                    self.supp_teacher_id = None;
                }


                //let teacher_ids: HashMap<usize, _> = self.teachers.cloned().collect();
                //for id in teacher_ids {
                for (id, teacher) in self.teachers.iter_mut() { 
                    //let teacher = self.teachers.get_mut(id).unwrap();
                    ui.horizontal(|ui| {
                        //modifie le nom du prof si on clique sur modifier
                        if self.editing_teacher_id == Some(*id) {
                            let mut new_name = teacher.get_name();
                            let response =  ui.text_edit_singleline(&mut new_name);
                            if response.changed() {
                                // Met à jour le nom en temps réel pendant que l'utilisateur tape
                                teacher.set_name(new_name.clone());
                            }
                            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                teacher.set_name(new_name.clone());
                                self.editing_teacher_id = None;
                            }
                        }
                        if ui.selectable_label(
                            self.selected_teacher_id == Some(*id),
                            
                            &teacher.get_name(),
                        )
                        .clicked()
                        {
                            self.selected_teacher_id = Some(*id);
                        }
                        if self.selected_teacher_id == Some(*id){
                            if ui.button("✏").clicked() {
                                self.editing_teacher_id =  self.selected_teacher_id
                            } 
                            if ui.button("❌").clicked() {
                                self.supp_teacher_id =  self.selected_teacher_id  
                            } 
                        }
                        
                    });
                }
                
                // Afficher le planning si un professeur est sélectionné
                if let Some(teacher_id) = self.selected_teacher_id {
                    if let Some(teacher) = self.teachers.get_mut(&teacher_id) {
                        
                        ui.visuals_mut().selection.bg_fill = Color32::TRANSPARENT; //enleve l'effet de selection sur les label du planning (changement couleur du fond)
                        ui.separator();
                        ui.heading("Planning de disponibilité");
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
                                let response_day= ui.add_sized(cell_size, egui::SelectableLabel::new(false, *day)); // ;label(*day);
                                //permet de cliquer sur le jour pour changer l'état des toutes les heures de la journée
                                if response_day.clicked() {
                                    for hour_idx in 0..hours.len() {
                                        teacher.set_availability(id_day, hour_idx);
                                    } 
                                }              
                            }

                            ui.end_row();
                            
                            for (hour_idx, hour) in hours.iter().enumerate() {
                                //genere les noms des plages horaires
                                let response_hours = ui.add_sized(cell_size, egui::SelectableLabel::new(false,*hour)); //ui.label(*hour);
                                //permet de cliquer sur l'heure pour changer l'état de cette plage horaire pour chaque journée
                                if response_hours.clicked() {
                                    for id_day in 0..days.len() {
                                        teacher.set_availability(id_day, hour_idx);
                                    } 
                                }
                                
                                for days_idx in 0..days.len() {

                                    let is_not_available = teacher.get_available(days_idx, hour_idx);
                                    
                                    // Définition des couleurs d'arrière-plan
                                    let (bg_color, text_color) = if is_not_available {
                                        (Color32::from_rgb(255, 200, 200), Color32::RED)  // Rouge clair pour indisponible
                                    } else {
                                        (Color32::from_rgb(200, 255, 200), Color32::DARK_GREEN)  // Vert clair pour disponible
                                    };
                                    let text = if is_not_available { "Indisponible" } else { "Disponible" };
                                    // Création d'un Frame aHashMap la couleur d'arrière-plan
                                    Frame::none()
                                        .fill(bg_color)
                                        .inner_margin(egui::style::Margin::symmetric(4.0, 0.0))
                                        .show(ui, |ui| {
                                            let response = ui.add_sized(
                                                cell_size,
                                                egui::SelectableLabel::new(
                                                    is_not_available,
                                                    egui::RichText::new(text).color(text_color)
                                                )
                                            );
                                            
                                            if response.clicked() {
                                                teacher.set_availability(days_idx, hour_idx);

                                            }
                                        });
                                }
                                ui.end_row();
                            }
                        });
                        
                    }
                }
            });
    }

}
