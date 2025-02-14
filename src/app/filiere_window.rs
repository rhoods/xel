use eframe::egui;
use std::sync::{Arc, Mutex};
use crate::struc::teacher::Teacher;
use crate::struc::programme::Semaine;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Filiere {
    id: Arc<Mutex<usize>>,
    name: Arc<Mutex<String>>,
    pub nb_semaine:  Arc<Mutex<usize>>,
    programmes: Arc<Mutex<Option<HashMap<usize,Semaine>>>>,  //usize correspond au numéro de la semaine
}
/*impl Clone for Filiere {
    fn clone(&self) -> Self {
        Self {
            id: Arc::clone(&self.id),
            name: Arc::clone(&self.name), // Clone sûr grâce à Arc
            nb_semaine: self.nb_semaine,
            programmes:  Arc::clone(&self.programmes),
        }
    }
}*/

impl Filiere   {
    pub fn new( id: usize, name: String,) -> Self {
        Self {
            id: Arc::new(Mutex::new(id)),
            name: Arc::new(Mutex::new(name)),
            nb_semaine: Arc::new(Mutex::new(0)),
            programmes: Arc::new(Mutex::new(None)),
        }
    }
    pub fn charge( id: usize, name: String, nb_semaine: usize) -> Self {
        Self {
            id: Arc::new(Mutex::new(id)),
            name: Arc::new(Mutex::new(name)),
            nb_semaine: Arc::new(Mutex::new(nb_semaine)),
            programmes: Arc::new(Mutex::new(None)),
        }
    }

    /*pub fn update(&self, name: String, nb_semaine: usize,  programmes: Arc<Mutex<Option<HashMap<usize,Programme>>>>,)  -> Self {
        Self {
            id: self.id,
            name: self.name,
            nb_semaine,
            programmes,
        }
    }*/

    pub fn get_id(&self) -> usize {
        let id: std::sync::MutexGuard<'_, usize> = self.id.lock().unwrap();
        id.clone()
    }

    pub fn get_name(&self) -> String {
        let name = self.name.lock().unwrap();
        name.clone()
    }
    
    /*pub fn set_name(&mut self, new_name:String) {
        self.name = new_name;
    }*/

    pub fn get_liste_semaine(&self) -> Option<HashMap<usize,Semaine>> {
        let semaine: std::sync::MutexGuard<'_, Option<HashMap<usize,Semaine>>> = self.programmes.lock().unwrap();
        semaine.clone()
        }

        
        //&self.programmes
    

    /*pub fn get_programme_mut(&mut self) -> &mut Option<HashMap<usize,Programme>> {
        &mut self.programmes
    }*/

    pub fn get_nb_semaine(&self) -> Option<usize> {
        let name = self.nb_semaine.lock().unwrap();
        Some(name.clone())
    }

    /*pub fn set_nb_semaine(&mut self, nombre: usize){
        self.nb_semaine = nombre;
    }*/

}




#[derive(Clone, Debug)]
pub struct Classe{
    id: usize,
    filiere: Arc<Filiere>,
    name: String,
    teachers: Option<HashMap<usize,Teacher>>, // Clé : (jour, créneau), Valeur : TimeSlot
    nb_groupe:usize,
}

