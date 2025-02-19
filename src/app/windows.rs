use eframe::egui;
use egui::Context;
use std::fs::File;
use std::io::Write;
use rusqlite::{params, Connection, Result};

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::struc::teacher;
//use crate::struc::programme::MatiereProg;
use crate::struc::{teacher::Teacher,  matiere::Matiere, programme::{Semaine, MatiereProg, MatiereInterClasse}};

use crate::struc::assignation::{Assignation, Groupe};

use crate::app::teachers_window::TeacherWindow as TeacherWindow;
use crate::app::room_window::RoomWindow as RoomWindow;
use crate::app::room_window::Room;
use crate::app::room_window::RoomType;

//use crate::app::assignation_window::Assignation_window;

use crate::app::filiere_window::ClasseWindow as ClasseWindow;
use crate::app::filiere_window::Filiere;
use crate::app::filiere_window::Classe;

//use crate::app::filiere_window::ClasseWindow as ClasseWindow;
use crate::app::matiere_window::MatiereWindow as MatiereWindow;
use crate::app::programme_window::ProgrammeWindow as ProgrammeWindow;
use crate::app::assignation_window::AssignationWindow as AssignationWindow;
use crate::app::planning_windows::PlanningWindow as PlanningWindow;



#[derive(Clone, Debug)]
enum FenetreActive{
    Aucune,
    Horaires,
    Professeurs,
    Salles,
    Filiere,
    Matieres,
    Programmes,
    Affectations,
    Generation,
}


#[derive(Clone, Debug)]
pub struct SchedulerApp{
    // États des fenêtres
    
    validation_save: bool,
    validation_charge: bool,
    show_validation_save_window: bool,
    show_validation_charge_window: bool,

    fenetre_active: FenetreActive,
    show_teachers_window: bool,
    show_rooms_window: bool,
    show_filiere_window: bool,
    show_matiere_window: bool,
    show_programs_window: bool,
    show_classes_window: bool,
    show_assignments_window: bool,
    show_generation_window: bool,

    // Données
    assignement : HashMap<usize, Arc<Assignation>>,
    groupe: HashMap<usize, Arc<Groupe>>,
    teachers: HashMap<usize,Teacher>,
    rooms: HashMap<usize,Room>,
    rooms_types: HashMap<usize,Arc<RoomType>>,
    classes: HashMap<usize,Arc<Classe>>,
    matieres: HashMap<usize,Arc<Matiere>>,
    filieres: HashMap<usize,Arc<Filiere>>,
    //programmes: HashMap<usize,Arc<Programme>>,
    semaines: HashMap<(usize,usize),Arc<Semaine>>,
    matieres_prog: HashMap<usize,Arc<MatiereProg>>,
    matieres_inter_classe: HashMap<usize,Arc<MatiereInterClasse>>,

    // Données temporaires pour les nouvelles entrées
    new_teacher: String,
    new_room: String,
    new_matiere: String,
    new_filiere: String,
    
    selected_teacher_id: Option<usize>,
    editing_teacher_id: Option<usize>,
    supp_teacher_id: Option<usize>,

    id_teacher: usize, 
    id_room: usize,
    id_planning_teacher: usize,
    id_classe: usize,
    id_filiere: usize,
    id_programme: usize,
    id_matiere: usize,
    window_position: egui::Pos2, // Coordonnées (x, y) pour afficher les fenêtres
    teacher_window: TeacherWindow,
    room_window: RoomWindow,
    classe_window: ClasseWindow,
    matiere_window: MatiereWindow,
    programme_window: ProgrammeWindow,
    assignation_window: AssignationWindow,
    planning_window: PlanningWindow,
}

