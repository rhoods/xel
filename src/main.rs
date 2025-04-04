//use eframe::egui;
//use egui::{Context, Ui};
//use std::collections::HashMap;

use rusqlite::{params, Connection, Result};
use eframe::egui::{self, Label, Response, pos2, vec2, Rect};

mod app;
mod struc;

use crate::app::windows::SchedulerApp;


//use app::SchedulerApp;

fn main() -> eframe::Result<()> {
    let ok = creation_table();
    match ok {
        Ok(_) => println!("creation de la base terminee")
        , Err(erreur) => println!("erreur lors de la base : {}", erreur)
    }
    
    //let native_options = eframe::NativeOptions::default();
    let options = eframe::NativeOptions {
        window_builder: Some(Box::new(|builder| {
            builder.with_min_inner_size(vec2(700.0, 500.0))
        })),
        // Autres options...
        ..Default::default()
    };
    eframe::run_native(
        "Générateur d'emploi du temps",
        options,
        Box::new(|_cc| Box::new(SchedulerApp::default())),
    )
}


fn creation_table() -> Result<()> {
    let conn = Connection::open("C:/Users/admin/source/repos/xel/bdd/bdd.db")?;

    conn.execute("CREATE TABLE IF NOT EXISTS Prof
                    (
                        id INTEGER PRIMARY KEY,
                        name TEXT
                    )",
        ()
    )?;
    conn.execute("CREATE TABLE IF NOT EXISTS Creneaux
                    (
                        id SERIAL PRIMARY KEY,
                        id_day INTEGER,
                        id_hour INTEGER,
                        id_prof INTEGER,
                        etat INTEGER,
                        FOREIGN KEY (id_prof) REFERENCES Prof(id) ON DELETE CASCADE
                    )",
        ()
    )?;
    conn.execute("CREATE TABLE IF NOT EXISTS TypeSalle
                    (
                        id INTEGER PRIMARY KEY,
                        name TEXT
                    )",
        ()
    )?;
    conn.execute("CREATE TABLE IF NOT EXISTS Salle
                    (
                        id INTEGER PRIMARY KEY,
                        name TEXT,
                        id_type_salle INTEGER,
                        FOREIGN KEY (id_type_salle) REFERENCES TypeSalle(id) ON DELETE CASCADE
                    )",
        ()
    )?;
    conn.execute("CREATE TABLE IF NOT EXISTS Filiere
                    (
                        id INTEGER PRIMARY KEY,
                        name TEXT,
                        nb_semaines INTEGER
                    )",
        ()
    )?;
    conn.execute("CREATE TABLE IF NOT EXISTS Classe
                    (
                        id INTEGER PRIMARY KEY,
                        name TEXT,
                        nb_groupe INTEGER,
                        id_filiere INTEGER,
                        FOREIGN KEY (id_filiere) REFERENCES Filiere(id) ON DELETE CASCADE
                    )",
        ()
    )?;
    conn.execute("CREATE TABLE IF NOT EXISTS Horaires
                    (
                        id_jour INTEGER,
                        id_heure INTEGER,
                        name_jour TEXT, 
                        name_heure TEXT,
                        type_creneau INTEGER
                    )",
        []
    )?;

    //PRIMARY KEY
    conn.execute("CREATE TABLE IF NOT EXISTS Semaine
                    (
                        id_semaine INTEGER ,
                        id_filiere INTEGER,
                        FOREIGN KEY (id_filiere) REFERENCES Filiere(id),
                        PRIMARY KEY (id_semaine, id_filiere)
                    )",
        ()
    )?;
    conn.execute("CREATE TABLE IF NOT EXISTS Matiere
                    (
                        id INTEGER PRIMARY KEY,
                        name TEXT,
                        id_type_salle INTEGER,
                        FOREIGN KEY (id_type_salle) REFERENCES TypeSalle(id) ON DELETE CASCADE
                    )",
        ()
    )?;

    conn.execute("CREATE TABLE IF NOT EXISTS option
                    (
                        id INTEGER PRIMARY KEY,
                        name TEXT,
                        id_filiere INTEGER,
                        FOREIGN KEY (id_filiere) REFERENCES Filiere(id) ON DELETE CASCADE
                    )",
        ()
    )?;

    conn.execute("CREATE TABLE IF NOT EXISTS MatiereProg
                    (
                        id INTEGER PRIMARY KEY,
                        id_semaine INTEGER,
                        id_filiere INTEGER,
                        id_matiere INTEGER,
                        nb_heure INTEGER,
                        duree_minimum INTEGER,
                        duree_maximum INTEGER,
                        groupe BOOLEAN,
                        nb_groupe INTEGER,
                        interclasse BOOLEAN,
                        id_option Integer,
                        FOREIGN KEY (id_semaine, id_filiere) REFERENCES Semaine(id_semaine, id_filiere) ON DELETE CASCADE,
                        FOREIGN KEY (id_matiere) REFERENCES Matiere(id) ON DELETE CASCADE,
                        FOREIGN KEY (id_option) REFERENCES Option(id) ON DELETE CASCADE
                    )",
        ()
    )?;
    //une occurrence par classe par cours en interclasse
    conn.execute("CREATE TABLE IF NOT EXISTS MatiereInterClasse
                    (
                        id INTEGER,
                        id_matiere INTEGER,
                        id_classe INTEGER,
                        id_classe_participante INTEGER,
                        FOREIGN KEY (id_matiere) REFERENCES Matiere(id) ON DELETE CASCADE,
                        FOREIGN KEY (id_classe) REFERENCES Classe(id) ON DELETE CASCADE,
                        FOREIGN KEY (id_classe_participante) REFERENCES Classe(id) ON DELETE CASCADE
                    )",
        ()
    )?;

    //une occurrence par classe par cours en interclasse
    conn.execute("CREATE TABLE IF NOT EXISTS Groupe
                    (
                        id PRIMARY KEY,
                        name INTEGER,
                        id_matiere INTEGER,
                        id_classe INTEGER,
                        FOREIGN KEY (id_matiere) REFERENCES Matiere(id) ON DELETE CASCADE,
                        FOREIGN KEY (id_classe) REFERENCES Classe(id) ON DELETE CASCADE
                    )",
        ()
    )?;
    
    conn.execute("CREATE TABLE IF NOT EXISTS Assignement
                    (
                        id INTEGER PRIMARY KEY,
                        id_classe INTEGER,
                        id_matiere INTEGER,
                        id_prof INTEGER,
                        id_groupe INTEGER,
                        id_option INTEGER,
                        id_matiere_prog INTEGER,
                        FOREIGN KEY (id_classe) REFERENCES Classe(id) ON DELETE CASCADE,
                        FOREIGN KEY (id_matiere) REFERENCES Matiere(id) ON DELETE CASCADE,
                        FOREIGN KEY (id_prof) REFERENCES Prof(id) ON DELETE CASCADE,
                        FOREIGN KEY (id_groupe) REFERENCES Groupe(id) ON DELETE CASCADE,
                        FOREIGN KEY (id_option) REFERENCES Option(id) ON DELETE CASCADE,
                        FOREIGN KEY (id_matiere_prog) REFERENCES MatiereProg(id) ON DELETE CASCADE
                    )",
        ()
    )?;

    Ok(())
}