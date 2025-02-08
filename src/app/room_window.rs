use eframe::egui;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
//use egui::{Context, Ui, Color32, Align2, Frame, HashMap2};
//use crate::struc::room::{Room, RoomType};

#[derive( Debug)]
pub struct RoomType {
    id:Arc<Mutex<usize>>,
    name: Arc<Mutex<String>>,
}

impl Clone for RoomType {
    fn clone(&self) -> Self {
        Self {
            id: Arc::clone(&self.id),
            name: Arc::clone(&self.name), // Clone sûr grâce à Arc
        }
    }
}

impl RoomType {

    pub fn new(id: usize, name: String,) -> Self {
        Self {
            id: Arc::new(Mutex::new(id)),
            name: Arc::new(Mutex::new(name)),
        }
    }

    /*pub fn set_name(&mut self, new_name:String) { 
        let mut name = self.name.lock().unwrap();
        *name = new_name;
    }*/

    pub fn get_name(&self) -> String {
        let name = self.name.lock().unwrap();
        name.clone()
    }

    pub fn get_id(&self) -> usize {
        let id: std::sync::MutexGuard<'_, usize> = self.id.lock().unwrap();
        id.clone()
    }
}



#[derive(Clone, Debug)]
pub struct Room {
    id:usize, 
    name: String,
    room_type: Arc<RoomType>, // salle de tp, ou autre
}

impl Room {
    pub fn new( id: usize, name: String, room_type: Arc<RoomType>,) -> Self {
        Self {
            id,
            name,
            room_type,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_room_type(&self) -> Arc<RoomType> {
        Arc::clone(&self.room_type)
    }

}

#[derive(Clone, Debug)]
pub struct RoomWindow {
    rooms: HashMap<usize, Room>,
    rooms_type: HashMap<usize, Arc<RoomType>>,
    new_room: String,
    new_room_type: String,

    selected_option: String,
    selected_option_id: usize,

    selected_room_type_id: Option<usize>,
    editing_room_type_id: Option<usize>,
    supp_room_type_id: Option<usize>,

    selected_room_id: Option<usize>,
    editing_room_id: Option<usize>,
    supp_room_id: Option<usize>,
    
    id_room: usize,
    id_room_type:usize,

    window_position: egui::Pos2, // Coordonnées (x, y) pour afficher les fenêtres
    window_position2: egui::Pos2,
}

impl  Default for RoomWindow {
    fn default() -> Self {
        Self {
            rooms:  HashMap::new(),
            rooms_type: HashMap::new(),
            new_room: String::new(),
            new_room_type: String::new(),
            
            selected_option: String::new(),
            selected_option_id:0, 

            selected_room_type_id: None,
            editing_room_type_id: None,
            supp_room_type_id: None,

            selected_room_id: None,
            editing_room_id: None,
            supp_room_id: None,
            
            id_room: 0,
            id_room_type:0,
            window_position: egui::Pos2::new(0.0, 0.0), // Par exemple, x=200, y=100
            window_position2: egui::Pos2::new(380.0, 0.0),

           
         
        }
    }
}


impl  RoomWindow {

    pub fn charge(&mut self, room_type: HashMap<usize, Arc<RoomType>>, room: HashMap<usize, Room>) {
        self.rooms_type = room_type;
        self.rooms = room;
    }
    pub fn get_liste_type_salle(&self) -> HashMap<usize, Arc<RoomType>>{
        //self.rooms_type.values().map(|arc| (**arc).clone()).collect()
        self.rooms_type.clone()
    }
    pub fn get_liste_salle(&self) -> &HashMap<usize, Room>{
        &self.rooms
    }

    pub fn build(&mut self, ctx: &egui::Context) {
        
        self.id_room_type = *self.rooms_type.keys().max().unwrap_or(&0) + 1;
        self.id_room = *self.rooms.keys().max().unwrap_or(&0) + 1;
        //si premier passage, on place une valeur par defaut dans le type de salle
        if self.rooms_type.is_empty(){
            self.rooms_type.insert(self.id_room_type,Arc::new(RoomType::new(self.id_room_type, "Default".to_string())));
            self.id_room_type += 1;
        }
        //si on a cliquer sur le bouton de suppression, delete le prof saisie
        if self.supp_room_id != None {
            self.rooms.remove(&self.supp_room_id.unwrap());
            self.supp_room_id = None;
            self.selected_room_id = None;
        }
        
        let mut ids_to_remove = Vec::new();
        if self.supp_room_type_id != None {
            for (id, room) in self.rooms.iter(){
                if Some(room.get_room_type().get_id()) == self.supp_room_type_id {
                    ids_to_remove.push(*id);
                }
            }
            for id in ids_to_remove.into_iter().rev() {
                self.rooms.remove(&id);
            }

            self.rooms_type.remove(&self.supp_room_type_id.unwrap());
            self.supp_room_type_id = None;
        }

        //selection du type de salle
        egui::Window::new("Création des types de salles")
            .current_pos(self.window_position)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    //ui.text_edit_singleline(&mut self.new_room_type);
                    let response = ui.text_edit_singleline(&mut self.new_room_type);

                    let ajout_type_salle = (ui.button("Ajouter").clicked() || 
                    (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))) 
                    && !self.new_room_type.is_empty();
                    
                    
                    if ajout_type_salle {
                        let mut doublon = false;
                        for (_id, room_type) in self.rooms_type.iter(){
                            if room_type.get_name() == self.new_room_type{
                                doublon = true;
                                //ajout d'un message qui indique que la type existe deja
                                break;
                            }
                        }
                        if !doublon {
                            self.rooms_type.insert(self.id_room_type, Arc::new(RoomType::new(self.id_room_type, self.new_room_type.clone())));
                            self.new_room_type.clear();
                            self.id_room_type += 1;      
                        }
                        response.request_focus();
                    }
                });
                