impl  Default for SchedulerApp{
    fn default() -> Self {
        Self {
         
            validation_save: false,
            validation_charge: false,
            show_validation_save_window: false,
            show_validation_charge_window: false,
            fenetre_active: FenetreActive::Aucune,
            show_teachers_window: false,
            show_rooms_window: false,
            show_filiere_window: false,
            show_matiere_window: false,
            show_programs_window: false,
            show_classes_window: false,
            show_assignments_window: false,
            show_generation_window: false,

            assignement : HashMap::new(),
            groupe: HashMap::new(),
            teachers: HashMap::new(),
            rooms: HashMap::new(),
            rooms_types: HashMap::new(),
            matieres: HashMap::new(),
            filieres: HashMap::new(),
            classes: HashMap::new(),
            //programmes: HashMap::new(),
            semaines: HashMap::new(),
            matieres_prog: HashMap::new(),
            matieres_inter_classe:HashMap::new(),

            new_teacher: String::new(),
            new_room: String::new(),
            new_matiere:String::new(),
            new_filiere: String::new(),

            selected_teacher_id: None,
            editing_teacher_id: None,
            supp_teacher_id: None,

            id_teacher: 1, 
            id_room: 1,
            id_planning_teacher: 1,
            id_classe: 1,
            id_filiere: 1,
            id_programme: 1,
            id_matiere: 1,
            window_position: egui::Pos2::new(0.0, 20.0), // Par exemple, x=200, y=100
            teacher_window: TeacherWindow::default(),
            room_window: RoomWindow::default(),
            classe_window: ClasseWindow::default(),
            matiere_window: MatiereWindow::default(),
            programme_window: ProgrammeWindow::default(),
            assignation_window: AssignationWindow::default(),
            planning_window: PlanningWindow::default(),
            
            
        }
    }
}

impl  eframe::App for SchedulerApp {
    fn update(&mut self,ctx: &Context, _frame: &mut eframe::Frame)
    {
        egui::TopBottomPanel::top("top_panel")
        .show(ctx, |ui| {
            egui::menu::bar(ui,   |ui| {
                ui.menu_button("Fichier",   |ui| {
                    if ui.button("Sauvegarder").clicked() {
                        self.show_validation_save_window = true;
                    }
                    if ui.button("Charger").clicked() {
                        self.show_validation_charge_window = true; 
                    }
                    
                });
            });
        });    

        egui::TopBottomPanel::top("onglets")
        .show(ctx, |ui| {
            egui::menu::bar(ui,   |ui| {
                ui.menu_button("Horaires",   |_ui| {
                    self.fenetre_active = FenetreActive::Horaires;
                });
                ui.menu_button("Professeurs",   |_ui| {
                    self.fenetre_active = FenetreActive::Professeurs;
                });
                ui.menu_button("Salles",   |_ui| {
                    self.fenetre_active = FenetreActive::Salles;
                });
                ui.menu_button("Filières",   |_ui| {
                    self.fenetre_active = FenetreActive::Filiere;
                });
                
                ui.menu_button("Matières",   |_ui| {
                    self.fenetre_active = FenetreActive::Matieres;
                });
                ui.menu_button("Programmes",   |_ui| {
                    self.fenetre_active = FenetreActive::Programmes;
                });
                ui.menu_button("Affectations",   |_ui| {
                    self.fenetre_active = FenetreActive::Affectations;
                });
                ui.menu_button("Generation",   |_ui| {
                    self.fenetre_active = FenetreActive::Generation;
                });
            });
        });

        match self.fenetre_active {
            FenetreActive::Aucune => {}
            FenetreActive::Horaires => {
                self.show_horaires_window(ctx);  
            }
            FenetreActive::Professeurs => {
                self.show_teachers_window(ctx);  
            }
            FenetreActive::Salles => {
                self.show_rooms_window(ctx);
            }
            FenetreActive::Filiere => {
                self.show_filiere_window(ctx);
            }
            FenetreActive::Matieres => {
                self.show_matiere_window(ctx);
            }
            FenetreActive::Programmes => {
                self.show_programs_window(ctx);
            }
            FenetreActive::Affectations => {
                self.show_assignments_window(ctx);
            }
            FenetreActive::Generation => {
                self.show_generations_window(ctx);  
            }
        }

            // SAUVEGARDE
            if self.show_validation_save_window {
                egui::Window::new("Validation")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Attention, la sauvegarde précédente sera supprimée");  
                        ui.horizontal(|(ui)| {
                            if ui.button("Valider").clicked(){
                                let reset = self.reset_bdd();
                                match reset {
                                    Ok(_) => println!("reset bdd termine")
                                    , Err(erreur) => println!("erreur lors de la sauvegarde : {}", erreur)
                                    }
                                

                                let ok = self.sauvegarder();
                                match ok {
                                Ok(_) => println!("sauvegarde terminee")
                                , Err(erreur) => println!("erreur lors de la sauvegarde : {}", erreur)
                                }
                                self.show_validation_save_window = false;
                            }
                            if ui.button("Annuler").clicked(){
                                self.show_validation_save_window = false;
                                
                            }
                            
                        });
                    });
                }

            //CHARGEMENT
            if self.show_validation_charge_window {
                egui::Window::new("Validation")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Attention, la saisie en cours sera supprimée");  
                    ui.horizontal(|ui| {
                            if ui.button("Valider").clicked(){
                                self.reset_saisie();
                                let ok = self.charger();
                                match ok {
                                Ok(_) => println!("chargement termine")
                                , Err(erreur) => println!("erreur lors du chargement : {}", erreur)
                                }
                                self.show_validation_charge_window = false;
                            }
                            if ui.button("Annuler").clicked(){
                                self.show_validation_charge_window = false;
                            }
                    });
                });
   
            }
            
        }
    }