impl  Classe  {
    pub fn new(id:usize, filiere:Arc<Filiere>, name: String, teachers: Option<HashMap<usize,Teacher>>, nb_groupe:usize ) -> Self {
        Self {
            id,
            filiere,
            name,
            teachers,
            nb_groupe,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn set_id(&mut self, new_id: usize) {
        self.id = new_id;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn set_name(mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn get_nb_groupe(&self) -> usize {
        self.nb_groupe
    }
    pub fn set_nb_groupe(mut self, nombre: usize) {
        self.nb_groupe = nombre;
    }

    pub fn get_filiere(&self) -> Arc<Filiere> {
        Arc::clone(&self.filiere)
    }

    /*pub fn set_filiere(&mut self, filiere: Rc<Filiere>) {
        self.filiere = filiere;
    }*/

    /*pub fn get_filiere_mut(&mut self) -> &mut Filiere {
        &mut self.filiere
    }*/

    pub fn get_teacher_liste(&self) -> &Option<HashMap<usize,Teacher>> {
        &self.teachers
    }

    pub fn get_teacher_liste_mut(&mut self) -> &mut Option<HashMap<usize,Teacher>> {
        &mut self.teachers
    }

}





#[derive(Clone, Debug)]
pub struct  ClasseWindow {
    classe: HashMap<usize,Arc<Classe>>,
    filiere: HashMap<usize,Arc<Filiere>>,
    new_classe: String,
    new_filiere: String,
    new_nb_groupe: String,
    nb_groupe: Option<usize>,

    selected_option: String,
    selected_option_id: usize,

    selected_filiere_id: Option<usize>,
    editing_filiere_id: Option<usize>,
    supp_filiere_id: Option<usize>,

    selected_classe_id: Option<usize>,
    editing_classe_id: Option<usize>,
    supp_classe_id: Option<usize>,
    
    id_classe: usize,
    id_filiere:usize,

    window_position: egui::Pos2, // Coordonnées (x, y) pour afficher les fenêtres
    window_position2: egui::Pos2,
}

impl  Default for ClasseWindow  {
    fn default() -> Self {
        Self {
            classe:  HashMap::new(),
            filiere:  HashMap::new(),
            new_classe: String::new(),
            new_filiere: String::new(),
            new_nb_groupe: String::new(),
            nb_groupe: None,
            
            selected_option: String::new(),
            selected_option_id:0, 

            selected_filiere_id: None,
            editing_filiere_id: None,
            supp_filiere_id: None,

            selected_classe_id: None,
            editing_classe_id: None,
            supp_classe_id: None,
            
            id_classe: 0,
            id_filiere:0,

            window_position: egui::Pos2::new(0.0, 0.0), // Par exemple, x=200, y=100
            window_position2: egui::Pos2::new(380.0, 0.0),
        }
    }
}


impl  ClasseWindow {

    pub fn charge(&mut self, filieres: HashMap<usize,Arc<Filiere>>, classes: HashMap<usize,Arc<Classe>>) {
        self.filiere = filieres;
        self.classe = classes;
    }
    
    pub fn get_liste_filiere(&self) -> &HashMap<usize,Arc<Filiere>>{
        &self.filiere
    }

    pub fn get_liste_classe(&self) -> &HashMap<usize,Arc<Classe>>{
        &self.classe
    }
    pub fn build(&mut self, ctx: &egui::Context) {
        //si on a cliquer sur le bouton de suppression, delete la classe saisie
        
        self.id_filiere = *self.filiere.keys().max().unwrap_or(&0) + 1;
        self.id_classe = *self.classe.keys().max().unwrap_or(&0) + 1;

        if self.supp_classe_id != None {
            self.classe.remove(&self.supp_classe_id.unwrap());
            self.supp_classe_id = None;
        }
        
        let mut ids_to_remove = Vec::new();
        if self.supp_filiere_id != None {
            for (id, classe) in self.classe.iter_mut(){
                if Some(classe.get_filiere().get_id()) == self.supp_filiere_id {
                    ids_to_remove.push(*id);
                }
            }
            for id in ids_to_remove {
                self.classe.remove(&id);
            }

            
            self.filiere.remove(&self.supp_filiere_id.unwrap());
            self.supp_filiere_id = None;
            self.selected_filiere_id = None;
        }
        //selection du type de classe
        egui::Window::new("Création des filières")
            .current_pos(self.window_position)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    //ui.text_edit_singleline(&mut self.new_filiere);
                    let response = ui.text_edit_singleline(&mut self.new_filiere);

                    let ajout_filiere = (ui.button("Ajouter").clicked() || 
                    (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))) 
                    && !self.new_filiere.is_empty();
                    
                    
                    if ajout_filiere {
                        let mut doublon = false;
                        for (_id, filiere) in self.filiere.iter(){
                            if filiere.get_name() == self.new_filiere{
                                doublon = true;
                                //ajout d'un message qui indique que la type existe deja
                                break;
                            }
                        }
                        if !doublon {
                            //self.filiere.insert(self.id_filiere, Filiere::new(self.id_filiere, self.new_filiere.clone(),  None));
                            //let nouvelle_filiere = Filiere::new(self.id_filiere, self.new_filiere.clone(), None);
                            self.filiere.insert(self.id_filiere, Arc::new(Filiere::new(self.id_filiere, self.new_filiere.clone())));
                            //self.filiere.push(nouvelle_filiere);
                            self.new_filiere.clear();
                            self.id_filiere += 1;      
                        }
                        response.request_focus();
                    }
               
                });
                    for (id, filiere) in self.filiere.iter_mut() {
                        
                        ui.horizontal(|ui| {
                            if self.editing_filiere_id == Some(*id) {
                                let mut name_guard = filiere.name.lock().unwrap();
                                let mut new_name = name_guard.clone();

                                //let mut new_name = filiere.get_name();
                                let response =  ui.text_edit_singleline(&mut new_name);
                                if response.changed() {
                                    // Met à jour le nom en temps réel pendant que l'utilisateur tape
                                    //filiere.set_name(new_name.clone());
                                    *name_guard = new_name.clone();
                                }
                                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                    *name_guard = new_name.clone();
                                    //filiere.set_name(new_name.clone());
                                    //maj les classes saisies aHashMap le type de classe modifié
                                    /*for (_id, classe) in self.classe.iter_mut().enumerate(){
                                        if Some(classe.get_filiere().get_id()) == self.editing_filiere_id {
                                            //classe.get_filiere_mut().set_name(new_name.clone());
                                        }
                                    }*/
                                    self.editing_filiere_id = None;
                                }
                                drop(name_guard);
                            }

                        
                            if ui.selectable_label(
                                self.selected_filiere_id == Some(filiere.get_id()),
                                
                                filiere.get_name(),
                            )
                            .clicked()
                            {
                                self.selected_filiere_id = Some(filiere.get_id());
                            }
                            
                            if self.selected_filiere_id  == Some(filiere.get_id()){
                                if ui.button("✏").clicked() {
                                    self.editing_filiere_id =  self.selected_filiere_id
                                }
                                if ui.button("❌").clicked() {
                                    self.supp_filiere_id =  self.selected_filiere_id  
                                } 
                            }
                        });

                        //ui.label(filiere.get_name());
                    }
            
        });
        //saisie du numero de classe
        egui::Window::new("Création des classes")
            .current_pos(self.window_position2)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Filière :");
                    let _response_filiere = 
                        egui::ComboBox::from_id_source("Filière")
                            .selected_text(&self.selected_option)
                            .show_ui(ui, |ui| {
                                for  (id,option) in self.filiere.iter() {
                                        if ui.selectable_label(self.selected_option == option.get_name() /*&& !self.selected_option.is_empty()*/, option.get_name()).clicked() {
                                            self.selected_option = (option.get_name()).to_string();
                                            self.selected_option_id = *id;
                                        }
                                }
                            });
                    ui.label("Classe:");
                    let response_classe = ui.add(egui::TextEdit::singleline(&mut self.new_classe).desired_width(100.0));
                    ui.label("nombre de groupes:");

                    let response_nb_groupe = ui.add(egui::TextEdit::singleline(&mut self.new_nb_groupe).desired_width(25.0));

                    if response_nb_groupe.lost_focus()  {
                        match self.new_nb_groupe.parse::<usize>() {
                            Ok(nombre) => {
                                self.nb_groupe = Some(nombre);
                            },
                            //None => { },
                            Err(_) => {
                                self.nb_groupe = None;
                                self.new_nb_groupe.clear();
                            }
                        }
                    }

                    let ajout_classe = 
                        (ui.button("Ajouter").clicked() || 
                        (response_classe.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                        || response_nb_groupe.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
                        ) 
                    && !self.new_classe.is_empty();

                    if ajout_classe {
                       
                        if let Some(selected_filiere) = self.filiere.get(&self.selected_option_id) {
                            let mut doublon = false;
                            for (id,classe) in self.classe.iter() {
                                if classe.get_name() == self.new_classe && Arc::ptr_eq(&classe.get_filiere(), selected_filiere) {
                                    doublon = true;
                                    break;
                                }
                            }
                            if !doublon {
                                self.classe.insert(self.id_classe, Arc::new(Classe::new(self.id_classe,  Arc::clone(selected_filiere), self.new_classe.clone(), None, self.nb_groupe.unwrap_or(0))));
                                self.new_classe.clear();
                                self.id_classe += 1;
                            }
                            response_classe.request_focus();
                        }


                    }
                });
                
                //affichage sous forme de tableau de la liste des classes sous chaque type de classe
                //let cell_size = HashMap2::new(100.0, 30.0);
                ui.vertical(|ui| {
                    egui::ScrollArea::both() // Activer le défilement vertical et horizontal
                    .auto_shrink([false, true]) // Permet à la zone de se rétrécir horizontalement, mais de ne pas se rétrécir verticalement
                    .show(ui, |ui| {
                        egui::Grid::new("rooms_by_type")
                            .striped(true)
                            .show(ui, |ui| {
                                // En-têtes : un pour chaque type de classe
                                for (_id_type, filiere) in self.filiere.iter() { 
                                    ui.label(filiere.get_name());
                                }
                                ui.end_row();
                                // Données : afficher les classes sous chaque type
                                
                                //détermine le nombre de ligne du tableau
                                let max_salles_par_type = self.filiere
                                    .keys()
                                    .map(|id_type| self.classe.values().filter(|classe| classe.get_filiere().get_id() == *id_type).count())
                                    .max()
                                    .unwrap_or(0);

                                for i in 0..max_salles_par_type {
                                    for (id_type, filiere) in self.filiere.iter(){
                                        let classes: Vec<_> = self.classe
                                            .iter()
                                            .filter(|(id, classe)| classe.get_filiere().get_id() == *id_type)
                                            .map(|(id,classe)| classe.clone())
                                            .collect();

                                        if let Some(classe) = classes.get(i) {
                                            
                                            //ui.horizontal sert à faire en sorte que les boutons ne s'affiche pas sur les autre colonnes et que la taille de la colonne s'adapte
                                            ui.horizontal( |ui|{
                                                if ui.selectable_label(
                                                    self.selected_classe_id == Some(classe.get_id()),
                                                    
                                                    classe.get_name(),
                                                )
                                                .clicked()
                                                {
                                                    self.selected_classe_id = Some(classe.get_id());
                                                }
                                                
                                                if self.selected_classe_id  == Some(classe.get_id()){
                                                    /*if ui.button("Modifier").clicked() {
                                                        self.editing_room_id =  self.selected_classe_id
                                                    }*/
                                                    ui.label(format!("{:} groupe(s)", classe.get_nb_groupe().to_string()));
                                                    if ui.button("❌").clicked() {
                                                        self.supp_classe_id =  self.selected_classe_id  
                                                    } 
                                                }
                                            });
                                        } else {
                                            ui.label(""); // Cellule vide si pas de classes
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