                for (id, rooms_type) in self.rooms_type.iter_mut() {
                    
                    ui.horizontal(|ui| {
                        if self.editing_room_type_id == Some(*id) {
                            //let mut new_name = rooms_type.get_name();
                            let mut name_guard = rooms_type.name.lock().unwrap();
                            let mut new_name = name_guard.clone();
                            let response =  ui.text_edit_singleline(&mut new_name);
                            if response.changed() { 
                                *name_guard = new_name.clone();
                            }
                            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                *name_guard = new_name.clone();
                                self.editing_room_type_id = None;
                            }
                            drop(name_guard);
                        }

                    
                        if ui.selectable_label(
                            self.selected_room_type_id == Some(rooms_type.get_id()),
                            
                            rooms_type.get_name(),
                        )
                        .clicked()
                        {
                            self.selected_room_type_id = Some(rooms_type.get_id());
                        }
                        
                        if self.selected_room_type_id  == Some(rooms_type.get_id()){
                            if ui.button("✏").clicked() {
                                self.editing_room_type_id =  self.selected_room_type_id
                            }
                            if ui.button("❌").clicked() {
                                self.supp_room_type_id =  self.selected_room_type_id  
                            } 
                        }
                    });

                    //ui.label(rooms_type.get_name());
                }
            });
        
        //saisie du numero de salle
        egui::Window::new("Création des salles")
            .current_pos(self.window_position2)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Type de salle :");
                    let _response_type_salle = 
                        egui::ComboBox::from_id_source("Type de salle")
                            .selected_text(&self.selected_option)
                            .show_ui(ui, |ui| {
                                for  (id,option) in self.rooms_type.iter() {
                                        if ui.selectable_label(self.selected_option == option.get_name() /*&& !self.selected_option.is_empty()*/, option.get_name()).clicked() {
                                            self.selected_option = (option.get_name()).to_string();
                                            self.selected_option_id = *id;
                                        }
                                }
                            });
                    let response_salle = ui.text_edit_singleline(&mut self.new_room);
                    
                    let ajout_salle = (ui.button("Ajouter").clicked() || 
                    (response_salle.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))) 
                    && !self.new_room.is_empty();

                    if ajout_salle {
                        if let Some(selected_type) = self.rooms_type.get(&self.selected_option_id) {
                            let mut doublon = false;
                            for (id, room) in self.rooms.iter() {
                                if room.get_name() == &self.new_room && Arc::ptr_eq(&room.get_room_type(), selected_type) {
                                    doublon = true;
                                    break;
                                }
                            }
                            if !doublon {
                                self.rooms.insert(self.id_room, Room::new(self.id_room, self.new_room.clone(), Arc::clone(selected_type)));
                                self.new_room.clear();
                                self.id_room += 1;
                            }
                            response_salle.request_focus();
                        }

                    }
                });
                
                //affichage sous forme de tableau de la liste des salles sous chaque type de salle
                //let cell_size = HashMap2::new(100.0, 30.0);
                ui.vertical(|ui| {
                    egui::ScrollArea::both() // Activer le défilement vertical et horizontal
                    .auto_shrink([false, true]) // Permet à la zone de se rétrécir horizontalement, mais de ne pas se rétrécir verticalement
                    .show(ui, |ui| {
                        egui::Grid::new("rooms_by_type")
                            .striped(true)
                            .show(ui, |ui| {
                                // En-têtes : un pour chaque type de salle
                                for (id, type_salle) in self.rooms_type.iter() { 
                                    ui.label(type_salle.get_name());
                                }
                                ui.end_row();
                                // Données : afficher les salles sous chaque type
                                
                                //détermine le nombre de ligne du tableau
                                let max_salles_par_type = self.rooms_type
                                    .keys()
                                    .map(|(id_type)| self.rooms.values().filter(|room| room.get_room_type().get_id() == *id_type).count())
                                    .max()
                                    .unwrap_or(0);


                                for i in 0..max_salles_par_type {
                                    for  (id, type_salle) in self.rooms_type.iter() {
                                        let salles: Vec<_> = self.rooms
                                            //.values()
                                            .iter()
                                            .filter(|( id, room)| room.get_room_type().get_id() == type_salle.get_id())
                                            .map(|(id,classe)|classe.clone())
                                            .collect();

                                        if let Some(salle) = salles.get(i) {
                                      
                                            //ui.horizontal sert à faire en sorte que les boutons ne s'affiche pas sur les autre colonnes et que la taille de la colonne s'adapte
                                            ui.horizontal( |ui|{
                                                if ui.selectable_label(
                                                    self.selected_room_id == Some(salle.get_id()),
                                                    
                                                    salle.get_name(),
                                                )
                                                .clicked()
                                                {
                                                    self.selected_room_id = Some(salle.get_id());
                                                }
                                                
                                                if self.selected_room_id  == Some(salle.get_id()){
                                                    /*if ui.button("Modifier").clicked() {
                                                        self.editing_room_id =  self.selected_room_id
                                                    }*/
                                                    if ui.button("❌").clicked() {
                                                        self.supp_room_id =  self.selected_room_id  
                                                    } 
                                                }
                                            });
                                        } else {
                                            ui.label(""); // Cellule vide si pas de salles
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