impl  SchedulerApp {    

    fn show_horaires_window(&mut self, ctx: &Context) {
        //self.teacher_window.charge(self.teachers.clone());
        //self.teacher_window.build(ctx);
        //self.teachers = self.teacher_window.get_liste_teacher().clone();       
    }

    fn show_teachers_window(&mut self, ctx: &Context) {
        self.teacher_window.charge(self.teachers.clone());
        self.teacher_window.build(ctx);
        self.teachers = self.teacher_window.get_liste_teacher().clone();       
    }

    fn show_rooms_window(&mut self, ctx: &Context) {
            self.room_window.charge(self.rooms_types.clone(), self.rooms.clone(),);
            self.room_window.build(ctx);
            self.rooms_types = self.room_window.get_liste_type_salle().clone();
            self.rooms = self.room_window.get_liste_salle().clone();
    }

    fn show_matiere_window(&mut self, ctx: &Context) 
    {
            self.matiere_window.charge(self.matieres.clone(), self.rooms_types.clone());
            self.matiere_window.build(ctx);
            self.matieres = self.matiere_window.get_liste_matiere().clone();
    }

    fn show_filiere_window(&mut self, ctx: &Context) 
    {
            self.classe_window.charge(self.filieres.clone(), self.classes.clone());
            self.classe_window.build(ctx);
            self.filieres = self.classe_window.get_liste_filiere().clone();
            self.classes = self.classe_window.get_liste_classe().clone();
    }
    fn show_programs_window(&mut self, ctx: &Context) {
            self.programme_window.charge(self.groupe.clone(), self.semaines.clone(), self.matieres_prog.clone(), self.filieres.clone(), self.classes.clone(), self.matieres.clone(), self.rooms_types.clone());
            self.programme_window.build(ctx);
            self.semaines = self.programme_window.get_liste_semaine().clone();
            self.matieres_prog = self.programme_window.get_liste_matiere_prog().clone();
            self.groupe = self.programme_window.get_groupe().clone();
    }

    fn show_assignments_window(&mut self, ctx: &Context) {
        self.assignation_window.charge(self.semaines.clone(),self.classes.clone(), self.filieres.clone(),self.matieres.clone(),self.matieres_prog.clone(),   self.matieres_inter_classe.clone(), self.teachers.clone(), self.groupe.clone(), self.assignement.clone());
        self.assignation_window.build(ctx);
        self.groupe = self.assignation_window.get_groupe().clone();
        self.assignement = self.assignation_window.get_assignement().clone();

    } //AssignationWindow

    fn show_generations_window(&mut self, ctx: &Context){
        self.planning_window.charge(self.rooms.clone(), self.semaines.clone(),self.classes.clone(), self.filieres.clone(),self.matieres.clone(),self.matieres_prog.clone(),   self.matieres_inter_classe.clone(), self.teachers.clone(), self.groupe.clone(), self.assignement.clone());
        self.planning_window.build(ctx);
    }

