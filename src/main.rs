//use eframe::egui;
//use egui::{Context, Ui};
//use std::collections::HashMap;

use rusqlite::{params, Connection, Result};
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

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Générateur d'emploi du temps",
        native_options,
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
                        not_available BOOLEAN,
                        FOREIGN KEY (id_prof) REFERENCES Prof(id)
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
                        FOREIGN KEY (id_type_salle) REFERENCES TypeSalle(id)
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
                        FOREIGN KEY (id_filiere) REFERENCES Filiere(id)
                    )",
        ()
    )?;
    /*conn.execute("CREATE TABLE IF NOT EXISTS Programme
                    (
                        id INTEGER PRIMARY KEY,
                        nb_semaine INTEGER,
                        id_filiere INTEGER,
                        FOREIGN KEY (id_filiere) REFERENCES Filiere(id)
                    )",
        ()
    )?;*/

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
                        FOREIGN KEY (id_type_salle) REFERENCES TypeSalle(id)
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
                        groupe BOOLEAN,
                        nb_groupe INTEGER,
                        interclasse BOOLEAN,
                        FOREIGN KEY (id_semaine, id_filiere) REFERENCES Semaine(id_semaine, id_filiere),
                        FOREIGN KEY (id_matiere) REFERENCES Matiere(id)
                    )",
        ()
    )?;
    //une occurrence par classe par cours en interclasse
    conn.execute("CREATE TABLE IF NOT EXISTS MatiereInterClasse
                    (
                        id SERIAL PRIMARY KEY,
                        id_matiere_prog INTEGER,
                        id_classe INTEGER,
                        FOREIGN KEY (id_matiere_prog) REFERENCES MatiereInterClasse(id),
                        FOREIGN KEY (id_classe) REFERENCES Classe(id)
                    )",
        ()
    )?;
    
    conn.execute("CREATE TABLE IF NOT EXISTS Assignement
                    (
                        id INTEGER PRIMARY KEY,
                        id_classe INTEGER,
                        id_matiere INTEGER,
                        id_prof INTEGER,
                        FOREIGN KEY (id_classe) REFERENCES Classe(id),
                        FOREIGN KEY (id_matiere) REFERENCES Matiere(id),
                        FOREIGN KEY (id_prof) REFERENCES Prof(id)
                    )",
        ()
    )?;

    Ok(())
}