    fn sauvegarder(&self) -> Result<()> {
        //let mut fichier = File::create("teachers.txt")?; // Crée ou remplace le fichier
        let conn = Connection::open("C:/Users/admin/source/repos/xel/bdd/bdd.db")?;

        //sauvegarde prof
        let mut insert_prof = conn.prepare("INSERT INTO Prof (id, name) VALUES (?1, ?2)")?;
        let mut insert_creneaux = conn.prepare("INSERT INTO Creneaux (id_day, id_hour, id_prof, not_available) VALUES (?1, ?2, ?3, ?4)")?;
        for (cle, teacher) in self.teachers.iter() {
            insert_prof.execute(rusqlite::params![cle, teacher.get_name()])?;
            for (id_creneau, not_available) in teacher.get_not_available_liste() {
                insert_creneaux.execute(rusqlite::params![id_creneau.0,id_creneau.1 , cle, not_available.get_available()])?;
            }
        }

        //sauvegarde roomtype
        let mut insert_type_salle = conn.prepare("INSERT INTO TypeSalle (id, name) VALUES (?1, ?2)")?;
        for (cle, type_salle) in self.rooms_types.iter() {
            insert_type_salle.execute(rusqlite::params![cle, type_salle.get_name()])?;
        }

        //sauvegarde salles
        let mut insert_salle = conn.prepare("INSERT INTO Salle (id, name, id_type_salle) VALUES (?1, ?2, ?3)")?;
        for (cle, salle) in self.rooms.iter() {
            insert_salle.execute(rusqlite::params![cle, salle.get_name(), salle.get_room_type().get_id()])?;
        }

        //sauvegarde filiere
        let mut insert_filiere = conn.prepare("INSERT INTO Filiere (id, name, nb_semaines) VALUES (?1, ?2, ?3)")?;
        for (cle, filiere) in self.filieres.iter() {
            insert_filiere.execute(rusqlite::params![cle, filiere.get_name(), filiere.get_nb_semaine()])?;
        }

        
        //sauvegarde classe
        let mut insert_classe = conn.prepare("INSERT INTO Classe (id, name, nb_groupe, id_filiere) VALUES (?1, ?2, ?3, ?4)")?;
        for (cle, classe) in self.classes.iter() {
            insert_classe.execute(rusqlite::params![cle, classe.get_name(), classe.get_nb_groupe(), classe.get_filiere().get_id()])?;
        }
        
        //sauvegarde classe
        let mut insert_matiere = conn.prepare("INSERT INTO Matiere (id, name, id_type_salle) VALUES (?1, ?2, ?3)")?;
        for (cle, matiere) in self.matieres.iter() {
            insert_matiere.execute(rusqlite::params![cle, matiere.get_name(), matiere.get_room_type().get_id()])?;
        }

        //sauvegarde programme
        //let mut insert_programme = conn.prepare("INSERT INTO Programme (id, nb_semaine, id_filiere) VALUES (?1, ?2, ?3)")?;
        let mut insert_semaine = conn.prepare("INSERT INTO Semaine (id_semaine, id_filiere) VALUES (?1, ?2)")?;
        let mut insert_matiere_prog = conn.prepare("INSERT INTO MatiereProg (id, id_semaine, id_filiere, id_matiere, nb_heure, groupe, nb_groupe, interclasse ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?;
        let mut insert_en_groupe_inter_classe = conn.prepare("INSERT INTO MatiereInterClasse (id, id_matiere_prog, id_classe) VALUES (?1, ?2, ?3)")?;
        let mut insert_groupe = conn.prepare("INSERT INTO Groupe (id, id_matiere,id_classe) values (?1, ?2, ?3)")?;
        let mut insert_assignement = conn.prepare("INSERT INTO Assignement (id, id_classe, id_matiere, id_prof, id_groupe) values (?1, ?2, ?3, ?4, ?5)")?;
        /*for (cle, programme) in self.programmes.iter() {
            insert_programme.execute(rusqlite::params![cle, programme.get_nb_semaine(), programme.get_filiere().get_id()])?;
        }*/
        for (cle_semaine, semaine) in self.semaines.iter(){
            insert_semaine.execute(rusqlite::params![cle_semaine.1, cle_semaine.0])?; //, semaine.get_filiere().get_id()
        }

        for(cle_matiere_prog, matiere_prog) in self.matieres_prog.iter() {
            insert_matiere_prog.execute(rusqlite::params![cle_matiere_prog, matiere_prog.get_semaine().get_id(),matiere_prog.get_semaine().get_filiere().get_id() , matiere_prog.get_matiere().get_id(), matiere_prog.get_nb_heure(),matiere_prog.get_en_groupe(),matiere_prog.get_nb_groupe(), matiere_prog.get_en_groupe_inter_classe()])?;
        }

        for (cle_en_groupe, matiere_inter) in self.matieres_inter_classe.iter(){
            insert_en_groupe_inter_classe.execute(rusqlite::params![cle_en_groupe, matiere_inter.get_matiere_prog().get_id(), matiere_inter.get_classe().get_id()])?;
        }

        //dbg!(&self.groupe);
        for (cle_groupe, groupe) in self.groupe.iter(){
            insert_groupe.execute(rusqlite::params![cle_groupe, groupe.get_matiere().get_id(), groupe.get_classe().get_id()])?;
        }

        for (cle_assignement, assignement) in self.assignement.iter(){
            insert_assignement.execute(rusqlite::params![cle_assignement, assignement.get_classe().get_id(), assignement.get_matiere().get_id(), assignement.get_prof().get_id(), assignement.get_groupe().get_id()])?;
        }

        Ok(())

    }


    fn charger(&mut self) -> Result<()> {
        //let mut fichier = File::create("teachers.txt")?; // Crée ou remplace le fichier
        let conn = Connection::open("C:/Users/admin/source/repos/xel/bdd/bdd.db")?;

        let mut recup_prof = conn.prepare("SELECT Prof.id, Prof.name FROM Prof")?;
        let mut recup_creneaux_prof = conn.prepare("SELECT id_prof, id_day, id_hour, not_available FROM Creneaux")?;
        let mut recup_room_type = conn.prepare("SELECT id, name FROM TypeSalle")?;
        let mut recup_room = conn.prepare("SELECT id, name, id_type_salle FROM Salle")?;
        let mut recup_filiere = conn.prepare("SELECT id, name, nb_semaines FROM Filiere")?;
        let mut recup_classe = conn.prepare("SELECT id, name, nb_groupe, id_filiere FROM Classe")?;
        let mut recup_matiere = conn.prepare("SELECT id, name, id_type_salle FROM Matiere")?;
        
        //prof
        let rows = recup_prof.query_map([], |row| {
                    let id_prof: usize = row.get(0)?;
                    let name_prof: String = row.get(1)?;
                    Ok((id_prof, name_prof.clone()))
                })?;
        
        for row in rows {
            let (id_prof, name_prof) = row?;
            self.teachers.insert(id_prof, Teacher::new(id_prof, name_prof.clone()));
        }
       
        //dispo prof
        let rows = recup_creneaux_prof.query_map([], |row| {
            let id_prof: usize = row.get(0)?;
            let id_day: usize = row.get(1)?;
            let id_hour: usize = row.get(2)?;
            let not_available: bool = row.get(3)?;
            Ok((id_prof, id_day, id_hour, not_available))
        })?;

        for row in rows {
            let (id_prof, id_day, id_hour, not_available) = row?;
            match self.teachers.get_mut(&id_prof) {
                Some(teacher) => teacher.charge_creneau(id_day, id_hour, not_available),
                None => println!("Aucune valeur"),
            }
        }


        
        //type salle
        let rows = recup_room_type.query_map([], |row| {
            let id_type_salle: usize = row.get(0)?;
            let name_type_salle: String = row.get(1)?;
            Ok((id_type_salle, name_type_salle.clone()))
        })?;

        for row in rows {
            let (id_type_salle, name_type_salle) = row?;
            self.rooms_types.insert(id_type_salle, Arc::new(RoomType::new(id_type_salle, name_type_salle.clone())));        
        }

        //salles
        let rows = recup_room.query_map([], |row| {
            let id_salle: usize = row.get(0)?;
            let name_salle: String = row.get(1)?;
            let id_type_salle: usize = row.get(2)?;
            Ok((id_salle, name_salle.clone(), id_type_salle))
        })?;

        for row in rows {
            let (id_salle, name_salle, id_type_salle) = row?;
            match self.rooms_types.get_key_value(&id_type_salle) {
                Some(room_type) => self.rooms.insert(id_salle, Room::new(id_salle, name_salle.clone(), Arc::clone(room_type.1))), //Some(room_type) ,
                None => None,
            };     
        }

        //filiere
        let rows = recup_filiere.query_map([], |row| {
            let id_filiere: usize = row.get(0)?;
            let name_filiere: String = row.get(1)?;
            let nb_semaines: Option<usize> = row.get(2)?;
            Ok((id_filiere, name_filiere.clone(), nb_semaines))
        })?;

        for row in rows {
            let (id_filiere, name_filiere, nb_semaines) = row?;
            self.filieres.insert(id_filiere, Arc::new(Filiere::charge(id_filiere, name_filiere.clone(), nb_semaines.unwrap_or(0))));        
        }

        //classe
        let rows = recup_classe.query_map([], |row| {
            let id_classe: usize = row.get(0)?;
            let name_classe: String = row.get(1)?;
            let nb_groupe: usize = row.get(2)?;
            let id_filiere: usize = row.get(3)?;
            Ok((id_classe, name_classe.clone(), nb_groupe, id_filiere))
        })?;

        for row in rows {
            let (id_classe, name_classe, nb_groupe, id_filiere) = row?;
            //let room_type = self.rooms_types.get_key_value(&id_type_salle);
            match self.filieres.get_key_value(&id_filiere) {
                Some(filiere) => self.classes.insert(id_classe, Arc::new(Classe::new(id_classe,Arc::clone(filiere.1), name_classe.clone(), None, nb_groupe ))), //Some(room_type) ,
                None => None,
            };     
        }

        //matiere
        let rows = recup_matiere.query_map([], |row| {
            let id_matiere: usize = row.get(0)?;
            let name_matiere: String = row.get(1)?;
            let id_type_salle: usize = row.get(2)?;
            Ok((id_matiere, name_matiere.clone(), id_type_salle))
        })?;

        for row in rows {
            let (id_matiere, name_matiere, id_type_salle) = row?;
            //let room_type = self.rooms_types.get_key_value(&id_type_salle);
            match self.rooms_types.get_key_value(&id_type_salle) {
                Some(type_salle) => self.matieres.insert(id_matiere, Arc::new(Matiere::new(id_matiere, name_matiere.clone(), Arc::clone(type_salle.1)))),
                None => None,
            };
        }

        let mut recup_semaine = conn.prepare("SELECT id_semaine, id_filiere FROM Semaine")?;
        let rows = recup_semaine.query_map([], |row| {
            let id_semaine: usize = row.get(0)?;
            let id_filiere: usize = row.get(1)?;
            Ok((id_semaine, id_filiere))
        })?;

        for row in rows {
            let (id_semaine, id_filiere) = row?;
            match self.filieres.get_key_value(&id_filiere) {
                Some(filiere) => self.semaines.insert((id_filiere,id_semaine), Arc::new(Semaine::new(id_semaine,Arc::clone(filiere.1)))),
                None => None,
            };
        }


        let mut recup_matiere_prog = conn.prepare("SELECT id, id_semaine,id_filiere, id_matiere, nb_heure, groupe,nb_groupe, interclasse FROM MatiereProg")?;
        let rows = recup_matiere_prog.query_map([], |row| {
            let id_matiere_prog: usize = row.get(0)?;
            let id_semaine: usize = row.get(1)?;
            let id_filiere: usize = row.get(2)?;
            let id_matiere: usize = row.get(3)?;
            let nb_heure: usize = row.get(4)?;
            let groupe: bool = row.get(5)?;
            let nb_groupe: usize = row.get(6)?;
            let interclasse: bool = row.get(7)?;
            Ok((id_matiere_prog, id_semaine, id_filiere, id_matiere, nb_heure, groupe, nb_groupe, interclasse))
        })?;

        for row in rows {
            let (id_matiere_prog, id_semaine, id_filiere, id_matiere, nb_heure, groupe, nb_groupe, interclasse) = row?;
            match (self.semaines.get_key_value(&(id_filiere,id_semaine)), self.matieres.get_key_value(&id_matiere) ) {
                (Some(semaine), Some(matiere)) => self.matieres_prog.insert(id_matiere_prog, Arc::new(MatiereProg::new(id_semaine,Arc::clone(matiere.1), nb_heure, groupe,  nb_groupe, interclasse, Arc::clone(semaine.1))), ),
                _ => None,
            };
        }

        let mut recup_matiere_inter_classe = conn.prepare("SELECT id, id_matiere_prog, id_classe FROM MatiereInterClasse")?;
        let rows = recup_matiere_inter_classe.query_map([], |row| {
            let id_matiere_inter_classe: usize = row.get(0)?;
            let id_matiere_prog: usize = row.get(1)?;
            let id_classe: usize = row.get(2)?;
            Ok((id_matiere_inter_classe, id_matiere_prog, id_classe))
        })?;

        for row in rows {
            let (id_matiere_inter_classe, id_matiere_prog, id_classe) = row?;
            match (self.matieres_prog.get_key_value(&id_matiere_prog), self.classes.get_key_value(&id_classe) ) {
                (Some(matiere_prog), Some(classe)) => self.matieres_inter_classe.insert(id_matiere_inter_classe, Arc::new(MatiereInterClasse::new(id_matiere_inter_classe,Arc::clone(matiere_prog.1), Arc::clone(classe.1))), ),
                _ => None,
            };
        }



        let mut recup_groupe = conn.prepare("SELECT id, id_matiere, id_classe FROM Groupe")?;
        let rows = recup_groupe.query_map([], |row| {
            let id_groupe: usize = row.get(0)?;
            let id_matiere: usize = row.get(1)?;
            let id_classe: usize = row.get(2)?;
            Ok((id_groupe, id_matiere, id_classe))
        })?;

        for row in rows {
            let (id_groupe, id_matiere, id_classe) = row?;
            match (self.matieres.get_key_value(&id_matiere), self.classes.get_key_value(&id_classe) ) {
                (Some(matiere), Some(classe)) => self.groupe.insert(id_groupe, Arc::new(Groupe::new(id_groupe, Arc::clone(classe.1), Arc::clone(matiere.1)))),
                _ => None,
            };
        }

        let mut recup_assignement = conn.prepare("SELECT id,  id_classe, id_matiere, id_prof, id_groupe FROM Assignement")?;
        let rows = recup_assignement.query_map([], |row| {
            let id_assignement: usize = row.get(0)?;
            let id_classe: usize = row.get(1)?;
            let id_matiere: usize = row.get(2)?;
            let id_prof: usize = row.get(3)?;
            let id_groupe: usize = row.get(4)?;
            Ok((id_assignement, id_classe, id_matiere, id_prof, id_groupe))
        })?;

        for row in rows {
            let (id_assignement, id_classe, id_matiere, id_prof, id_groupe) = row?;
            match (self.matieres.get_key_value(&id_matiere), self.classes.get_key_value(&id_classe), self.teachers.get_key_value(&id_prof), self.groupe.get_key_value(&id_groupe)) {
                (Some(matiere), Some(classe), Some(prof), Some(groupe)) => self.assignement.insert(id_assignement, Arc::new(Assignation::new(id_assignement, Arc::clone(classe.1), Arc::clone(matiere.1), Arc::clone(groupe.1) , prof.1.clone()))),
                _ => None,
            };
        }

        Ok(())

    }

    pub fn reset_saisie(&mut self) {
        self.validation_save = false;
        self.validation_charge= false;
        self.show_validation_save_window= false;
        self.show_validation_charge_window= false;
        self.fenetre_active= FenetreActive::Aucune;
        self.show_teachers_window= false;
        self.show_rooms_window= false;
        self.show_filiere_window= false;
        self.show_matiere_window= false;
        self.show_programs_window= false;
        self.show_classes_window= false;
        self.show_assignments_window= false;

        self.teachers= HashMap::new();
        self.rooms= HashMap::new();
        self.rooms_types= HashMap::new();
        self.matieres= HashMap::new();
        self.filieres= HashMap::new();
        self.classes= HashMap::new();
        
        self.semaines= HashMap::new();
        self.matieres_prog= HashMap::new();
        self.matieres_inter_classe=HashMap::new();

        self.new_teacher= String::new();
        self.new_room= String::new();
        self.new_matiere=String::new();
        self.new_filiere= String::new();

        self.selected_teacher_id= None;
        self.editing_teacher_id= None;
        self.supp_teacher_id= None;

        self.id_teacher= 1; 
        self.id_room= 1;
        self.id_planning_teacher= 1;
        self.id_classe= 1;
        self.id_filiere= 1;
        self.id_programme= 1;
        self.id_matiere= 1;
       // self.window_position = egui::Pos2::new(0.0; 20.0); // Par exemple; x=200; y=100
        self.teacher_window= TeacherWindow::default();
        self.room_window= RoomWindow::default();
        self.classe_window= ClasseWindow::default();
        self.matiere_window= MatiereWindow::default();
        self.programme_window= ProgrammeWindow::default();

    }


    pub fn reset_bdd(&self) -> Result<()> {

        let conn = Connection::open("C:/Users/admin/source/repos/xel/bdd/bdd.db")?;

        let mut reset_assignement = conn.prepare("DELETE FROM Assignement")?;
        let mut reset_groupe = conn.prepare("DELETE FROM Groupe")?;
        let mut reset_creneaux = conn.prepare("DELETE FROM Creneaux")?;
        let mut reset_prof = conn.prepare("DELETE FROM Prof")?;
        let mut reset_matiere_inter = conn.prepare("DELETE FROM MatiereInterClasse")?;
        let mut reset_matiere_prog = conn.prepare("DELETE FROM MatiereProg")?;
        let mut reset_semaine = conn.prepare("DELETE FROM Semaine")?;
        let mut reset_matiere = conn.prepare("DELETE FROM Matiere")?;
        let mut reset_salle = conn.prepare("DELETE FROM Salle")?;
        let mut reset_classe = conn.prepare("DELETE FROM Classe")?;
        let mut reset_filiere = conn.prepare("DELETE FROM Filiere")?;
        let mut reset_type_salle = conn.prepare("DELETE FROM TypeSalle")?;

        reset_assignement.execute(())?;
        conn.execute("VACUUM;", [])?;
        reset_creneaux.execute(())?;
        conn.execute("VACUUM;", [])?;
        reset_prof.execute(())?;
        conn.execute("VACUUM;", [])?;
        reset_matiere_inter.execute(())?;
        conn.execute("VACUUM;", [])?;
        reset_matiere_prog.execute(())?;
        conn.execute("VACUUM;", [])?;
        reset_semaine.execute(())?;
        conn.execute("VACUUM;", [])?;
        reset_matiere.execute(())?;
        conn.execute("VACUUM;", [])?;
        reset_salle.execute(())?;
        conn.execute("VACUUM;", [])?;
        reset_classe.execute(())?;
        conn.execute("VACUUM;", [])?;
        reset_filiere.execute(())?;
        conn.execute("VACUUM;", [])?;
        reset_type_salle.execute(())?;
        conn.execute("VACUUM;", [])?;
        reset_groupe.execute(())?;
        conn.execute("VACUUM;", [])?;

        Ok(())
    }



    fn clone(&self) -> Self {
        Self {
   
            validation_save: self.validation_save,
            validation_charge: self.validation_charge,
            show_validation_save_window: self.show_validation_save_window,
            show_validation_charge_window: self.show_validation_charge_window,
            fenetre_active: self.fenetre_active.clone(),
            show_teachers_window: self.show_teachers_window,
            show_rooms_window: self.show_rooms_window,
            show_filiere_window: self.show_filiere_window,
            show_matiere_window: self.show_matiere_window,
            show_programs_window: self.show_programs_window,
            show_classes_window: self.show_classes_window,
            show_assignments_window: self.show_assignments_window,
            show_generation_window: self.show_generation_window,
            
            assignement : self.assignement.clone(),
            groupe: self.groupe.clone(),
            teachers: self.teachers.clone(),
            rooms: self.rooms.clone(),
            rooms_types: self.rooms_types.clone(),
            classes: self.classes.clone(),
            matieres: self.matieres.clone(),
            filieres: self.filieres.clone(),
            //programmes: self.programmes.clone(),
            semaines: self.semaines.clone(),
            matieres_prog: self.matieres_prog.clone(),
            matieres_inter_classe: self.matieres_inter_classe.clone(),
            
            new_teacher: self.new_teacher.clone(),
            new_room: self.new_room.clone(),
            new_matiere: self.new_matiere.clone(),
            new_filiere: self.new_filiere.clone(),
            
            selected_teacher_id: self.selected_teacher_id,
            editing_teacher_id: self.editing_teacher_id,
            supp_teacher_id: self.supp_teacher_id,
            
            id_teacher: self.id_teacher,
            id_room: self.id_room,
            id_planning_teacher: self.id_planning_teacher,
            id_classe: self.id_classe,
            id_filiere: self.id_filiere,
            id_programme: self.id_programme,
            id_matiere: self.id_matiere,
            
            window_position: self.window_position,
            teacher_window: self.teacher_window.clone(),
            room_window: self.room_window.clone(),
            classe_window: self.classe_window.clone(),
            matiere_window: self.matiere_window.clone(),
            programme_window: self.programme_window.clone(),
            assignation_window: self.assignation_window.clone(),
            planning_window: self.planning_window.clone(),
        }
    }
}